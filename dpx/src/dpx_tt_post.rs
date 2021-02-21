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

use super::dpx_numbers::GetFromFile;
use super::dpx_sfnt::sfnt_locate_table;
use crate::warn;
use std::ffi::CStr;
use std::io::Read;

use super::dpx_mem::new;
use libc::free;

use std::ptr;

pub(crate) type Fixed = u32;
pub(crate) type FWord = i16;

use super::dpx_sfnt::sfnt;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct tt_post_table {
    pub(crate) Version: Fixed,
    pub(crate) italicAngle: Fixed,
    pub(crate) underlinePosition: FWord,
    pub(crate) underlineThickness: FWord,
    pub(crate) isFixedPitch: u32,
    pub(crate) minMemType42: u32,
    pub(crate) maxMemType42: u32,
    pub(crate) minMemType1: u32,
    pub(crate) maxMemType1: u32,
    pub(crate) numberOfGlyphs: u16,
    pub(crate) glyphNamePtr: *mut *const i8,
    pub(crate) names: *mut *mut i8,
    pub(crate) count: u16,
}

/* offset from begenning of the post table */
unsafe fn read_v2_post_names<R: Read>(mut post: *mut tt_post_table, handle: &mut R) -> i32 {
    (*post).numberOfGlyphs = u16::get(handle);
    let indices = new(((*post).numberOfGlyphs as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<u16>() as u64) as u32) as *mut u16;
    let mut maxidx = 257_u16;
    for i in 0..(*post).numberOfGlyphs as i32 {
        let mut idx = u16::get(handle);
        if idx as i32 >= 258 {
            if idx as i32 > maxidx as i32 {
                maxidx = idx
            }
            if idx as i32 > 32767 {
                /* Although this is strictly speaking out of spec, it seems to work
                and there are real-life fonts that use it.
                We show a warning only once, instead of thousands of times */
                static mut warning_issued: i8 = 0_i8;
                if warning_issued == 0 {
                    warn!("TrueType post table name index {} > 32767", idx);
                    warning_issued = 1_i8
                }
                /* In a real-life large font, (x)dvipdfmx crashes if we use
                nonvanishing idx in the case of idx > 32767.
                If we set idx = 0, (x)dvipdfmx works fine for the font and
                created pdf seems fine. The post table may not be important
                in such a case */
                idx = 0_u16
            }
        }
        *indices.offset(i as isize) = idx;
    }
    (*post).count = (maxidx as i32 - 257) as u16;
    if ((*post).count as i32) < 1 {
        (*post).names = 0 as *mut *mut i8
    } else {
        (*post).names = new(((*post).count as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<*mut i8>() as u64)
            as u32) as *mut *mut i8;
        for i in 0..(*post).count as i32 {
            /* read Pascal strings */
            let len = u8::get(handle) as i32;
            if len > 0 {
                *(*post).names.offset(i as isize) = new(((len + 1) as u32 as u64)
                    .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                    as u32) as *mut i8;
                let slice = std::slice::from_raw_parts_mut(
                    (*(*post).names.offset(i as isize)) as *mut u8,
                    len as usize,
                );
                handle.read(slice).unwrap();
                *(*(*post).names.offset(i as isize)).offset(len as isize) = 0_i8
            } else {
                *(*post).names.offset(i as isize) = ptr::null_mut();
            }
        }
    }
    (*post).glyphNamePtr = new(((*post).numberOfGlyphs as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<*const i8>() as u64)
        as u32) as *mut *const i8;
    for i in 0..(*post).numberOfGlyphs as i32 {
        let idx = *indices.offset(i as isize);
        if (idx as i32) < 258 {
            *(*post).glyphNamePtr.offset(i as isize) =
                macglyphorder[idx as usize].as_ptr() as *const i8
        } else if idx as i32 - 258 < (*post).count as i32 {
            *(*post).glyphNamePtr.offset(i as isize) =
                *(*post).names.offset((idx as i32 - 258) as isize)
        } else {
            warn!(
                "Invalid glyph name index number: {} (>= {})",
                idx,
                (*post).count + 258,
            );
            free(indices as *mut libc::c_void);
            return -1;
        }
    }
    free(indices as *mut libc::c_void);
    0
}

