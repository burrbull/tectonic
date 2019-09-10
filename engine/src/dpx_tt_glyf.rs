#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn sfnt_set_table(
        sfont: *mut sfnt,
        tag: *const i8,
        data: *mut libc::c_void,
        length: SFNT_ULONG,
    );
    #[no_mangle]
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const i8) -> SFNT_ULONG;
    #[no_mangle]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const i8) -> SFNT_ULONG;
    #[no_mangle]
    fn put_big_endian(s: *mut libc::c_void, q: SFNT_LONG, n: i32) -> i32;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: i32,
    ) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut i8,
        len: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> u16;
    #[no_mangle]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> i16;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> u32;
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
    /* head, hhea, maxp */
    #[no_mangle]
    fn tt_pack_head_table(table: *mut tt_head_table) -> *mut i8;
    #[no_mangle]
    fn tt_read_head_table(sfont: *mut sfnt) -> *mut tt_head_table;
    #[no_mangle]
    fn tt_pack_hhea_table(table: *mut tt_hhea_table) -> *mut i8;
    #[no_mangle]
    fn tt_read_hhea_table(sfont: *mut sfnt) -> *mut tt_hhea_table;
    #[no_mangle]
    fn tt_pack_maxp_table(table: *mut tt_maxp_table) -> *mut i8;
    #[no_mangle]
    fn tt_read_maxp_table(sfont: *mut sfnt) -> *mut tt_maxp_table;
    /* vhea */
    #[no_mangle]
    fn tt_read_vhea_table(sfont: *mut sfnt) -> *mut tt_vhea_table;
    /* hmtx and vmtx */
    #[no_mangle]
    fn tt_read_longMetrics(
        sfont: *mut sfnt,
        numGlyphs: USHORT,
        numLongMetrics: USHORT,
        numExSideBearings: USHORT,
    ) -> *mut tt_longMetrics;
    /* OS/2 table */
    #[no_mangle]
    fn tt_read_os2__table(sfont: *mut sfnt) -> *mut tt_os2__table;
}
pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32>;
pub type rust_input_handle_t = *mut libc::c_void;
pub type BYTE = u8;
pub type SFNT_CHAR = libc::c_schar;
pub type USHORT = u16;
pub type SHORT = i16;
pub type SFNT_ULONG = u32;
pub type SFNT_LONG = i32;
pub type Fixed = u32;
pub type FWord = i16;
pub type uFWord = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table {
    pub tag: [i8; 4],
    pub check_sum: SFNT_ULONG,
    pub offset: SFNT_ULONG,
    pub length: SFNT_ULONG,
    pub data: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table_directory {
    pub version: SFNT_ULONG,
    pub num_tables: USHORT,
    pub search_range: USHORT,
    pub entry_selector: USHORT,
    pub range_shift: USHORT,
    pub num_kept_tables: USHORT,
    pub flags: *mut i8,
    pub tables: *mut sfnt_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt {
    pub type_0: i32,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: SFNT_ULONG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_glyph_desc {
    pub gid: USHORT,
    pub ogid: USHORT,
    pub advw: USHORT,
    pub advh: USHORT,
    pub lsb: SHORT,
    pub tsb: SHORT,
    pub llx: SHORT,
    pub lly: SHORT,
    pub urx: SHORT,
    pub ury: SHORT,
    pub length: SFNT_ULONG,
    pub data: *mut BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_glyphs {
    pub num_glyphs: USHORT,
    pub max_glyphs: USHORT,
    pub last_gid: USHORT,
    pub emsize: USHORT,
    pub dw: USHORT,
    pub default_advh: USHORT,
    pub default_tsb: SHORT,
    pub gd: *mut tt_glyph_desc,
    pub used_slot: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_head_table {
    pub version: Fixed,
    pub fontRevision: Fixed,
    pub checkSumAdjustment: SFNT_ULONG,
    pub magicNumber: SFNT_ULONG,
    pub flags: USHORT,
    pub unitsPerEm: USHORT,
    pub created: [BYTE; 8],
    pub modified: [BYTE; 8],
    pub xMin: FWord,
    pub yMin: FWord,
    pub xMax: FWord,
    pub yMax: FWord,
    pub macStyle: USHORT,
    pub lowestRecPPEM: USHORT,
    pub fontDirectionHint: SHORT,
    pub indexToLocFormat: SHORT,
    pub glyphDataFormat: SHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_maxp_table {
    pub version: Fixed,
    pub numGlyphs: USHORT,
    pub maxPoints: USHORT,
    pub maxContours: USHORT,
    pub maxComponentPoints: USHORT,
    pub maxComponentContours: USHORT,
    pub maxZones: USHORT,
    pub maxTwilightPoints: USHORT,
    pub maxStorage: USHORT,
    pub maxFunctionDefs: USHORT,
    pub maxInstructionDefs: USHORT,
    pub maxStackElements: USHORT,
    pub maxSizeOfInstructions: USHORT,
    pub maxComponentElements: USHORT,
    pub maxComponentDepth: USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_hhea_table {
    pub version: Fixed,
    pub ascent: FWord,
    pub descent: FWord,
    pub lineGap: FWord,
    pub advanceWidthMax: uFWord,
    pub minLeftSideBearing: FWord,
    pub minRightSideBearing: FWord,
    pub xMaxExtent: FWord,
    pub caretSlopeRise: SHORT,
    pub caretSlopeRun: SHORT,
    pub caretOffset: FWord,
    pub reserved: [SHORT; 4],
    pub metricDataFormat: SHORT,
    pub numOfLongHorMetrics: USHORT,
    pub numOfExSideBearings: USHORT,
    /* extra information */
}
/* hmtx and vmtx */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_longMetrics {
    pub advance: USHORT,
    pub sideBearing: SHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_os2__table {
    pub version: USHORT,
    pub xAvgCharWidth: SHORT,
    pub usWeightClass: USHORT,
    pub usWidthClass: USHORT,
    pub fsType: SHORT,
    pub ySubscriptXSize: SHORT,
    pub ySubscriptYSize: SHORT,
    pub ySubscriptXOffset: SHORT,
    pub ySubscriptYOffset: SHORT,
    pub ySuperscriptXSize: SHORT,
    pub ySuperscriptYSize: SHORT,
    pub ySuperscriptXOffset: SHORT,
    pub ySuperscriptYOffset: SHORT,
    pub yStrikeoutSize: SHORT,
    pub yStrikeoutPosition: SHORT,
    pub sFamilyClass: SHORT,
    pub panose: [BYTE; 10],
    pub ulUnicodeRange1: SFNT_ULONG,
    pub ulUnicodeRange2: SFNT_ULONG,
    pub ulUnicodeRange3: SFNT_ULONG,
    pub ulUnicodeRange4: SFNT_ULONG,
    pub achVendID: [SFNT_CHAR; 4],
    pub fsSelection: USHORT,
    pub usFirstCharIndex: USHORT,
    pub usLastCharIndex: USHORT,
    pub sTypoAscender: SHORT,
    pub sTypoDescender: SHORT,
    pub sTypoLineGap: SHORT,
    pub usWinAscent: USHORT,
    pub usWinDescent: USHORT,
    pub ulCodePageRange1: SFNT_ULONG,
    pub ulCodePageRange2: SFNT_ULONG,
    pub sxHeight: SHORT,
    pub sCapHeight: SHORT,
    pub usDefaultChar: USHORT,
    pub usBreakChar: USHORT,
    pub usMaxContext: USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_vhea_table {
    pub version: Fixed,
    pub vertTypoAscender: SHORT,
    pub vertTypoDescender: SHORT,
    pub vertTypoLineGap: SHORT,
    pub advanceHeightMax: SHORT,
    pub minTopSideBearing: SHORT,
    pub minBottomSideBearing: SHORT,
    pub yMaxExtent: SHORT,
    pub caretSlopeRise: SHORT,
    pub caretSlopeRun: SHORT,
    pub caretOffset: SHORT,
    pub reserved: [SHORT; 4],
    pub metricDataFormat: SHORT,
    pub numOfLongVerMetrics: USHORT,
    pub numOfExSideBearings: USHORT,
    /* extra information */
}
unsafe extern "C" fn find_empty_slot(mut g: *mut tt_glyphs) -> USHORT {
    let mut gid: USHORT = 0;
    if !g.is_null() {
    } else {
        __assert_fail(
            b"g\x00" as *const u8 as *const i8,
            b"dpx-tt_glyf.c\x00" as *const u8 as *const i8,
            47i32 as u32,
            (*::std::mem::transmute::<&[u8; 43], &[i8; 43]>(
                b"USHORT find_empty_slot(struct tt_glyphs *)\x00",
            ))
            .as_ptr(),
        );
    }
    gid = 0i32 as USHORT;
    while (gid as i32) < 65534i32 {
        if *(*g).used_slot.offset((gid as i32 / 8i32) as isize) as i32
            & 1i32 << 7i32 - gid as i32 % 8i32
            == 0
        {
            break;
        }
        gid = gid.wrapping_add(1)
    }
    if gid as i32 == 65534i32 {
        _tt_abort(b"No empty glyph slot available.\x00" as *const u8 as *const i8);
    }
    return gid;
}
#[no_mangle]
pub unsafe extern "C" fn tt_find_glyph(mut g: *mut tt_glyphs, mut gid: USHORT) -> USHORT {
    let mut idx: USHORT = 0;
    let mut new_gid: USHORT = 0i32 as USHORT;
    if !g.is_null() {
    } else {
        __assert_fail(
            b"g\x00" as *const u8 as *const i8,
            b"dpx-tt_glyf.c\x00" as *const u8 as *const i8,
            64i32 as u32,
            (*::std::mem::transmute::<&[u8; 49], &[i8; 49]>(
                b"USHORT tt_find_glyph(struct tt_glyphs *, USHORT)\x00",
            ))
            .as_ptr(),
        );
    }
    idx = 0i32 as USHORT;
    while (idx as i32) < (*g).num_glyphs as i32 {
        if gid as i32 == (*(*g).gd.offset(idx as isize)).ogid as i32 {
            new_gid = (*(*g).gd.offset(idx as isize)).gid;
            break;
        } else {
            idx = idx.wrapping_add(1)
        }
    }
    return new_gid;
}
#[no_mangle]
pub unsafe extern "C" fn tt_get_index(mut g: *mut tt_glyphs, mut gid: USHORT) -> USHORT {
    let mut idx: USHORT = 0;
    if !g.is_null() {
    } else {
        __assert_fail(
            b"g\x00" as *const u8 as *const i8,
            b"dpx-tt_glyf.c\x00" as *const u8 as *const i8,
            81i32 as u32,
            (*::std::mem::transmute::<&[u8; 48], &[i8; 48]>(
                b"USHORT tt_get_index(struct tt_glyphs *, USHORT)\x00",
            ))
            .as_ptr(),
        );
    }
    idx = 0i32 as USHORT;
    while (idx as i32) < (*g).num_glyphs as i32 {
        if gid as i32 == (*(*g).gd.offset(idx as isize)).gid as i32 {
            break;
        }
        idx = idx.wrapping_add(1)
    }
    if idx as i32 == (*g).num_glyphs as i32 {
        idx = 0i32 as USHORT
    }
    return idx;
}
#[no_mangle]
pub unsafe extern "C" fn tt_add_glyph(
    mut g: *mut tt_glyphs,
    mut gid: USHORT,
    mut new_gid: USHORT,
) -> USHORT {
    if !g.is_null() {
    } else {
        __assert_fail(
            b"g\x00" as *const u8 as *const i8,
            b"dpx-tt_glyf.c\x00" as *const u8 as *const i8,
            96i32 as u32,
            (*::std::mem::transmute::<&[u8; 56], &[i8; 56]>(
                b"USHORT tt_add_glyph(struct tt_glyphs *, USHORT, USHORT)\x00",
            ))
            .as_ptr(),
        );
    }
    if *(*g)
        .used_slot
        .offset((new_gid as i32 / 8i32) as isize) as i32
        & 1i32 << 7i32 - new_gid as i32 % 8i32
        != 0
    {
        dpx_warning(
            b"Slot %u already used.\x00" as *const u8 as *const i8,
            new_gid as i32,
        );
    } else {
        if (*g).num_glyphs as i32 + 1i32 >= 65534i32 {
            _tt_abort(b"Too many glyphs.\x00" as *const u8 as *const i8);
        }
        if (*g).num_glyphs as i32 >= (*g).max_glyphs as i32 {
            (*g).max_glyphs = ((*g).max_glyphs as i32 + 256i32) as USHORT;
            (*g).gd = renew(
                (*g).gd as *mut libc::c_void,
                ((*g).max_glyphs as u32 as u64)
                    .wrapping_mul(::std::mem::size_of::<tt_glyph_desc>() as u64)
                    as u32,
            ) as *mut tt_glyph_desc
        }
        (*(*g).gd.offset((*g).num_glyphs as isize)).gid = new_gid;
        (*(*g).gd.offset((*g).num_glyphs as isize)).ogid = gid;
        (*(*g).gd.offset((*g).num_glyphs as isize)).length = 0i32 as SFNT_ULONG;
        let ref mut fresh0 = (*(*g).gd.offset((*g).num_glyphs as isize)).data;
        *fresh0 = 0 as *mut BYTE;
        let ref mut fresh1 = *(*g)
            .used_slot
            .offset((new_gid as i32 / 8i32) as isize);
        *fresh1 = (*fresh1 as i32 | 1i32 << 7i32 - new_gid as i32 % 8i32)
            as u8;
        (*g).num_glyphs = ((*g).num_glyphs as i32 + 1i32) as USHORT
    }
    if new_gid as i32 > (*g).last_gid as i32 {
        (*g).last_gid = new_gid
    }
    return new_gid;
}
/*
 * Initialization
 */
#[no_mangle]
pub unsafe extern "C" fn tt_build_init() -> *mut tt_glyphs {
    let mut g: *mut tt_glyphs = 0 as *mut tt_glyphs;
    g = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<tt_glyphs>() as u64) as u32)
        as *mut tt_glyphs;
    (*g).num_glyphs = 0i32 as USHORT;
    (*g).max_glyphs = 0i32 as USHORT;
    (*g).last_gid = 0i32 as USHORT;
    (*g).emsize = 1i32 as USHORT;
    (*g).default_advh = 0i32 as USHORT;
    (*g).default_tsb = 0i32 as SHORT;
    (*g).gd = 0 as *mut tt_glyph_desc;
    (*g).used_slot = new((8192i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<u8>() as u64)
        as u32) as *mut u8;
    memset(
        (*g).used_slot as *mut libc::c_void,
        0i32,
        8192i32 as u64,
    );
    tt_add_glyph(g, 0i32 as USHORT, 0i32 as USHORT);
    return g;
}
#[no_mangle]
pub unsafe extern "C" fn tt_build_finish(mut g: *mut tt_glyphs) {
    if !g.is_null() {
        if !(*g).gd.is_null() {
            let mut idx: USHORT = 0;
            idx = 0i32 as USHORT;
            while (idx as i32) < (*g).num_glyphs as i32 {
                free((*(*g).gd.offset(idx as isize)).data as *mut libc::c_void);
                idx = idx.wrapping_add(1)
            }
            free((*g).gd as *mut libc::c_void);
        }
        free((*g).used_slot as *mut libc::c_void);
        free(g as *mut libc::c_void);
    };
}
#[inline]
unsafe extern "C" fn glyf_cmp(
    mut v1: *const libc::c_void,
    mut v2: *const libc::c_void,
) -> i32 {
    let mut cmp: i32 = 0i32;
    let mut sv1: *const tt_glyph_desc = 0 as *const tt_glyph_desc;
    let mut sv2: *const tt_glyph_desc = 0 as *const tt_glyph_desc;
    sv1 = v1 as *const tt_glyph_desc;
    sv2 = v2 as *const tt_glyph_desc;
    if (*sv1).gid as i32 == (*sv2).gid as i32 {
        cmp = 0i32
    } else if ((*sv1).gid as i32) < (*sv2).gid as i32 {
        cmp = -1i32
    } else {
        cmp = 1i32
    }
    return cmp;
}
#[no_mangle]
pub unsafe extern "C" fn tt_build_tables(
    mut sfont: *mut sfnt,
    mut g: *mut tt_glyphs,
) -> i32 {
    let mut hmtx_table_data: *mut i8 = 0 as *mut i8;
    let mut loca_table_data: *mut i8 = 0 as *mut i8;
    let mut glyf_table_data: *mut i8 = 0 as *mut i8;
    let mut hmtx_table_size: SFNT_ULONG = 0;
    let mut loca_table_size: SFNT_ULONG = 0;
    let mut glyf_table_size: SFNT_ULONG = 0;
    /* some information available from other TrueType table */
    let mut head: *mut tt_head_table = 0 as *mut tt_head_table;
    let mut hhea: *mut tt_hhea_table = 0 as *mut tt_hhea_table;
    let mut maxp: *mut tt_maxp_table = 0 as *mut tt_maxp_table;
    let mut hmtx: *mut tt_longMetrics = 0 as *mut tt_longMetrics;
    let mut vmtx: *mut tt_longMetrics = 0 as *mut tt_longMetrics;
    let mut os2: *mut tt_os2__table = 0 as *mut tt_os2__table;
    /* temp */
    let mut location: *mut SFNT_ULONG = 0 as *mut SFNT_ULONG; /* Estimate most frequently appeared width */
    let mut offset: SFNT_ULONG = 0;
    let mut i: i32 = 0;
    let mut w_stat: *mut USHORT = 0 as *mut USHORT;
    if !g.is_null() {
    } else {
        __assert_fail(
            b"g\x00" as *const u8 as *const i8,
            b"dpx-tt_glyf.c\x00" as *const u8 as *const i8,
            213i32 as u32,
            (*::std::mem::transmute::<&[u8; 48], &[i8; 48]>(
                b"int tt_build_tables(sfnt *, struct tt_glyphs *)\x00",
            ))
            .as_ptr(),
        );
    }
    if sfont.is_null() || (*sfont).handle.is_null() {
        _tt_abort(b"File not opened.\x00" as *const u8 as *const i8);
    }
    if (*sfont).type_0 != 1i32 << 0i32
        && (*sfont).type_0 != 1i32 << 4i32
        && (*sfont).type_0 != 1i32 << 8i32
    {
        _tt_abort(b"Invalid font type\x00" as *const u8 as *const i8);
    }
    if (*g).num_glyphs as i32 > 65534i32 {
        _tt_abort(b"Too many glyphs.\x00" as *const u8 as *const i8);
    }
    /*
     * Read head, hhea, maxp, loca:
     *
     *   unitsPerEm       --> head
     *   numHMetrics      --> hhea
     *   indexToLocFormat --> head
     *   numGlyphs        --> maxp
     */
    head = tt_read_head_table(sfont);
    hhea = tt_read_hhea_table(sfont);
    maxp = tt_read_maxp_table(sfont);
    if (*hhea).metricDataFormat as i32 != 0i32 {
        _tt_abort(b"Unknown metricDataFormat.\x00" as *const u8 as *const i8);
    }
    (*g).emsize = (*head).unitsPerEm;
    sfnt_locate_table(sfont, b"hmtx\x00" as *const u8 as *const i8);
    hmtx = tt_read_longMetrics(
        sfont,
        (*maxp).numGlyphs,
        (*hhea).numOfLongHorMetrics,
        (*hhea).numOfExSideBearings,
    );
    os2 = tt_read_os2__table(sfont);
    if !os2.is_null() {
        (*g).default_advh =
            ((*os2).sTypoAscender as i32 - (*os2).sTypoDescender as i32) as USHORT;
        (*g).default_tsb =
            ((*g).default_advh as i32 - (*os2).sTypoAscender as i32) as SHORT
    }
    if sfnt_find_table_pos(sfont, b"vmtx\x00" as *const u8 as *const i8)
        > 0i32 as u32
    {
        let mut vhea: *mut tt_vhea_table = 0 as *mut tt_vhea_table;
        vhea = tt_read_vhea_table(sfont);
        sfnt_locate_table(sfont, b"vmtx\x00" as *const u8 as *const i8);
        vmtx = tt_read_longMetrics(
            sfont,
            (*maxp).numGlyphs,
            (*vhea).numOfLongVerMetrics,
            (*vhea).numOfExSideBearings,
        );
        free(vhea as *mut libc::c_void);
    } else {
        vmtx = 0 as *mut tt_longMetrics
    }
    sfnt_locate_table(sfont, b"loca\x00" as *const u8 as *const i8);
    location = new(
        (((*maxp).numGlyphs as i32 + 1i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<SFNT_ULONG>() as u64) as u32,
    ) as *mut SFNT_ULONG;
    if (*head).indexToLocFormat as i32 == 0i32 {
        i = 0i32;
        while i <= (*maxp).numGlyphs as i32 {
            *location.offset(i as isize) = (2i32 as u32)
                .wrapping_mul(tt_get_unsigned_pair((*sfont).handle) as SFNT_ULONG);
            i += 1
        }
    } else if (*head).indexToLocFormat as i32 == 1i32 {
        i = 0i32;
        while i <= (*maxp).numGlyphs as i32 {
            *location.offset(i as isize) = tt_get_unsigned_quad((*sfont).handle);
            i += 1
        }
    } else {
        _tt_abort(b"Unknown IndexToLocFormat.\x00" as *const u8 as *const i8);
    }
    w_stat = new(
        (((*g).emsize as i32 + 2i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<USHORT>() as u64) as u32,
    ) as *mut USHORT;
    memset(
        w_stat as *mut libc::c_void,
        0i32,
        (::std::mem::size_of::<USHORT>() as u64)
            .wrapping_mul(((*g).emsize as i32 + 2i32) as u64),
    );
    /*
     * Read glyf table.
     */
    offset = sfnt_locate_table(sfont, b"glyf\x00" as *const u8 as *const i8);
    /*
     * The num_glyphs may grow when composite glyph is found.
     * A component of glyph refered by a composite glyph is appended
     * to used_glyphs if it is not already registered in used_glyphs.
     * Glyph programs of composite glyphs are modified so that it
     * correctly refer to new gid of their components.
     */
    i = 0i32; /* old gid */
    while i < 65534i32 {
        let mut gid: USHORT = 0;
        let mut loc: SFNT_ULONG = 0;
        let mut len: SFNT_ULONG = 0;
        let mut p: *mut BYTE = 0 as *mut BYTE;
        let mut endptr: *mut BYTE = 0 as *mut BYTE;
        let mut number_of_contours: SHORT = 0;
        if i >= (*g).num_glyphs as i32 {
            break;
        }
        gid = (*(*g).gd.offset(i as isize)).ogid;
        if gid as i32 >= (*maxp).numGlyphs as i32 {
            _tt_abort(
                b"Invalid glyph index (gid %u)\x00" as *const u8 as *const i8,
                gid as i32,
            );
        }
        loc = *location.offset(gid as isize);
        len = (*location.offset((gid as i32 + 1i32) as isize)).wrapping_sub(loc);
        (*(*g).gd.offset(i as isize)).advw = (*hmtx.offset(gid as isize)).advance;
        (*(*g).gd.offset(i as isize)).lsb = (*hmtx.offset(gid as isize)).sideBearing;
        if !vmtx.is_null() {
            (*(*g).gd.offset(i as isize)).advh = (*vmtx.offset(gid as isize)).advance;
            (*(*g).gd.offset(i as isize)).tsb = (*vmtx.offset(gid as isize)).sideBearing
        } else {
            (*(*g).gd.offset(i as isize)).advh = (*g).default_advh;
            (*(*g).gd.offset(i as isize)).tsb = (*g).default_tsb
        }
        (*(*g).gd.offset(i as isize)).length = len;
        let ref mut fresh2 = (*(*g).gd.offset(i as isize)).data;
        *fresh2 = 0 as *mut BYTE;
        if (*(*g).gd.offset(i as isize)).advw as i32 <= (*g).emsize as i32 {
            let ref mut fresh3 = *w_stat.offset((*(*g).gd.offset(i as isize)).advw as isize);
            *fresh3 = (*fresh3 as i32 + 1i32) as USHORT
        } else {
            let ref mut fresh4 = *w_stat.offset(((*g).emsize as i32 + 1i32) as isize);
            *fresh4 = (*fresh4 as i32 + 1i32) as USHORT
            /* larger than em */
        }
        if !(len == 0i32 as u32) {
            if len < 10i32 as u32 {
                _tt_abort(
                    b"Invalid TrueType glyph data (gid %u).\x00" as *const u8
                        as *const i8,
                    gid as i32,
                );
            }
            p = new((len as u64)
                .wrapping_mul(::std::mem::size_of::<BYTE>() as u64)
                as u32) as *mut BYTE;
            let ref mut fresh5 = (*(*g).gd.offset(i as isize)).data;
            *fresh5 = p;
            endptr = p.offset(len as isize);
            ttstub_input_seek((*sfont).handle, offset.wrapping_add(loc) as ssize_t, 0i32);
            number_of_contours = tt_get_signed_pair((*sfont).handle);
            p = p.offset(put_big_endian(
                p as *mut libc::c_void,
                number_of_contours as SFNT_LONG,
                2i32,
            ) as isize);
            /* BoundingBox: FWord x 4 */
            (*(*g).gd.offset(i as isize)).llx = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).lly = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).urx = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).ury = tt_get_signed_pair((*sfont).handle);
            /* _FIXME_ */
            if vmtx.is_null() {
                /* vertOriginY == sTypeAscender */
                (*(*g).gd.offset(i as isize)).tsb = ((*g).default_advh as i32
                    - (*g).default_tsb as i32
                    - (*(*g).gd.offset(i as isize)).ury as i32)
                    as SHORT
            }
            p = p.offset(put_big_endian(
                p as *mut libc::c_void,
                (*(*g).gd.offset(i as isize)).llx as SFNT_LONG,
                2i32,
            ) as isize);
            p = p.offset(put_big_endian(
                p as *mut libc::c_void,
                (*(*g).gd.offset(i as isize)).lly as SFNT_LONG,
                2i32,
            ) as isize);
            p = p.offset(put_big_endian(
                p as *mut libc::c_void,
                (*(*g).gd.offset(i as isize)).urx as SFNT_LONG,
                2i32,
            ) as isize);
            p = p.offset(put_big_endian(
                p as *mut libc::c_void,
                (*(*g).gd.offset(i as isize)).ury as SFNT_LONG,
                2i32,
            ) as isize);
            /* Read evrything else. */
            ttstub_input_read(
                (*sfont).handle,
                p as *mut i8,
                len.wrapping_sub(10i32 as u32) as size_t,
            );
            /*
             * Fix GIDs of composite glyphs.
             */
            if (number_of_contours as i32) < 0i32 {
                let mut flags: USHORT = 0; /* flag, gid of a component */
                let mut cgid: USHORT = 0;
                let mut new_gid: USHORT = 0;
                loop {
                    if p >= endptr {
                        _tt_abort(
                            b"Invalid TrueType glyph data (gid %u): %u bytes\x00" as *const u8
                                as *const i8,
                            gid as i32,
                            len,
                        );
                    }
                    /*
                     * Flags and gid of component glyph are both USHORT.
                     */
                    flags = ((*p as i32) << 8i32 | *p.offset(1) as i32) as USHORT;
                    p = p.offset(2);
                    cgid = ((*p as i32) << 8i32 | *p.offset(1) as i32) as USHORT;
                    if cgid as i32 >= (*maxp).numGlyphs as i32 {
                        _tt_abort(
                            b"Invalid gid (%u > %u) in composite glyph %u.\x00" as *const u8
                                as *const i8,
                            cgid as i32,
                            (*maxp).numGlyphs as i32,
                            gid as i32,
                        );
                    }
                    new_gid = tt_find_glyph(g, cgid);
                    if new_gid as i32 == 0i32 {
                        new_gid = tt_add_glyph(g, cgid, find_empty_slot(g))
                    }
                    p = p.offset(
                        put_big_endian(p as *mut libc::c_void, new_gid as SFNT_LONG, 2i32) as isize,
                    );
                    /*
                     * Just skip remaining part.
                     */
                    p = p.offset(
                        (if flags as i32 & 1i32 << 0i32 != 0 {
                            4i32
                        } else {
                            2i32
                        }) as isize,
                    );
                    if flags as i32 & 1i32 << 3i32 != 0 {
                        /* F2Dot14 */
                        p = p.offset(2)
                    } else if flags as i32 & 1i32 << 6i32 != 0 {
                        /* F2Dot14 x 2 */
                        p = p.offset(4)
                    } else if flags as i32 & 1i32 << 7i32 != 0 {
                        /* F2Dot14 x 4 */
                        p = p.offset(8)
                    }
                    if !(flags as i32 & 1i32 << 5i32 != 0) {
                        break;
                    }
                }
            }
        }
        /* Does not contains any data. */
        i += 1
    }
    free(location as *mut libc::c_void);
    free(hmtx as *mut libc::c_void);
    free(vmtx as *mut libc::c_void);
    let mut max_count: i32 = -1i32;
    (*g).dw = (*(*g).gd.offset(0)).advw;
    i = 0i32;
    while i < (*g).emsize as i32 + 1i32 {
        if *w_stat.offset(i as isize) as i32 > max_count {
            max_count = *w_stat.offset(i as isize) as i32;
            (*g).dw = i as USHORT
        }
        i += 1
    }
    free(w_stat as *mut libc::c_void);
    qsort(
        (*g).gd as *mut libc::c_void,
        (*g).num_glyphs as size_t,
        ::std::mem::size_of::<tt_glyph_desc>() as u64,
        Some(
            glyf_cmp
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *const libc::c_void,
                ) -> i32,
        ),
    );
    let mut prev: USHORT = 0;
    let mut last_advw: USHORT = 0;
    let mut p_0: *mut i8 = 0 as *mut i8;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut padlen: i32 = 0;
    let mut num_hm_known: i32 = 0;
    glyf_table_size = 0u64 as SFNT_ULONG;
    num_hm_known = 0i32;
    last_advw = (*(*g)
        .gd
        .offset(((*g).num_glyphs as i32 - 1i32) as isize))
    .advw;
    i = (*g).num_glyphs as i32 - 1i32;
    while i >= 0i32 {
        padlen = (if (*(*g).gd.offset(i as isize))
            .length
            .wrapping_rem(4i32 as u32)
            != 0
        {
            (4i32 as u32).wrapping_sub(
                (*(*g).gd.offset(i as isize))
                    .length
                    .wrapping_rem(4i32 as u32),
            )
        } else {
            0i32 as u32
        }) as i32;
        glyf_table_size = (glyf_table_size as u32).wrapping_add(
            (*(*g).gd.offset(i as isize))
                .length
                .wrapping_add(padlen as u32),
        ) as SFNT_ULONG as SFNT_ULONG;
        if num_hm_known == 0
            && last_advw as i32 != (*(*g).gd.offset(i as isize)).advw as i32
        {
            (*hhea).numOfLongHorMetrics =
                ((*(*g).gd.offset(i as isize)).gid as i32 + 2i32) as USHORT;
            num_hm_known = 1i32
        }
        i -= 1
    }
    /* All advance widths are same. */
    if num_hm_known == 0 {
        (*hhea).numOfLongHorMetrics = 1i32 as USHORT
    }
    hmtx_table_size = ((*hhea).numOfLongHorMetrics as i32 * 2i32
        + ((*g).last_gid as i32 + 1i32) * 2i32) as SFNT_ULONG;
    /*
     * Choosing short format does not always give good result
     * when compressed. Sometimes increases size.
     */
    if (glyf_table_size as u64) < 0x20000 {
        (*head).indexToLocFormat = 0i32 as SHORT;
        loca_table_size = (((*g).last_gid as i32 + 2i32) * 2i32) as SFNT_ULONG
    } else {
        (*head).indexToLocFormat = 1i32 as SHORT;
        loca_table_size = (((*g).last_gid as i32 + 2i32) * 4i32) as SFNT_ULONG
    }
    p_0 = new((hmtx_table_size as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
        as u32) as *mut i8;
    hmtx_table_data = p_0;
    q = new((loca_table_size as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
        as u32) as *mut i8;
    loca_table_data = q;
    glyf_table_data = new((glyf_table_size as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
        as u32) as *mut i8;
    offset = 0u64 as SFNT_ULONG;
    prev = 0i32 as USHORT;
    i = 0i32;
    while i < (*g).num_glyphs as i32 {
        let mut gap: i32 = 0;
        let mut j: i32 = 0;
        gap = (*(*g).gd.offset(i as isize)).gid as i32 - prev as i32 - 1i32;
        j = 1i32;
        while j <= gap {
            if prev as i32 + j == (*hhea).numOfLongHorMetrics as i32 - 1i32 {
                p_0 = p_0.offset(put_big_endian(
                    p_0 as *mut libc::c_void,
                    last_advw as SFNT_LONG,
                    2i32,
                ) as isize)
            } else if prev as i32 + j < (*hhea).numOfLongHorMetrics as i32 {
                p_0 = p_0.offset(put_big_endian(p_0 as *mut libc::c_void, 0i32, 2i32) as isize)
            }
            p_0 = p_0.offset(put_big_endian(p_0 as *mut libc::c_void, 0i32, 2i32) as isize);
            if (*head).indexToLocFormat as i32 == 0i32 {
                q = q.offset(put_big_endian(
                    q as *mut libc::c_void,
                    offset.wrapping_div(2i32 as u32) as USHORT as SFNT_LONG,
                    2i32,
                ) as isize)
            } else {
                q = q.offset(
                    put_big_endian(q as *mut libc::c_void, offset as SFNT_LONG, 4i32) as isize,
                )
            }
            j += 1
        }
        padlen = (if (*(*g).gd.offset(i as isize))
            .length
            .wrapping_rem(4i32 as u32)
            != 0
        {
            (4i32 as u32).wrapping_sub(
                (*(*g).gd.offset(i as isize))
                    .length
                    .wrapping_rem(4i32 as u32),
            )
        } else {
            0i32 as u32
        }) as i32;
        if ((*(*g).gd.offset(i as isize)).gid as i32)
            < (*hhea).numOfLongHorMetrics as i32
        {
            p_0 = p_0.offset(put_big_endian(
                p_0 as *mut libc::c_void,
                (*(*g).gd.offset(i as isize)).advw as SFNT_LONG,
                2i32,
            ) as isize)
        }
        p_0 = p_0.offset(put_big_endian(
            p_0 as *mut libc::c_void,
            (*(*g).gd.offset(i as isize)).lsb as SFNT_LONG,
            2i32,
        ) as isize);
        if (*head).indexToLocFormat as i32 == 0i32 {
            q = q.offset(put_big_endian(
                q as *mut libc::c_void,
                offset.wrapping_div(2i32 as u32) as USHORT as SFNT_LONG,
                2i32,
            ) as isize)
        } else {
            q = q.offset(put_big_endian(q as *mut libc::c_void, offset as SFNT_LONG, 4i32) as isize)
        }
        memset(
            glyf_table_data.offset(offset as isize) as *mut libc::c_void,
            0i32,
            (*(*g).gd.offset(i as isize))
                .length
                .wrapping_add(padlen as u32) as u64,
        );
        memcpy(
            glyf_table_data.offset(offset as isize) as *mut libc::c_void,
            (*(*g).gd.offset(i as isize)).data as *const libc::c_void,
            (*(*g).gd.offset(i as isize)).length as u64,
        );
        offset = (offset as u32).wrapping_add(
            (*(*g).gd.offset(i as isize))
                .length
                .wrapping_add(padlen as u32),
        ) as SFNT_ULONG as SFNT_ULONG;
        prev = (*(*g).gd.offset(i as isize)).gid;
        /* free data here since it consume much memory */
        free((*(*g).gd.offset(i as isize)).data as *mut libc::c_void);
        (*(*g).gd.offset(i as isize)).length = 0i32 as SFNT_ULONG;
        let ref mut fresh6 = (*(*g).gd.offset(i as isize)).data;
        *fresh6 = 0 as *mut BYTE;
        i += 1
    }
    if (*head).indexToLocFormat as i32 == 0i32 {
        q = q.offset(put_big_endian(
            q as *mut libc::c_void,
            offset.wrapping_div(2i32 as u32) as USHORT as SFNT_LONG,
            2i32,
        ) as isize)
    } else {
        q = q.offset(put_big_endian(q as *mut libc::c_void, offset as SFNT_LONG, 4i32) as isize)
    }
    sfnt_set_table(
        sfont,
        b"hmtx\x00" as *const u8 as *const i8,
        hmtx_table_data as *mut libc::c_void,
        hmtx_table_size,
    );
    sfnt_set_table(
        sfont,
        b"loca\x00" as *const u8 as *const i8,
        loca_table_data as *mut libc::c_void,
        loca_table_size,
    );
    sfnt_set_table(
        sfont,
        b"glyf\x00" as *const u8 as *const i8,
        glyf_table_data as *mut libc::c_void,
        glyf_table_size,
    );
    (*head).checkSumAdjustment = 0i32 as SFNT_ULONG;
    (*maxp).numGlyphs = ((*g).last_gid as i32 + 1i32) as USHORT;
    /* TODO */
    sfnt_set_table(
        sfont,
        b"maxp\x00" as *const u8 as *const i8,
        tt_pack_maxp_table(maxp) as *mut libc::c_void,
        32u64 as SFNT_ULONG,
    );
    sfnt_set_table(
        sfont,
        b"hhea\x00" as *const u8 as *const i8,
        tt_pack_hhea_table(hhea) as *mut libc::c_void,
        36u64 as SFNT_ULONG,
    );
    sfnt_set_table(
        sfont,
        b"head\x00" as *const u8 as *const i8,
        tt_pack_head_table(head) as *mut libc::c_void,
        54u64 as SFNT_ULONG,
    );
    free(maxp as *mut libc::c_void);
    free(hhea as *mut libc::c_void);
    free(head as *mut libc::c_void);
    free(os2 as *mut libc::c_void);
    return 0i32;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

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
/* GID in original font */
/* optimal value for DW */
/* default value */
/* default value */
#[no_mangle]
pub unsafe extern "C" fn tt_get_metrics(
    mut sfont: *mut sfnt,
    mut g: *mut tt_glyphs,
) -> i32 {
    let mut head: *mut tt_head_table = 0 as *mut tt_head_table;
    let mut hhea: *mut tt_hhea_table = 0 as *mut tt_hhea_table;
    let mut maxp: *mut tt_maxp_table = 0 as *mut tt_maxp_table;
    let mut hmtx: *mut tt_longMetrics = 0 as *mut tt_longMetrics;
    let mut vmtx: *mut tt_longMetrics = 0 as *mut tt_longMetrics;
    let mut os2: *mut tt_os2__table = 0 as *mut tt_os2__table;
    /* temp */
    let mut location: *mut SFNT_ULONG = 0 as *mut SFNT_ULONG;
    let mut offset: SFNT_ULONG = 0;
    let mut i: u32 = 0;
    let mut w_stat: *mut USHORT = 0 as *mut USHORT;
    if !g.is_null() {
    } else {
        __assert_fail(
            b"g\x00" as *const u8 as *const i8,
            b"dpx-tt_glyf.c\x00" as *const u8 as *const i8,
            519i32 as u32,
            (*::std::mem::transmute::<&[u8; 47], &[i8; 47]>(
                b"int tt_get_metrics(sfnt *, struct tt_glyphs *)\x00",
            ))
            .as_ptr(),
        );
    }
    if sfont.is_null() || (*sfont).handle.is_null() {
        _tt_abort(b"File not opened.\x00" as *const u8 as *const i8);
    }
    if (*sfont).type_0 != 1i32 << 0i32
        && (*sfont).type_0 != 1i32 << 4i32
        && (*sfont).type_0 != 1i32 << 8i32
    {
        _tt_abort(b"Invalid font type\x00" as *const u8 as *const i8);
    }
    /*
     * Read head, hhea, maxp, loca:
     *
     *   unitsPerEm       --> head
     *   numHMetrics      --> hhea
     *   indexToLocFormat --> head
     *   numGlyphs        --> maxp
     */
    head = tt_read_head_table(sfont);
    hhea = tt_read_hhea_table(sfont);
    maxp = tt_read_maxp_table(sfont);
    if (*hhea).metricDataFormat as i32 != 0i32 {
        _tt_abort(b"Unknown metricDataFormat.\x00" as *const u8 as *const i8);
    }
    (*g).emsize = (*head).unitsPerEm;
    sfnt_locate_table(sfont, b"hmtx\x00" as *const u8 as *const i8);
    hmtx = tt_read_longMetrics(
        sfont,
        (*maxp).numGlyphs,
        (*hhea).numOfLongHorMetrics,
        (*hhea).numOfExSideBearings,
    );
    os2 = tt_read_os2__table(sfont);
    (*g).default_advh =
        ((*os2).sTypoAscender as i32 - (*os2).sTypoDescender as i32) as USHORT;
    (*g).default_tsb =
        ((*g).default_advh as i32 - (*os2).sTypoAscender as i32) as SHORT;
    if sfnt_find_table_pos(sfont, b"vmtx\x00" as *const u8 as *const i8)
        > 0i32 as u32
    {
        let mut vhea: *mut tt_vhea_table = 0 as *mut tt_vhea_table;
        vhea = tt_read_vhea_table(sfont);
        sfnt_locate_table(sfont, b"vmtx\x00" as *const u8 as *const i8);
        vmtx = tt_read_longMetrics(
            sfont,
            (*maxp).numGlyphs,
            (*vhea).numOfLongVerMetrics,
            (*vhea).numOfExSideBearings,
        );
        free(vhea as *mut libc::c_void);
    } else {
        vmtx = 0 as *mut tt_longMetrics
    }
    sfnt_locate_table(sfont, b"loca\x00" as *const u8 as *const i8);
    location = new(
        (((*maxp).numGlyphs as i32 + 1i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<SFNT_ULONG>() as u64) as u32,
    ) as *mut SFNT_ULONG;
    if (*head).indexToLocFormat as i32 == 0i32 {
        i = 0i32 as u32;
        while i <= (*maxp).numGlyphs as u32 {
            *location.offset(i as isize) = (2i32 as u32)
                .wrapping_mul(tt_get_unsigned_pair((*sfont).handle) as SFNT_ULONG);
            i = i.wrapping_add(1)
        }
    } else if (*head).indexToLocFormat as i32 == 1i32 {
        i = 0i32 as u32;
        while i <= (*maxp).numGlyphs as u32 {
            *location.offset(i as isize) = tt_get_unsigned_quad((*sfont).handle);
            i = i.wrapping_add(1)
        }
    } else {
        _tt_abort(b"Unknown IndexToLocFormat.\x00" as *const u8 as *const i8);
    }
    w_stat = new(
        (((*g).emsize as i32 + 2i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<USHORT>() as u64) as u32,
    ) as *mut USHORT;
    memset(
        w_stat as *mut libc::c_void,
        0i32,
        (::std::mem::size_of::<USHORT>() as u64)
            .wrapping_mul(((*g).emsize as i32 + 2i32) as u64),
    );
    /*
     * Read glyf table.
     */
    offset = sfnt_locate_table(sfont, b"glyf\x00" as *const u8 as *const i8); /* old gid */
    i = 0i32 as u32;
    while i < (*g).num_glyphs as u32 {
        let mut gid: USHORT = 0;
        let mut loc: SFNT_ULONG = 0;
        let mut len: SFNT_ULONG = 0;
        gid = (*(*g).gd.offset(i as isize)).ogid;
        if gid as i32 >= (*maxp).numGlyphs as i32 {
            _tt_abort(
                b"Invalid glyph index (gid %u)\x00" as *const u8 as *const i8,
                gid as i32,
            );
        }
        loc = *location.offset(gid as isize);
        len = (*location.offset((gid as i32 + 1i32) as isize)).wrapping_sub(loc);
        (*(*g).gd.offset(i as isize)).advw = (*hmtx.offset(gid as isize)).advance;
        (*(*g).gd.offset(i as isize)).lsb = (*hmtx.offset(gid as isize)).sideBearing;
        if !vmtx.is_null() {
            (*(*g).gd.offset(i as isize)).advh = (*vmtx.offset(gid as isize)).advance;
            (*(*g).gd.offset(i as isize)).tsb = (*vmtx.offset(gid as isize)).sideBearing
        } else {
            (*(*g).gd.offset(i as isize)).advh = (*g).default_advh;
            (*(*g).gd.offset(i as isize)).tsb = (*g).default_tsb
        }
        (*(*g).gd.offset(i as isize)).length = len;
        let ref mut fresh7 = (*(*g).gd.offset(i as isize)).data;
        *fresh7 = 0 as *mut BYTE;
        if (*(*g).gd.offset(i as isize)).advw as i32 <= (*g).emsize as i32 {
            let ref mut fresh8 = *w_stat.offset((*(*g).gd.offset(i as isize)).advw as isize);
            *fresh8 = (*fresh8 as i32 + 1i32) as USHORT
        } else {
            let ref mut fresh9 = *w_stat.offset(((*g).emsize as i32 + 1i32) as isize);
            *fresh9 = (*fresh9 as i32 + 1i32) as USHORT
            /* larger than em */
        }
        if !(len == 0i32 as u32) {
            if len < 10i32 as u32 {
                _tt_abort(
                    b"Invalid TrueType glyph data (gid %u).\x00" as *const u8
                        as *const i8,
                    gid as i32,
                );
            }
            ttstub_input_seek((*sfont).handle, offset.wrapping_add(loc) as ssize_t, 0i32);
            tt_get_signed_pair((*sfont).handle);
            /* BoundingBox: FWord x 4 */
            (*(*g).gd.offset(i as isize)).llx = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).lly = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).urx = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).ury = tt_get_signed_pair((*sfont).handle);
            /* _FIXME_ */
            if vmtx.is_null() {
                /* vertOriginY == sTypeAscender */
                (*(*g).gd.offset(i as isize)).tsb = ((*g).default_advh as i32
                    - (*g).default_tsb as i32
                    - (*(*g).gd.offset(i as isize)).ury as i32)
                    as SHORT
            }
        }
        /* Does not contains any data. */
        i = i.wrapping_add(1)
    }
    free(location as *mut libc::c_void);
    free(hmtx as *mut libc::c_void);
    free(maxp as *mut libc::c_void);
    free(hhea as *mut libc::c_void);
    free(head as *mut libc::c_void);
    free(os2 as *mut libc::c_void);
    free(vmtx as *mut libc::c_void);
    let mut max_count: i32 = -1i32;
    (*g).dw = (*(*g).gd.offset(0)).advw;
    i = 0i32 as u32;
    while i < ((*g).emsize as i32 + 1i32) as u32 {
        if *w_stat.offset(i as isize) as i32 > max_count {
            max_count = *w_stat.offset(i as isize) as i32;
            (*g).dw = i as USHORT
        }
        i = i.wrapping_add(1)
    }
    free(w_stat as *mut libc::c_void);
    return 0i32;
}
