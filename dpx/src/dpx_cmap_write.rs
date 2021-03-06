/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

use crate::warn;
use std::io::Write;

use super::dpx_cid::{CSI_IDENTITY, CSI_UNICODE};
use crate::dpx_pdfobj::{pdf_dict, pdf_stream, pdf_string, STREAM_COMPRESS};
use libc::memcmp;

use super::dpx_cmap::mapDef;
use super::dpx_cmap::CMap;

/*
 * References:
 *
 *  PostScript Language Reference Manual, 3rd. ed. (Adobe Systems Inc.)
 *    5.11.4 CMap Dictionaries
 *    5.11.5 FMapType 9 Composite Fonts
 *  Building CMap Files for CID-Keyed Fonts, Adobe Technical Note #5099
 *  CID-Keyed Font Technology Overview, Adobe Technical Note #5092
 *  Adobe CMap and CIDFont Files Specification, Adobe Technical Specification #5014
 *
 *  Undefined Character Handling:
 *    PLRM 3rd. ed., sec. 5.11.5., "Handling Undefined Characters"
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct C2RustUnnamed_1 {
    pub(crate) start: i32,
    pub(crate) count: i32,
}
unsafe fn block_count(mtab: &[mapDef], mut c: usize) -> usize {
    let mut count = 0;
    let n = mtab[c].len - 1;
    c += 1;
    while c < 256 {
        if mtab[c].flag & 1 << 4 != 0
            || (if mtab[c].flag & 0xf != 0 { 1 } else { 0 }) == 0
            || mtab[c].flag & 0xf != 1 << 0 && mtab[c].flag & 0xf != 1 << 2
            || mtab[c - 1].len != mtab[c].len
        {
            break;
        }
        if !(memcmp(
            mtab[c - 1].code as *const libc::c_void,
            mtab[c].code as *const libc::c_void,
            n as _,
        ) == 0
            && (*mtab[c - 1].code.offset(n as isize) as i32) < 255
            && *mtab[c - 1].code.offset(n as isize) as i32 + 1
                == *mtab[c].code.offset(n as isize) as i32)
        {
            break;
        }
        count += 1;
        c += 1
    }
    count
}
unsafe fn sputx(c: u8, s: &mut Vec<u8>, lim: usize) {
    let hi: i8 = (c as i32 >> 4) as i8;
    let lo: i8 = (c as i32 & 0xf) as i8;
    if s.len() > lim - 2 {
        panic!("Buffer overflow.");
    }
    s.push(
        (if (hi as i32) < 10 {
            hi as i32 + '0' as i32
        } else {
            hi as i32 + '7' as i32
        }) as u8,
    );
    s.push(
        (if (lo as i32) < 10 {
            lo as i32 + '0' as i32
        } else {
            lo as i32 + '7' as i32
        }) as u8,
    );
}
unsafe fn write_map(
    mtab: &mut [mapDef],
    mut count: usize,
    codestr: &mut [u8],
    depth: usize,
    wbuf: &mut Vec<u8>,
    lim: usize,
    stream: &mut pdf_stream,
) -> i32 {
    /* Must be greater than 1 */
    let mut blocks: [C2RustUnnamed_1; 129] = [C2RustUnnamed_1 { start: 0, count: 0 }; 129];
    let mut num_blocks = 0;
    let mut c = 0_usize;
    while c < 256 {
        codestr[depth as usize] = (c & 0xff) as u8;
        if mtab[c].flag & 1 << 4 != 0 {
            let mtab1 = mtab[c].next.as_mut().unwrap();
            count = write_map(mtab1, count, codestr, depth + 1, wbuf, lim, stream) as usize
        } else if if mtab[c].flag & 0xf != 0 { 1 } else { 0 } != 0 {
            match mtab[c].flag & 0xf {
                1 | 4 => {
                    let block_length = block_count(mtab, c);
                    if block_length >= 2 {
                        blocks[num_blocks].start = c as i32;
                        blocks[num_blocks].count = block_length as i32;
                        num_blocks += 1;
                        c += block_length as usize;
                    } else {
                        wbuf.push(b'<');
                        for i in 0..=depth {
                            sputx(codestr[i as usize], wbuf, lim);
                        }
                        wbuf.push(b'>');
                        wbuf.push(b' ');
                        wbuf.push(b'<');
                        for i in 0..mtab[c].len {
                            sputx(*mtab[c].code.offset(i as isize), wbuf, lim);
                        }
                        wbuf.push(b'>');
                        wbuf.push(b'\n');
                        count = count.wrapping_add(1)
                    }
                }
                2 => {
                    panic!("{}: Unexpected error...", "CMap");
                }
                8 => {}
                _ => {
                    panic!("{}: Unknown mapping type: {}", "CMap", mtab[c].flag & 0xf,);
                }
            }
        }
        /* Flush if necessary */
        if count >= 100 || wbuf.len() >= lim {
            if count > 100 {
                panic!("Unexpected error....: {}", count);
            }
            stream.add_str(&format!("{} beginbfchar\n", count));
            stream.add_slice(wbuf.as_slice());
            wbuf.clear();
            stream.add_str("endbfchar\n");
            count = 0;
        }
        c += 1;
    }
    if num_blocks > 0 {
        if count > 0 {
            stream.add_str(&format!("{} beginbfchar\n", count));
            stream.add_slice(wbuf.as_slice());
            wbuf.clear();
            stream.add_str("endbfchar\n");
            count = 0;
        }
        stream.add_str(&format!("{} beginbfrange\n", num_blocks));
        for i in 0..num_blocks {
            let c = blocks[i].start as usize;
            wbuf.push(b'<');
            for j in 0..depth {
                sputx(codestr[j as usize], wbuf, lim);
            }
            sputx(c as u8, wbuf, lim);
            wbuf.push(b'>');
            wbuf.push(b' ');
            wbuf.push(b'<');
            for j in 0..depth {
                sputx(codestr[j as usize], wbuf, lim);
            }
            sputx(c.wrapping_add(blocks[i].count as _) as u8, wbuf, lim);
            wbuf.push(b'>');
            wbuf.push(b' ');
            wbuf.push(b'<');
            for j in 0..mtab[c].len {
                sputx(*mtab[c].code.offset(j as isize), wbuf, lim);
            }
            wbuf.push(b'>');
            wbuf.push(b'\n');
        }
        stream.add_slice(wbuf.as_slice());
        wbuf.clear();
        stream.add_str("endbfrange\n");
    }
    count as i32
}