pub(crate) unsafe fn tt_read_post_table(sfont: &sfnt) -> *mut tt_post_table {
    /* offset = */
    sfnt_locate_table(sfont, b"post"); /* Fixed */
    let mut post = new((1_u64).wrapping_mul(::std::mem::size_of::<tt_post_table>() as u64) as u32)
        as *mut tt_post_table; /* Fixed */
    let handle = &mut &*sfont.handle;
    (*post).Version = u32::get(handle); /* FWord */
    (*post).italicAngle = u32::get(handle); /* FWord */
    (*post).underlinePosition = i16::get(handle); /* wrong */
    (*post).underlineThickness = i16::get(handle);
    (*post).isFixedPitch = u32::get(handle);
    (*post).minMemType42 = u32::get(handle);
    (*post).maxMemType42 = u32::get(handle);
    (*post).minMemType1 = u32::get(handle);
    (*post).maxMemType1 = u32::get(handle);
    (*post).numberOfGlyphs = 0_u16;
    (*post).glyphNamePtr = 0 as *mut *const i8;
    (*post).count = 0_u16;
    (*post).names = 0 as *mut *mut i8;
    if (*post).Version as u64 == 0x10000 {
        (*post).numberOfGlyphs = 258_u16;
        (*post).glyphNamePtr = macglyphorder.as_mut_ptr() as *mut *const u8 as *mut *const i8
    } else if (*post).Version as u64 == 0x28000 {
        warn!("TrueType \'post\' version 2.5 found (deprecated)");
    } else if (*post).Version as u64 == 0x20000 {
        if read_v2_post_names(post, handle) < 0 {
            warn!("Invalid version 2.0 \'post\' table");
            tt_release_post_table(post);
            post = ptr::null_mut()
        }
    } else if !((*post).Version as u64 == 0x30000 || (*post).Version as u64 == 0x40000) {
        warn!(
            "Unknown \'post\' version: {:08X}, assuming version 3.0",
            (*post).Version,
        );
    }
    post
}

pub(crate) unsafe fn tt_lookup_post_table(post: *mut tt_post_table, glyphname: &str) -> u16 {
    assert!(!post.is_null() && !glyphname.is_empty());
    for gid in 0..(*post).count as u16 {
        if !(*(*post).glyphNamePtr.offset(gid as isize)).is_null()
            && glyphname.as_bytes()
                == CStr::from_ptr(*(*post).glyphNamePtr.offset(gid as isize)).to_bytes()
        {
            return gid;
        }
    }
    0
}

pub(crate) unsafe fn tt_get_glyphname(post: *mut tt_post_table, gid: u16) -> String {
    if (gid as i32) < (*post).count as i32
        && !(*(*post).glyphNamePtr.offset(gid as isize)).is_null()
    {
        return CStr::from_ptr(*(*post).glyphNamePtr.offset(gid as isize))
            .to_str()
            .unwrap()
            .to_string();
    }
    String::new()
}
/* Glyph names (pointer to C string) */
/* Non-standard glyph names */
/* Number of glyph names in names[] */