pub(crate) unsafe fn CMap_create_stream(cmap: &mut CMap) -> Option<pdf_stream> {
    if !cmap.is_valid() {
        warn!("Invalid CMap");
        return None;
    }
    if cmap.type_0 == 0 {
        return None;
    }
    let mut stream = pdf_stream::new(STREAM_COMPRESS);
    let stream_dict = stream.get_dict_mut();
    let csi = cmap.get_CIDSysInfo().unwrap_or_else(|| {
        if cmap.type_0 != 2 {
            &CSI_IDENTITY
        } else {
            &CSI_UNICODE
        }
    });
    if cmap.type_0 != 2 {
        let mut csi_dict = pdf_dict::new();
        csi_dict.set("Registry", pdf_string::new(csi.registry.as_bytes()));
        csi_dict.set("Ordering", pdf_string::new(csi.ordering.as_bytes()));
        csi_dict.set("Supplement", csi.supplement as f64);
        stream_dict.set("Type", "CMap");
        stream_dict.set("CMapName", cmap.name.as_str());
        stream_dict.set("CIDSystemInfo", csi_dict);
        if cmap.wmode != 0 {
            stream_dict.set("WMode", cmap.wmode as f64);
        }
    }
    /* TODO:
     * Predefined CMaps need not to be embedded.
     */
    if cmap.useCMap.is_some() {
        panic!("UseCMap found (not supported yet)...");
    }
    let mut wbuf = Vec::<u8>::with_capacity(4096);
    let mut codestr = vec![0_u8; cmap.profile.maxBytesIn as _];
    let lim = 4096
        - ((2_u64).wrapping_mul(
            cmap.profile
                .maxBytesIn
                .wrapping_add(cmap.profile.maxBytesOut) as _,
        ) as usize)
        + 16;
    /* Start CMap */
    stream.add_str("/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n");
    writeln!(wbuf, "/CMapName /{} def", cmap.name).unwrap();
    writeln!(wbuf, "/CMapType {} def", cmap.type_0).unwrap();

    if cmap.wmode != 0 && cmap.type_0 != 2 {
        writeln!(wbuf, "/WMode {} def", cmap.wmode).unwrap();
    }
    writeln!(
        wbuf,
        "/CIDSystemInfo <<\n  /Registry ({})\n  /Ordering ({})\n  /Supplement {}\n>> def",
        csi.registry, csi.ordering, csi.supplement,
    )
    .unwrap();

    stream.add_slice(wbuf.as_slice());
    wbuf.clear();
    /* codespacerange */
    writeln!(wbuf, "{} begincodespacerange", cmap.codespace.len()).unwrap();
    for csr in &cmap.codespace {
        wbuf.push(b'<');
        for j in 0..csr.dim {
            sputx(*csr.codeLo.offset(j as isize), &mut wbuf, lim);
        }
        wbuf.push(b'>');
        wbuf.push(b' ');
        wbuf.push(b'<');
        for j in 0..csr.dim {
            sputx(*csr.codeHi.offset(j as isize), &mut wbuf, lim);
        }
        wbuf.push(b'>');
        wbuf.push(b'\n');
    }
    stream.add_slice(wbuf.as_slice());
    wbuf.clear();
    stream.add_str("endcodespacerange\n");
    /* CMap body */
    if let Some(mapTbl) = cmap.mapTbl.as_deref_mut() {
        let count = write_map(mapTbl, 0, &mut codestr, 0, &mut wbuf, lim, &mut stream) as usize; /* Top node */
        if count > 0 {
            /* Flush */
            if count > 100 {
                panic!("Unexpected error....: {}", count);
            }
            stream.add_str(&format!("{} beginbfchar\n", count));
            stream.add_slice(wbuf.as_slice());
            stream.add_str("endbfchar\n");
            wbuf.clear();
        }
    }
    /* End CMap */
    stream.add_str("endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n");
    Some(stream)
}