pub(crate) unsafe fn tt_release_post_table(mut post: *mut tt_post_table) {
    assert!(!post.is_null());
    if !(*post).glyphNamePtr.is_null() && (*post).Version as u64 != 0x10000 {
        free((*post).glyphNamePtr as *mut libc::c_void);
    }
    if !(*post).names.is_null() {
        for i in 0..(*post).count {
            free(*(*post).names.offset(i as isize) as *mut libc::c_void);
        }
        free((*post).names as *mut libc::c_void);
    }
    (*post).count = 0_u16;
    (*post).glyphNamePtr = 0 as *mut *const i8;
    (*post).names = 0 as *mut *mut i8;
    free(post as *mut libc::c_void);
}
/* Macintosh glyph order - from apple's TTRefMan */
static mut macglyphorder: [&[u8]; 258] = [
    b".notdef\x00",
    b".null\x00",
    b"nonmarkingreturn\x00",
    b"space\x00",
    b"exclam\x00",
    b"quotedbl\x00",
    b"numbersign\x00",
    b"dollar\x00",
    b"percent\x00",
    b"ampersand\x00",
    b"quotesingle\x00",
    b"parenleft\x00",
    b"parenright\x00",
    b"asterisk\x00",
    b"plus\x00",
    b"comma\x00",
    b"hyphen\x00",
    b"period\x00",
    b"slash\x00",
    b"zero\x00",
    b"one\x00",
    b"two\x00",
    b"three\x00",
    b"four\x00",
    b"five\x00",
    b"six\x00",
    b"seven\x00",
    b"eight\x00",
    b"nine\x00",
    b"colon\x00",
    b"semicolon\x00",
    b"less\x00",
    b"equal\x00",
    b"greater\x00",
    b"question\x00",
    b"at\x00",
    b"A\x00",
    b"B\x00",
    b"C\x00",
    b"D\x00",
    b"E\x00",
    b"F\x00",
    b"G\x00",
    b"H\x00",
    b"I\x00",
    b"J\x00",
    b"K\x00",
    b"L\x00",
    b"M\x00",
    b"N\x00",
    b"O\x00",
    b"P\x00",
    b"Q\x00",
    b"R\x00",
    b"S\x00",
    b"T\x00",
    b"U\x00",
    b"V\x00",
    b"W\x00",
    b"X\x00",
    b"Y\x00",
    b"Z\x00",
    b"bracketleft\x00",
    b"backslash\x00",
    b"bracketright\x00",
    b"asciicircum\x00",
    b"underscore\x00",
    b"grave\x00",
    b"a\x00",
    b"b\x00",
    b"c\x00",
    b"d\x00",
    b"e\x00",
    b"f\x00",
    b"g\x00",
    b"h\x00",
    b"i\x00",
    b"j\x00",
    b"k\x00",
    b"l\x00",
    b"m\x00",
    b"n\x00",
    b"o\x00",
    b"p\x00",
    b"q\x00",
    b"r\x00",
    b"s\x00",
    b"t\x00",
    b"u\x00",
    b"v\x00",
    b"w\x00",
    b"x\x00",
    b"y\x00",
    b"z\x00",
    b"braceleft\x00",
    b"bar\x00",
    b"braceright\x00",
    b"asciitilde\x00",
    b"Adieresis\x00",
    b"Aring\x00",
    b"Ccedilla\x00",
    b"Eacute\x00",
    b"Ntilde\x00",
    b"Odieresis\x00",
    b"Udieresis\x00",
    b"aacute\x00",
    b"agrave\x00",
    b"acircumflex\x00",
    b"adieresis\x00",
    b"atilde\x00",
    b"aring\x00",
    b"ccedilla\x00",
    b"eacute\x00",
    b"egrave\x00",
    b"ecircumflex\x00",
    b"edieresis\x00",
    b"iacute\x00",
    b"igrave\x00",
    b"icircumflex\x00",
    b"idieresis\x00",
    b"ntilde\x00",
    b"oacute\x00",
    b"ograve\x00",
    b"ocircumflex\x00",
    b"odieresis\x00",
    b"otilde\x00",
    b"uacute\x00",
    b"ugrave\x00",
    b"ucircumflex\x00",
    b"udieresis\x00",
    b"dagger\x00",
    b"degree\x00",
    b"cent\x00",
    b"sterling\x00",
    b"section\x00",
    b"bullet\x00",
    b"paragraph\x00",
    b"germandbls\x00",
    b"registered\x00",
    b"copyright\x00",
    b"trademark\x00",
    b"acute\x00",
    b"dieresis\x00",
    b"notequal\x00",
    b"AE\x00",
    b"Oslash\x00",
    b"infinity\x00",
    b"plusminus\x00",
    b"lessequal\x00",
    b"greaterequal\x00",
    b"yen\x00",
    b"mu\x00",
    b"partialdiff\x00",
    b"summation\x00",
    b"product\x00",
    b"pi\x00",
    b"integral\x00",
    b"ordfeminine\x00",
    b"ordmasculine\x00",
    b"Omega\x00",
    b"ae\x00",
    b"oslash\x00",
    b"questiondown\x00",
    b"exclamdown\x00",
    b"logicalnot\x00",
    b"radical\x00",
    b"florin\x00",
    b"approxequal\x00",
    b"Delta\x00",
    b"guillemotleft\x00",
    b"guillemotright\x00",
    b"ellipsis\x00",
    b"nonbreakingspace\x00",
    b"Agrave\x00",
    b"Atilde\x00",
    b"Otilde\x00",
    b"OE\x00",
    b"oe\x00",
    b"endash\x00",
    b"emdash\x00",
    b"quotedblleft\x00",
    b"quotedblright\x00",
    b"quoteleft\x00",
    b"quoteright\x00",
    b"divide\x00",
    b"lozenge\x00",
    b"ydieresis\x00",
    b"Ydieresis\x00",
    b"fraction\x00",
    b"currency\x00",
    b"guilsinglleft\x00",
    b"guilsinglright\x00",
    b"fi\x00",
    b"fl\x00",
    b"daggerdbl\x00",
    b"periodcentered\x00",
    b"quotesinglbase\x00",
    b"quotedblbase\x00",
    b"perthousand\x00",
    b"Acircumflex\x00",
    b"Ecircumflex\x00",
    b"Aacute\x00",
    b"Edieresis\x00",
    b"Egrave\x00",
    b"Iacute\x00",
    b"Icircumflex\x00",
    b"Idieresis\x00",
    b"Igrave\x00",
    b"Oacute\x00",
    b"Ocircumflex\x00",
    b"apple\x00",
    b"Ograve\x00",
    b"Uacute\x00",
    b"Ucircumflex\x00",
    b"Ugrave\x00",
    b"dotlessi\x00",
    b"circumflex\x00",
    b"tilde\x00",
    b"macron\x00",
    b"breve\x00",
    b"dotaccent\x00",
    b"ring\x00",
    b"cedilla\x00",
    b"hungarumlaut\x00",
    b"ogonek\x00",
    b"caron\x00",
    b"Lslash\x00",
    b"lslash\x00",
    b"Scaron\x00",
    b"scaron\x00",
    b"Zcaron\x00",
    b"zcaron\x00",
    b"brokenbar\x00",
    b"Eth\x00",
    b"eth\x00",
    b"Yacute\x00",
    b"yacute\x00",
    b"Thorn\x00",
    b"thorn\x00",
    b"minus\x00",
    b"multiply\x00",
    b"onesuperior\x00",
    b"twosuperior\x00",
    b"threesuperior\x00",
    b"onehalf\x00",
    b"onequarter\x00",
    b"threequarters\x00",
    b"franc\x00",
    b"Gbreve\x00",
    b"gbreve\x00",
    b"Idotaccent\x00",
    b"Scedilla\x00",
    b"scedilla\x00",
    b"Cacute\x00",
    b"cacute\x00",
    b"Ccaron\x00",
    b"ccaron\x00",
    b"dcroat\x00",
];
