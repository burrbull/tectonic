#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    pub type pdf_obj;
    pub type pdf_font;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn pdf_font_set_subtype(font: *mut pdf_font, subtype: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pdf_font_set_flags(font: *mut pdf_font, flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pdf_font_set_fontname(font: *mut pdf_font, fontname: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn pdf_font_get_encoding(font: *mut pdf_font) -> libc::c_int;
    #[no_mangle]
    fn pdf_font_get_usedchars(font: *mut pdf_font) -> *mut libc::c_char;
    #[no_mangle]
    fn pdf_font_get_descriptor(font: *mut pdf_font) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_font_get_resource(font: *mut pdf_font) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_font_get_uniqueTag(font: *mut pdf_font) -> *mut libc::c_char;
    #[no_mangle]
    fn pdf_font_get_fontname(font: *mut pdf_font) -> *mut libc::c_char;
    #[no_mangle]
    fn pdf_font_get_mapname(font: *mut pdf_font) -> *mut libc::c_char;
    #[no_mangle]
    fn pdf_font_get_ident(font: *mut pdf_font) -> *mut libc::c_char;
    #[no_mangle]
    fn pdf_font_is_in_use(font: *mut pdf_font) -> bool;
    #[no_mangle]
    fn pdf_font_get_verbose() -> libc::c_int;
    #[no_mangle]
    fn pdf_stream_dataptr(stream: *mut pdf_obj) -> *const libc::c_void;
    #[no_mangle]
    fn pdf_stream_length(stream: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_stream(
        stream: *mut pdf_obj,
        stream_data_ptr: *const libc::c_void,
        stream_data_len: libc::c_int,
    );
    #[no_mangle]
    fn pdf_new_stream(flags: libc::c_int) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_array_length(array: *mut pdf_obj) -> libc::c_uint;
    #[no_mangle]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_new_array() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_name(name: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_string(str: *const libc::c_void, length: size_t) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const libc::c_char,
        format: tt_input_format_type,
        is_gz: libc::c_int,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn cff_close(cff: *mut cff_font);
    #[no_mangle]
    fn cff_release_index(idx: *mut cff_index);
    #[no_mangle]
    fn cff_index_size(idx: *mut cff_index) -> libc::c_int;
    #[no_mangle]
    fn cff_pack_index(idx: *mut cff_index, dest: *mut card8, destlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cff_pack_charsets(cff: *mut cff_font, dest: *mut card8, destlen: libc::c_int)
        -> libc::c_int;
    #[no_mangle]
    fn cff_pack_encoding(cff: *mut cff_font, dest: *mut card8, destlen: libc::c_int)
        -> libc::c_int;
    #[no_mangle]
    fn cff_put_header(cff: *mut cff_font, dest: *mut card8, destlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cff_glyph_lookup(cff: *mut cff_font, glyph: *const libc::c_char) -> card16;
    #[no_mangle]
    fn cff_release_charsets(charset: *mut cff_charsets);
    #[no_mangle]
    fn cff_new_index(count: card16) -> *mut cff_index;
    #[no_mangle]
    fn cff_set_name(cff: *mut cff_font, name: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn cff_get_seac_sid(cff: *mut cff_font, str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn cff_add_string(cff: *mut cff_font, str: *const libc::c_char, unique: libc::c_int) -> s_SID;
    #[no_mangle]
    fn cff_update_string(cff: *mut cff_font);
    #[no_mangle]
    fn cff_dict_set(
        dict: *mut cff_dict,
        key: *const libc::c_char,
        idx: libc::c_int,
        value: libc::c_double,
    );
    #[no_mangle]
    fn cff_dict_get(
        dict: *mut cff_dict,
        key: *const libc::c_char,
        idx: libc::c_int,
    ) -> libc::c_double;
    #[no_mangle]
    fn cff_dict_add(dict: *mut cff_dict, key: *const libc::c_char, count: libc::c_int);
    #[no_mangle]
    fn cff_dict_known(dict: *mut cff_dict, key: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn cff_dict_pack(dict: *mut cff_dict, dest: *mut card8, destlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cff_dict_update(dict: *mut cff_dict, cff: *mut cff_font);
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
    fn dpx_message(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
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
    #[no_mangle]
    fn pdf_encoding_get_encoding(enc_id: libc::c_int) -> *mut *mut libc::c_char;
    /*
     * pdf_create_ToUnicode_CMap() returns stream object but not
     * reference. This need to be renamed to other name like
     * pdf_create_ToUnicode_stream().
     */
    #[no_mangle]
    fn pdf_create_ToUnicode_CMap(
        enc_name: *const libc::c_char,
        enc_vec: *mut *mut libc::c_char,
        is_used: *const libc::c_char,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn t1char_get_metrics(
        src: *mut card8,
        srclen: libc::c_int,
        subrs: *mut cff_index,
        ginfo: *mut t1_ginfo,
    ) -> libc::c_int;
    #[no_mangle]
    fn t1char_convert_charstring(
        dst: *mut card8,
        dstlen: libc::c_int,
        src: *mut card8,
        srclen: libc::c_int,
        subrs: *mut cff_index,
        default_width: libc::c_double,
        nominal_width: libc::c_double,
        ginfo: *mut t1_ginfo,
    ) -> libc::c_int;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 2012-2015 by Khaled Hosny <khaledhosny@eglug.org>

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
    fn t1_load_font(
        enc_vec: *mut *mut libc::c_char,
        mode: libc::c_int,
        handle: rust_input_handle_t,
    ) -> *mut cff_font;
    #[no_mangle]
    fn is_pfb(handle: rust_input_handle_t) -> bool;
    #[no_mangle]
    fn t1_get_fontname(handle: rust_input_handle_t, fontname: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn t1_get_standard_glyph(code: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn tfm_open(tex_name: *const libc::c_char, must_exist: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tfm_get_width(font_id: libc::c_int, ch: int32_t) -> libc::c_double;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = libc::c_ulong;
/* The weird enum values are historical and could be rationalized. But it is
 * good to write them explicitly since they must be kept in sync with
 * `src/engines/mod.rs`.
 */
pub type tt_input_format_type = libc::c_uint;
pub const TTIF_TECTONIC_PRIMARY: tt_input_format_type = 59;
pub const TTIF_OPENTYPE: tt_input_format_type = 47;
pub const TTIF_SFD: tt_input_format_type = 46;
pub const TTIF_CMAP: tt_input_format_type = 45;
pub const TTIF_ENC: tt_input_format_type = 44;
pub const TTIF_MISCFONTS: tt_input_format_type = 41;
pub const TTIF_BINARY: tt_input_format_type = 40;
pub const TTIF_TRUETYPE: tt_input_format_type = 36;
pub const TTIF_VF: tt_input_format_type = 33;
pub const TTIF_TYPE1: tt_input_format_type = 32;
pub const TTIF_TEX_PS_HEADER: tt_input_format_type = 30;
pub const TTIF_TEX: tt_input_format_type = 26;
pub const TTIF_PICT: tt_input_format_type = 25;
pub const TTIF_OVF: tt_input_format_type = 23;
pub const TTIF_OFM: tt_input_format_type = 20;
pub const TTIF_FONTMAP: tt_input_format_type = 11;
pub const TTIF_FORMAT: tt_input_format_type = 10;
pub const TTIF_CNF: tt_input_format_type = 8;
pub const TTIF_BST: tt_input_format_type = 7;
pub const TTIF_BIB: tt_input_format_type = 6;
pub const TTIF_AFM: tt_input_format_type = 4;
pub const TTIF_TFM: tt_input_format_type = 3;
pub type rust_input_handle_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_index {
    pub count: card16,
    pub offsize: c_offsize,
    pub offset: *mut l_offset,
    pub data: *mut card8,
}
/* quasi-hack to get the primary input */
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
/* CFF Data Types */
/* SID SID number */
/* offset(0) */
/* size offset(0) */
pub type card8 = libc::c_uchar;
/* 1-byte unsigned number specifies the size
of an Offset field or fields, range 1-4 */
pub type l_offset = u32;
/* 2-byte unsigned number */
pub type c_offsize = libc::c_uchar;
/* 1-byte unsigned number */
pub type card16 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_font {
    pub fontname: *mut libc::c_char,
    pub header: cff_header,
    pub name: *mut cff_index,
    pub topdict: *mut cff_dict,
    pub string: *mut cff_index,
    pub gsubr: *mut cff_index,
    pub encoding: *mut cff_encoding,
    pub charsets: *mut cff_charsets,
    pub fdselect: *mut cff_fdselect,
    pub cstrings: *mut cff_index,
    pub fdarray: *mut *mut cff_dict,
    pub private: *mut *mut cff_dict,
    pub subrs: *mut *mut cff_index,
    pub offset: l_offset,
    pub gsubr_offset: l_offset,
    pub num_glyphs: card16,
    pub num_fds: card8,
    pub _string: *mut cff_index,
    pub handle: rust_input_handle_t,
    pub filter: libc::c_int,
    pub index: libc::c_int,
    pub flag: libc::c_int,
    pub is_notdef_notzero: libc::c_int,
}
/* format major version (starting at 1) */
/* format minor version (starting at 0) */
/* Header size (bytes)                  */
/* Absolute offset (0) size             */
/* Dictionary */
/* encoded data value (as card8 or card16) */
/* opname                                 */
/* number of values                        */
/* values                                  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict {
    pub max: libc::c_int,
    pub count: libc::c_int,
    pub entries: *mut cff_dict_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict_entry {
    pub id: libc::c_int,
    pub key: *const libc::c_char,
    pub count: libc::c_int,
    pub values: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_fdselect {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub fds: *mut card8,
    pub ranges: *mut cff_range3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range3 {
    pub first: card16,
    pub fd: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_charsets {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub glyphs: *mut s_SID,
    pub range1: *mut cff_range1,
    pub range2: *mut cff_range2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range2 {
    pub first: s_SID,
    pub n_left: card16,
}
/* 1, 2, 3, or 4-byte offset */
pub type s_SID = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range1 {
    pub first: s_SID,
    pub n_left: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_encoding {
    pub format: card8,
    pub num_entries: card8,
    pub data: C2RustUnnamed_1,
    pub num_supps: card8,
    pub supp: *mut cff_map,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_map {
    pub code: card8,
    pub glyph: s_SID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub codes: *mut card8,
    pub range1: *mut cff_range1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_header {
    pub major: card8,
    pub minor: card8,
    pub hdr_size: card8,
    pub offsize: c_offsize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t1_ginfo {
    pub use_seac: libc::c_int,
    pub wx: libc::c_double,
    pub wy: libc::c_double,
    pub bbox: C2RustUnnamed_3,
    pub seac: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub asb: libc::c_double,
    pub adx: libc::c_double,
    pub ady: libc::c_double,
    pub bchar: card8,
    pub achar: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub llx: libc::c_double,
    pub lly: libc::c_double,
    pub urx: libc::c_double,
    pub ury: libc::c_double,
}
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    return 0 as *mut libc::c_void;
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const libc::c_char, mut s2: *const libc::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    return 0i32 != 0;
}
/* Force bold at small text sizes */
unsafe extern "C" fn is_basefont(mut name: *const libc::c_char) -> bool {
    static mut basefonts: [*const libc::c_char; 14] = [
        b"Courier\x00" as *const u8 as *const libc::c_char,
        b"Courier-Bold\x00" as *const u8 as *const libc::c_char,
        b"Courier-Oblique\x00" as *const u8 as *const libc::c_char,
        b"Courier-BoldOblique\x00" as *const u8 as *const libc::c_char,
        b"Helvetica\x00" as *const u8 as *const libc::c_char,
        b"Helvetica-Bold\x00" as *const u8 as *const libc::c_char,
        b"Helvetica-Oblique\x00" as *const u8 as *const libc::c_char,
        b"Helvetica-BoldOblique\x00" as *const u8 as *const libc::c_char,
        b"Symbol\x00" as *const u8 as *const libc::c_char,
        b"Times-Roman\x00" as *const u8 as *const libc::c_char,
        b"Times-Bold\x00" as *const u8 as *const libc::c_char,
        b"Times-Italic\x00" as *const u8 as *const libc::c_char,
        b"Times-BoldItalic\x00" as *const u8 as *const libc::c_char,
        b"ZapfDingbats\x00" as *const u8 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 14i32 {
        if streq_ptr(name, basefonts[i as usize]) {
            return 1i32 != 0;
        }
        i += 1
    }
    return 0i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_open_type1(mut font: *mut pdf_font) -> libc::c_int {
    let mut ident: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fontname: [libc::c_char; 128] = [0; 128];
    if !font.is_null() {
    } else {
        __assert_fail(
            b"font\x00" as *const u8 as *const libc::c_char,
            b"dpx-type1.c\x00" as *const u8 as *const libc::c_char,
            85i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                b"int pdf_font_open_type1(pdf_font *)\x00",
            ))
            .as_ptr(),
        );
    }
    ident = pdf_font_get_ident(font);
    if is_basefont(ident) {
        pdf_font_set_fontname(font, ident);
        pdf_font_set_subtype(font, 0i32);
        pdf_font_set_flags(font, 1i32 << 0i32 | 1i32 << 2i32);
    } else {
        let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
        handle = ttstub_input_open(ident, TTIF_TYPE1, 0i32);
        /* NOTE: skipping qcheck_filetype() call in dpx_find_type1_file but we
         * call is_pfb() in just a second anyway.
         */
        if handle.is_null() {
            return -1i32;
        }
        memset(
            fontname.as_mut_ptr() as *mut libc::c_void,
            0i32,
            (127i32 + 1i32) as libc::c_ulong,
        );
        if !is_pfb(handle) || t1_get_fontname(handle, fontname.as_mut_ptr()) < 0i32 {
            _tt_abort(
                b"Failed to read Type 1 font \"%s\".\x00" as *const u8 as *const libc::c_char,
                ident,
            );
        }
        ttstub_input_close(handle);
        pdf_font_set_fontname(font, fontname.as_mut_ptr());
        pdf_font_set_subtype(font, 0i32);
    }
    return 0i32;
}
unsafe extern "C" fn get_font_attr(mut font: *mut pdf_font, mut cffont: *mut cff_font) {
    let mut fontname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut descriptor: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut capheight: libc::c_double = 0.;
    let mut ascent: libc::c_double = 0.;
    let mut descent: libc::c_double = 0.;
    let mut italicangle: libc::c_double = 0.;
    let mut stemv: libc::c_double = 0.;
    let mut defaultwidth: libc::c_double = 0.;
    let mut nominalwidth: libc::c_double = 0.;
    let mut flags: libc::c_int = 0i32;
    let mut gid: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    static mut L_c: [*const libc::c_char; 5] = [
        b"H\x00" as *const u8 as *const libc::c_char,
        b"P\x00" as *const u8 as *const libc::c_char,
        b"Pi\x00" as *const u8 as *const libc::c_char,
        b"Rho\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut L_d: [*const libc::c_char; 5] = [
        b"p\x00" as *const u8 as *const libc::c_char,
        b"q\x00" as *const u8 as *const libc::c_char,
        b"mu\x00" as *const u8 as *const libc::c_char,
        b"eta\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut L_a: [*const libc::c_char; 4] = [
        b"b\x00" as *const u8 as *const libc::c_char,
        b"h\x00" as *const u8 as *const libc::c_char,
        b"lambda\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut gm: t1_ginfo = t1_ginfo {
        use_seac: 0,
        wx: 0.,
        wy: 0.,
        bbox: C2RustUnnamed_3 {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        seac: C2RustUnnamed_2 {
            asb: 0.,
            adx: 0.,
            ady: 0.,
            bchar: 0,
            achar: 0,
        },
    };
    defaultwidth = 500.0f64;
    nominalwidth = 0.0f64;
    /*
     * CapHeight, Ascent, and Descent is meaningfull only for Latin/Greek/Cyrillic.
     * The BlueValues and OtherBlues also have those information.
     */
    if cff_dict_known(
        (*cffont).topdict,
        b"FontBBox\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        /* Default values */
        ascent = cff_dict_get(
            (*cffont).topdict,
            b"FontBBox\x00" as *const u8 as *const libc::c_char,
            3i32,
        );
        capheight = ascent;
        descent = cff_dict_get(
            (*cffont).topdict,
            b"FontBBox\x00" as *const u8 as *const libc::c_char,
            1i32,
        )
    } else {
        capheight = 680.0f64;
        ascent = 690.0f64;
        descent = -190.0f64
    }
    if cff_dict_known(
        *(*cffont).private.offset(0),
        b"StdVW\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        stemv = cff_dict_get(
            *(*cffont).private.offset(0),
            b"StdVW\x00" as *const u8 as *const libc::c_char,
            0i32,
        )
    } else {
        /*
         * We may use the following values for StemV:
         *  Thin - ExtraLight: <= 50
         *  Light: 71
         *  Regular(Normal): 88
         *  Medium: 109
         *  SemiBold(DemiBold): 135
         *  Bold - Heavy: >= 166
         */
        stemv = 88.0f64
    }
    if cff_dict_known(
        (*cffont).topdict,
        b"ItalicAngle\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        italicangle = cff_dict_get(
            (*cffont).topdict,
            b"ItalicAngle\x00" as *const u8 as *const libc::c_char,
            0i32,
        );
        if italicangle != 0.0f64 {
            flags |= 1i32 << 6i32
        }
    } else {
        italicangle = 0.0f64
    }
    /*
     * Use "space", "H", "p", and "b" for various values.
     * Those characters should not "seac". (no accent)
     */
    gid = cff_glyph_lookup(cffont, b"space\x00" as *const u8 as *const libc::c_char) as libc::c_int; /* FIXME */
    if gid >= 0i32 && gid < (*(*cffont).cstrings).count as libc::c_int {
        t1char_get_metrics(
            (*(*cffont).cstrings)
                .data
                .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                .offset(-1),
            (*(*(*cffont).cstrings).offset.offset((gid + 1i32) as isize))
                .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize))
                as libc::c_int,
            *(*cffont).subrs.offset(0),
            &mut gm,
        );
        defaultwidth = gm.wx
    }
    i = 0i32;
    while !L_c[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_c[i as usize]) as libc::c_int;
        if gid >= 0i32 && gid < (*(*cffont).cstrings).count as libc::c_int {
            t1char_get_metrics(
                (*(*cffont).cstrings)
                    .data
                    .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                    .offset(-1),
                (*(*(*cffont).cstrings).offset.offset((gid + 1i32) as isize))
                    .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize))
                    as libc::c_int,
                *(*cffont).subrs.offset(0),
                &mut gm,
            );
            capheight = gm.bbox.ury;
            break;
        } else {
            i += 1
        }
    }
    i = 0i32;
    while !L_d[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_d[i as usize]) as libc::c_int;
        if gid >= 0i32 && gid < (*(*cffont).cstrings).count as libc::c_int {
            t1char_get_metrics(
                (*(*cffont).cstrings)
                    .data
                    .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                    .offset(-1),
                (*(*(*cffont).cstrings).offset.offset((gid + 1i32) as isize))
                    .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize))
                    as libc::c_int,
                *(*cffont).subrs.offset(0),
                &mut gm,
            );
            descent = gm.bbox.lly;
            break;
        } else {
            i += 1
        }
    }
    i = 0i32;
    while !L_a[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_a[i as usize]) as libc::c_int;
        if gid >= 0i32 && gid < (*(*cffont).cstrings).count as libc::c_int {
            t1char_get_metrics(
                (*(*cffont).cstrings)
                    .data
                    .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                    .offset(-1),
                (*(*(*cffont).cstrings).offset.offset((gid + 1i32) as isize))
                    .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize))
                    as libc::c_int,
                *(*cffont).subrs.offset(0),
                &mut gm,
            );
            ascent = gm.bbox.ury;
            break;
        } else {
            i += 1
        }
    }
    if defaultwidth != 0.0f64 {
        cff_dict_add(
            *(*cffont).private.offset(0),
            b"defaultWidthX\x00" as *const u8 as *const libc::c_char,
            1i32,
        );
        cff_dict_set(
            *(*cffont).private.offset(0),
            b"defaultWidthX\x00" as *const u8 as *const libc::c_char,
            0i32,
            defaultwidth,
        );
    }
    if nominalwidth != 0.0f64 {
        cff_dict_add(
            *(*cffont).private.offset(0),
            b"nominalWidthX\x00" as *const u8 as *const libc::c_char,
            1i32,
        );
        cff_dict_set(
            *(*cffont).private.offset(0),
            b"nominalWidthX\x00" as *const u8 as *const libc::c_char,
            0i32,
            nominalwidth,
        );
    }
    if cff_dict_known(
        *(*cffont).private.offset(0),
        b"ForceBold\x00" as *const u8 as *const libc::c_char,
    ) != 0
        && cff_dict_get(
            *(*cffont).private.offset(0),
            b"ForceBold\x00" as *const u8 as *const libc::c_char,
            0i32,
        ) != 0.
    {
        flags |= 1i32 << 18i32
    }
    if cff_dict_known(
        *(*cffont).private.offset(0),
        b"IsFixedPitch\x00" as *const u8 as *const libc::c_char,
    ) != 0
        && cff_dict_get(
            *(*cffont).private.offset(0),
            b"IsFixedPitch\x00" as *const u8 as *const libc::c_char,
            0i32,
        ) != 0.
    {
        flags |= 1i32 << 0i32
    }
    fontname = pdf_font_get_fontname(font);
    descriptor = pdf_font_get_descriptor(font);
    if !fontname.is_null()
        && strstr(fontname, b"Sans\x00" as *const u8 as *const libc::c_char).is_null()
    {
        flags |= 1i32 << 1i32
    }
    if !fontname.is_null()
        && !strstr(fontname, b"Caps\x00" as *const u8 as *const libc::c_char).is_null()
    {
        flags |= 1i32 << 17i32
    }
    flags |= 1i32 << 2i32;
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"CapHeight\x00" as *const u8 as *const libc::c_char),
        pdf_new_number(capheight),
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"Ascent\x00" as *const u8 as *const libc::c_char),
        pdf_new_number(ascent),
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"Descent\x00" as *const u8 as *const libc::c_char),
        pdf_new_number(descent),
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"ItalicAngle\x00" as *const u8 as *const libc::c_char),
        pdf_new_number(italicangle),
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"StemV\x00" as *const u8 as *const libc::c_char),
        pdf_new_number(stemv),
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"Flags\x00" as *const u8 as *const libc::c_char),
        pdf_new_number(flags as libc::c_double),
    );
}
unsafe extern "C" fn add_metrics(
    mut font: *mut pdf_font,
    mut cffont: *mut cff_font,
    mut enc_vec: *mut *mut libc::c_char,
    mut widths: *mut libc::c_double,
    mut num_glyphs: libc::c_int,
) {
    let mut fontdict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut descriptor: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut tmp_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut code: libc::c_int = 0;
    let mut firstchar: libc::c_int = 0;
    let mut lastchar: libc::c_int = 0;
    let mut val: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut tfm_id: libc::c_int = 0;
    let mut usedchars: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scaling: libc::c_double = 0.;
    fontdict = pdf_font_get_resource(font);
    descriptor = pdf_font_get_descriptor(font);
    usedchars = pdf_font_get_usedchars(font);
    /*
     * The original FontBBox of the font is preserved, instead
     * of replacing it with tight bounding box calculated from
     * charstrings, to prevent Acrobat 4 from greeking text as
     * much as possible.
     */
    if cff_dict_known(
        (*cffont).topdict,
        b"FontBBox\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        _tt_abort(b"No FontBBox?\x00" as *const u8 as *const libc::c_char);
    }
    /* The widhts array in the font dictionary must be given relative
     * to the default scaling of 1000:1, not relative to the scaling
     * given by the font matrix.
     */
    if cff_dict_known(
        (*cffont).topdict,
        b"FontMatrix\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        scaling = 1000i32 as libc::c_double
            * cff_dict_get(
                (*cffont).topdict,
                b"FontMatrix\x00" as *const u8 as *const libc::c_char,
                0i32,
            )
    } else {
        scaling = 1i32 as libc::c_double
    }
    tmp_array = pdf_new_array();
    i = 0i32;
    while i < 4i32 {
        val = cff_dict_get(
            (*cffont).topdict,
            b"FontBBox\x00" as *const u8 as *const libc::c_char,
            i,
        );
        pdf_add_array(
            tmp_array,
            pdf_new_number(floor(val / 1.0f64 + 0.5f64) * 1.0f64),
        );
        i += 1
    }
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"FontBBox\x00" as *const u8 as *const libc::c_char),
        tmp_array,
    );
    tmp_array = pdf_new_array();
    if num_glyphs <= 1i32 {
        /* This must be an error. */
        lastchar = 0i32;
        firstchar = lastchar;
        pdf_add_array(tmp_array, pdf_new_number(0.0f64));
    } else {
        firstchar = 255i32;
        lastchar = 0i32;
        code = 0i32;
        while code < 256i32 {
            if *usedchars.offset(code as isize) != 0 {
                if code < firstchar {
                    firstchar = code
                }
                if code > lastchar {
                    lastchar = code
                }
            }
            code += 1
        }
        if firstchar > lastchar {
            dpx_warning(b"No glyphs actually used???\x00" as *const u8 as *const libc::c_char);
            pdf_release_obj(tmp_array);
            return;
        }
        /* PLEASE FIX THIS
         * It's wrong to use TFM width here... We should warn if TFM width
         * and actual glyph width are different.
         */
        tfm_id = tfm_open(pdf_font_get_mapname(font), 0i32);
        code = firstchar;
        while code <= lastchar {
            if *usedchars.offset(code as isize) != 0 {
                let mut width: libc::c_double = 0.;
                if tfm_id < 0i32 {
                    /* tfm is not found */
                    width = scaling
                        * *widths.offset(
                            cff_glyph_lookup(cffont, *enc_vec.offset(code as isize)) as isize
                        )
                } else {
                    let mut diff: libc::c_double = 0.;
                    width = 1000.0f64 * tfm_get_width(tfm_id, code);
                    diff = width
                        - scaling
                            * *widths
                                .offset(cff_glyph_lookup(cffont, *enc_vec.offset(code as isize))
                                    as isize);
                    if fabs(diff) > 1.0f64 {
                        dpx_warning(
                            b"Glyph width mismatch for TFM and font (%s)\x00" as *const u8
                                as *const libc::c_char,
                            pdf_font_get_mapname(font),
                        );
                        dpx_warning(
                            b"TFM: %g vs. Type1 font: %g\x00" as *const u8 as *const libc::c_char,
                            width,
                            *widths
                                .offset(cff_glyph_lookup(cffont, *enc_vec.offset(code as isize))
                                    as isize),
                        );
                    }
                }
                pdf_add_array(
                    tmp_array,
                    pdf_new_number(floor(width / 0.1f64 + 0.5f64) * 0.1f64),
                );
            } else {
                pdf_add_array(tmp_array, pdf_new_number(0.0f64));
            }
            code += 1
        }
    }
    if pdf_array_length(tmp_array) > 0i32 as libc::c_uint {
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"Widths\x00" as *const u8 as *const libc::c_char),
            pdf_ref_obj(tmp_array),
        );
    }
    pdf_release_obj(tmp_array);
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"FirstChar\x00" as *const u8 as *const libc::c_char),
        pdf_new_number(firstchar as libc::c_double),
    );
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"LastChar\x00" as *const u8 as *const libc::c_char),
        pdf_new_number(lastchar as libc::c_double),
    );
}
unsafe extern "C" fn write_fontfile(
    mut font: *mut pdf_font,
    mut cffont: *mut cff_font,
    mut pdfcharset: *mut pdf_obj,
) -> libc::c_int {
    let mut descriptor: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut fontfile: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut topdict: *mut cff_index = 0 as *mut cff_index;
    let mut private_size: libc::c_int = 0;
    let mut stream_data_len: libc::c_int = 0;
    let mut charstring_len: libc::c_int = 0;
    let mut topdict_offset: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut stream_data_ptr: *mut card8 = 0 as *mut card8;
    let mut wbuf: [card8; 1024] = [0; 1024];
    descriptor = pdf_font_get_descriptor(font);
    topdict = cff_new_index(1i32 as card16);
    /*
     * Force existence of Encoding.
     */
    if cff_dict_known(
        (*cffont).topdict,
        b"CharStrings\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        cff_dict_add(
            (*cffont).topdict,
            b"CharStrings\x00" as *const u8 as *const libc::c_char,
            1i32,
        );
    }
    if cff_dict_known(
        (*cffont).topdict,
        b"charset\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        cff_dict_add(
            (*cffont).topdict,
            b"charset\x00" as *const u8 as *const libc::c_char,
            1i32,
        );
    }
    if cff_dict_known(
        (*cffont).topdict,
        b"Encoding\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        cff_dict_add(
            (*cffont).topdict,
            b"Encoding\x00" as *const u8 as *const libc::c_char,
            1i32,
        );
    }
    private_size = cff_dict_pack(*(*cffont).private.offset(0), wbuf.as_mut_ptr(), 1024i32);
    /* Private dict is required (but may have size 0) */
    if cff_dict_known(
        (*cffont).topdict,
        b"Private\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        cff_dict_add(
            (*cffont).topdict,
            b"Private\x00" as *const u8 as *const libc::c_char,
            2i32,
        );
    }
    *(*topdict).offset.offset(1) =
        (cff_dict_pack((*cffont).topdict, wbuf.as_mut_ptr(), 1024i32) + 1i32) as l_offset;
    /*
     * Estimate total size of fontfile.
     */
    charstring_len = cff_index_size((*cffont).cstrings); /* header size */
    stream_data_len = 4i32;
    stream_data_len += cff_index_size((*cffont).name);
    stream_data_len += cff_index_size(topdict);
    stream_data_len += cff_index_size((*cffont).string);
    stream_data_len += cff_index_size((*cffont).gsubr);
    /* We are using format 1 for Encoding and format 0 for charset.
     * TODO: Should implement cff_xxx_size().
     */
    stream_data_len += 2i32
        + (*(*cffont).encoding).num_entries as libc::c_int * 2i32
        + 1i32
        + (*(*cffont).encoding).num_supps as libc::c_int * 3i32;
    stream_data_len += 1i32 + (*(*cffont).charsets).num_entries as libc::c_int * 2i32;
    stream_data_len += charstring_len;
    stream_data_len += private_size;
    /*
     * Now we create FontFile data.
     */
    stream_data_ptr = new((stream_data_len as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong)
        as u32) as *mut card8;
    /*
     * Data Layout order as described in CFF spec., sec 2 "Data Layout".
     */
    offset = 0i32;
    /* Header */
    offset += cff_put_header(
        cffont,
        stream_data_ptr.offset(offset as isize),
        stream_data_len - offset,
    );
    /* Name */
    offset += cff_pack_index(
        (*cffont).name,
        stream_data_ptr.offset(offset as isize),
        stream_data_len - offset,
    );
    /* Top DICT */
    topdict_offset = offset;
    offset += cff_index_size(topdict);
    /* Strings */
    offset += cff_pack_index(
        (*cffont).string,
        stream_data_ptr.offset(offset as isize),
        stream_data_len - offset,
    );
    /* Global Subrs */
    offset += cff_pack_index(
        (*cffont).gsubr,
        stream_data_ptr.offset(offset as isize),
        stream_data_len - offset,
    );
    /* Encoding */
    /* TODO: don't write Encoding entry if the font is always used
     * with PDF Encoding information. Applies to type1c.c as well.
     */
    cff_dict_set(
        (*cffont).topdict,
        b"Encoding\x00" as *const u8 as *const libc::c_char,
        0i32,
        offset as libc::c_double,
    );
    offset += cff_pack_encoding(
        cffont,
        stream_data_ptr.offset(offset as isize),
        stream_data_len - offset,
    );
    /* charset */
    cff_dict_set(
        (*cffont).topdict,
        b"charset\x00" as *const u8 as *const libc::c_char,
        0i32,
        offset as libc::c_double,
    );
    offset += cff_pack_charsets(
        cffont,
        stream_data_ptr.offset(offset as isize),
        stream_data_len - offset,
    );
    /* CharStrings */
    cff_dict_set(
        (*cffont).topdict,
        b"CharStrings\x00" as *const u8 as *const libc::c_char,
        0i32,
        offset as libc::c_double,
    );
    offset += cff_pack_index(
        (*cffont).cstrings,
        stream_data_ptr.offset(offset as isize),
        charstring_len,
    );
    /* Private */
    if !(*(*cffont).private.offset(0)).is_null() && private_size > 0i32 {
        private_size = cff_dict_pack(
            *(*cffont).private.offset(0),
            stream_data_ptr.offset(offset as isize),
            private_size,
        );
        cff_dict_set(
            (*cffont).topdict,
            b"Private\x00" as *const u8 as *const libc::c_char,
            1i32,
            offset as libc::c_double,
        );
        cff_dict_set(
            (*cffont).topdict,
            b"Private\x00" as *const u8 as *const libc::c_char,
            0i32,
            private_size as libc::c_double,
        );
    }
    offset += private_size;
    /* Finally Top DICT */
    (*topdict).data = new(
        ((*(*topdict).offset.offset(1)).wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong) as u32,
    ) as *mut card8;
    cff_dict_pack(
        (*cffont).topdict,
        (*topdict).data,
        (*(*topdict).offset.offset(1)).wrapping_sub(1i32 as libc::c_uint) as libc::c_int,
    );
    cff_pack_index(
        topdict,
        stream_data_ptr.offset(topdict_offset as isize),
        cff_index_size(topdict),
    );
    cff_release_index(topdict);
    /* Copyright and Trademark Notice ommited. */
    /* Flush Font File */
    fontfile = pdf_new_stream(1i32 << 0i32);
    stream_dict = pdf_stream_dict(fontfile);
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"FontFile3\x00" as *const u8 as *const libc::c_char),
        pdf_ref_obj(fontfile),
    );
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Subtype\x00" as *const u8 as *const libc::c_char),
        pdf_new_name(b"Type1C\x00" as *const u8 as *const libc::c_char),
    );
    pdf_add_stream(fontfile, stream_data_ptr as *mut libc::c_void, offset);
    pdf_release_obj(fontfile);
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"CharSet\x00" as *const u8 as *const libc::c_char),
        pdf_new_string(
            pdf_stream_dataptr(pdfcharset),
            pdf_stream_length(pdfcharset) as size_t,
        ),
    );
    free(stream_data_ptr as *mut libc::c_void);
    return offset;
}
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
pub unsafe extern "C" fn pdf_font_load_type1(mut font: *mut pdf_font) -> libc::c_int {
    let mut fontdict: *mut pdf_obj = 0 as *mut pdf_obj; /* Actually string object */
    let mut pdfcharset: *mut pdf_obj = 0 as *mut pdf_obj; /* With pseudo unique tag */
    let mut encoding_id: libc::c_int = 0;
    let mut usedchars: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ident: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fontname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uniqueTag: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    let mut enc_vec: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut defaultwidth: libc::c_double = 0.;
    let mut nominalwidth: libc::c_double = 0.;
    let mut widths: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut GIDMap: *mut card16 = 0 as *mut card16;
    let mut num_glyphs: card16 = 0i32 as card16;
    let mut offset: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    let mut verbose: libc::c_int = 0;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    if !font.is_null() {
    } else {
        __assert_fail(
            b"font\x00" as *const u8 as *const libc::c_char,
            b"dpx-type1.c\x00" as *const u8 as *const libc::c_char,
            505i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                b"int pdf_font_load_type1(pdf_font *)\x00",
            ))
            .as_ptr(),
        );
    }
    if !pdf_font_is_in_use(font) {
        return 0i32;
    }
    verbose = pdf_font_get_verbose();
    encoding_id = pdf_font_get_encoding(font);
    fontdict = pdf_font_get_resource(font);
    pdf_font_get_descriptor(font);
    usedchars = pdf_font_get_usedchars(font);
    ident = pdf_font_get_ident(font);
    fontname = pdf_font_get_fontname(font);
    uniqueTag = pdf_font_get_uniqueTag(font);
    if usedchars.is_null() || ident.is_null() || fontname.is_null() {
        _tt_abort(b"Type1: Unexpected error.\x00" as *const u8 as *const libc::c_char);
    }
    handle = ttstub_input_open(ident, TTIF_TYPE1, 0i32);
    if handle.is_null() {
        _tt_abort(
            b"Type1: Could not open Type1 font: %s\x00" as *const u8 as *const libc::c_char,
            ident,
        );
    }
    GIDMap = 0 as *mut card16;
    num_glyphs = 0i32 as card16;
    if encoding_id >= 0i32 {
        enc_vec = 0 as *mut *mut libc::c_char
    } else {
        enc_vec = new((256i32 as u32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as u32) as *mut *mut libc::c_char;
        code = 0i32;
        while code <= 0xffi32 {
            let ref mut fresh0 = *enc_vec.offset(code as isize);
            *fresh0 = 0 as *mut libc::c_char;
            code += 1
        }
    }
    cffont = t1_load_font(enc_vec, 0i32, handle);
    if cffont.is_null() {
        _tt_abort(
            b"Could not load Type 1 font: %s\x00" as *const u8 as *const libc::c_char,
            ident,
        );
    }
    ttstub_input_close(handle);
    fullname = new(
        (strlen(fontname).wrapping_add(8i32 as libc::c_ulong) as u32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as u32,
    ) as *mut libc::c_char;
    sprintf(
        fullname,
        b"%6s+%s\x00" as *const u8 as *const libc::c_char,
        uniqueTag,
        fontname,
    );
    /* Encoding related things. */
    if encoding_id >= 0i32 {
        enc_vec = pdf_encoding_get_encoding(encoding_id)
    } else {
        /* Create enc_vec and ToUnicode CMap for built-in encoding. */
        let mut tounicode: *mut pdf_obj = 0 as *mut pdf_obj;
        if pdf_lookup_dict(
            fontdict,
            b"ToUnicode\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
        {
            tounicode = pdf_create_ToUnicode_CMap(fullname, enc_vec, usedchars);
            if !tounicode.is_null() {
                pdf_add_dict(
                    fontdict,
                    pdf_new_name(b"ToUnicode\x00" as *const u8 as *const libc::c_char),
                    pdf_ref_obj(tounicode),
                );
                pdf_release_obj(tounicode);
            }
        }
    }
    cff_set_name(cffont, fullname);
    free(fullname as *mut libc::c_void);
    /* defaultWidthX, CapHeight, etc. */
    get_font_attr(font, cffont);
    if cff_dict_known(
        *(*cffont).private.offset(0),
        b"defaultWidthX\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        defaultwidth = cff_dict_get(
            *(*cffont).private.offset(0),
            b"defaultWidthX\x00" as *const u8 as *const libc::c_char,
            0i32,
        )
    } else {
        defaultwidth = 0.0f64
    }
    if cff_dict_known(
        *(*cffont).private.offset(0),
        b"nominalWidthX\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        nominalwidth = cff_dict_get(
            *(*cffont).private.offset(0),
            b"nominalWidthX\x00" as *const u8 as *const libc::c_char,
            0i32,
        )
    } else {
        nominalwidth = 0.0f64
    }
    /* Create CFF encoding, charset, sort glyphs */
    GIDMap = new((1024i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<card16>() as libc::c_ulong)
        as u32) as *mut card16; /* FIXME */
    pdfcharset = pdf_new_stream(0i32);
    let mut prev: libc::c_int = 0;
    let mut duplicate: libc::c_int = 0;
    let mut gid: libc::c_int = 0;
    let mut glyph: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sid: s_SID = 0;
    (*cffont).encoding = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cff_encoding>() as libc::c_ulong)
        as u32) as *mut cff_encoding;
    (*(*cffont).encoding).format = 1i32 as card8;
    (*(*cffont).encoding).num_entries = 0i32 as card8;
    (*(*cffont).encoding).data.range1 = new((256i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cff_range1>() as libc::c_ulong)
        as u32) as *mut cff_range1;
    (*(*cffont).encoding).num_supps = 0i32 as card8;
    (*(*cffont).encoding).supp = new((256i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cff_map>() as libc::c_ulong)
        as u32) as *mut cff_map;
    charset = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cff_charsets>() as libc::c_ulong)
        as u32) as *mut cff_charsets;
    (*charset).format = 0i32 as card8;
    (*charset).num_entries = 0i32 as card16;
    (*charset).data.glyphs = new((1024i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<s_SID>() as libc::c_ulong)
        as u32) as *mut s_SID;
    gid =
        cff_glyph_lookup(cffont, b".notdef\x00" as *const u8 as *const libc::c_char) as libc::c_int;
    if gid < 0i32 {
        _tt_abort(
            b"Type 1 font with no \".notdef\" glyph???\x00" as *const u8 as *const libc::c_char,
        );
    }
    *GIDMap.offset(0) = gid as card16;
    if verbose > 2i32 {
        dpx_message(b"[glyphs:/.notdef\x00" as *const u8 as *const libc::c_char);
    }
    num_glyphs = 1i32 as card16;
    prev = -2i32;
    code = 0i32;
    while code <= 0xffi32 {
        glyph = *enc_vec.offset(code as isize);
        if !(*usedchars.offset(code as isize) == 0) {
            if streq_ptr(glyph, b".notdef\x00" as *const u8 as *const libc::c_char) {
                dpx_warning(
                    b"Character mapped to .notdef used in font: %s\x00" as *const u8
                        as *const libc::c_char,
                    fontname,
                );
                *usedchars.offset(code as isize) = 0i32 as libc::c_char
            } else {
                gid = cff_glyph_lookup(cffont, glyph) as libc::c_int;
                if gid < 1i32 || gid >= (*(*cffont).cstrings).count as libc::c_int {
                    dpx_warning(
                        b"Glyph \"%s\" missing in font \"%s\".\x00" as *const u8
                            as *const libc::c_char,
                        glyph,
                        fontname,
                    );
                    *usedchars.offset(code as isize) = 0i32 as libc::c_char
                } else {
                    duplicate = 0i32;
                    while duplicate < code {
                        if *usedchars.offset(duplicate as isize) as libc::c_int != 0
                            && !(*enc_vec.offset(duplicate as isize)).is_null()
                            && streq_ptr(*enc_vec.offset(duplicate as isize), glyph) as libc::c_int
                                != 0
                        {
                            break;
                        }
                        duplicate += 1
                    }
                    sid = cff_add_string(cffont, glyph, 1i32);
                    if duplicate < code {
                        /* found duplicates */
                        (*(*(*cffont).encoding)
                            .supp
                            .offset((*(*cffont).encoding).num_supps as isize))
                        .code = duplicate as card8;
                        (*(*(*cffont).encoding)
                            .supp
                            .offset((*(*cffont).encoding).num_supps as isize))
                        .glyph = sid;
                        (*(*cffont).encoding).num_supps =
                            ((*(*cffont).encoding).num_supps as libc::c_int + 1i32) as card8
                    } else {
                        *GIDMap.offset(num_glyphs as isize) = gid as card16;
                        *(*charset)
                            .data
                            .glyphs
                            .offset((*charset).num_entries as isize) = sid;
                        (*charset).num_entries =
                            ((*charset).num_entries as libc::c_int + 1i32) as card16;
                        if code != prev + 1i32 {
                            (*(*cffont).encoding).num_entries =
                                ((*(*cffont).encoding).num_entries as libc::c_int + 1i32) as card8;
                            (*(*(*cffont).encoding).data.range1.offset(
                                ((*(*cffont).encoding).num_entries as libc::c_int - 1i32) as isize,
                            ))
                            .first = code as s_SID;
                            (*(*(*cffont).encoding).data.range1.offset(
                                ((*(*cffont).encoding).num_entries as libc::c_int - 1i32) as isize,
                            ))
                            .n_left = 0i32 as card8
                        } else {
                            let ref mut fresh1 = (*(*(*cffont).encoding).data.range1.offset(
                                ((*(*cffont).encoding).num_entries as libc::c_int - 1i32) as isize,
                            ))
                            .n_left;
                            *fresh1 = (*fresh1 as libc::c_int + 1i32) as card8
                        }
                        prev = code;
                        num_glyphs = num_glyphs.wrapping_add(1);
                        if verbose > 2i32 {
                            dpx_message(b"/%s\x00" as *const u8 as *const libc::c_char, glyph);
                        }
                        /* CharSet is actually string object. */
                        pdf_add_stream(
                            pdfcharset,
                            b"/\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                            1i32,
                        );
                        pdf_add_stream(
                            pdfcharset,
                            glyph as *const libc::c_void,
                            strlen(glyph) as libc::c_int,
                        );
                    }
                }
            }
        }
        code += 1
    }
    if (*(*cffont).encoding).num_supps as libc::c_int > 0i32 {
        (*(*cffont).encoding).format =
            ((*(*cffont).encoding).format as libc::c_int | 0x80i32) as card8
    } else {
        (*(*cffont).encoding).supp =
            mfree((*(*cffont).encoding).supp as *mut libc::c_void) as *mut cff_map
    }
    widths = new(((*(*cffont).cstrings).count as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as u32) as *mut libc::c_double;
    /* No more strings will be added. The Type 1 seac operator may add another
     * glyph but the glyph name of those glyphs are contained in standard
     * string. The String Index will not be modified after here. BUT: We
     * cannot update the String Index yet because then we wouldn't be able to
     * find the GIDs of the base and accent characters (unless they have been
     * used already).
     */
    let mut cstring: *mut cff_index = 0 as *mut cff_index;
    let mut gm: t1_ginfo = t1_ginfo {
        use_seac: 0,
        wx: 0.,
        wy: 0.,
        bbox: C2RustUnnamed_3 {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        seac: C2RustUnnamed_2 {
            asb: 0.,
            adx: 0.,
            ady: 0.,
            bchar: 0,
            achar: 0,
        },
    };
    let mut gid_0: card16 = 0;
    let mut gid_orig: card16 = 0;
    let mut dstlen_max: libc::c_int = 0;
    let mut srclen: libc::c_int = 0;
    let mut srcptr: *mut card8 = 0 as *mut card8;
    let mut dstptr: *mut card8 = 0 as *mut card8;
    dstlen_max = 0i64 as libc::c_int;
    offset = dstlen_max;
    cstring = cff_new_index((*(*cffont).cstrings).count);
    (*cstring).data = 0 as *mut card8;
    *(*cstring).offset.offset(0) = 1i32 as l_offset;
    let mut current_block_150: u64;
    /* The num_glyphs increases if "seac" operators are used. */
    gid_0 = 0i32 as card16;
    while (gid_0 as libc::c_int) < num_glyphs as libc::c_int {
        if offset + 65536i32 >= dstlen_max {
            dstlen_max += 65536i32 * 2i32;
            (*cstring).data = renew(
                (*cstring).data as *mut libc::c_void,
                (dstlen_max as u32 as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong)
                    as u32,
            ) as *mut card8
        }
        gid_orig = *GIDMap.offset(gid_0 as isize);
        dstptr = (*cstring)
            .data
            .offset(*(*cstring).offset.offset(gid_0 as isize) as isize)
            .offset(-1);
        srcptr = (*(*cffont).cstrings)
            .data
            .offset(*(*(*cffont).cstrings).offset.offset(gid_orig as isize) as isize)
            .offset(-1);
        srclen = (*(*(*cffont).cstrings)
            .offset
            .offset((gid_orig as libc::c_int + 1i32) as isize))
        .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid_orig as isize))
            as libc::c_int;
        offset += t1char_convert_charstring(
            dstptr,
            65536i32,
            srcptr,
            srclen,
            *(*cffont).subrs.offset(0),
            defaultwidth,
            nominalwidth,
            &mut gm,
        );
        *(*cstring)
            .offset
            .offset((gid_0 as libc::c_int + 1i32) as isize) = (offset + 1i32) as l_offset;
        if gm.use_seac != 0 {
            let mut bchar_gid: libc::c_int = 0;
            let mut achar_gid: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            let mut bchar_name: *const libc::c_char = 0 as *const libc::c_char;
            let mut achar_name: *const libc::c_char = 0 as *const libc::c_char;
            /*
             * NOTE:
             *  1. seac.achar and seac.bchar must be contained in the CFF standard string.
             *  2. Those characters need not to be encoded.
             *  3. num_glyphs == charsets->num_entries + 1.
             */
            achar_name = t1_get_standard_glyph(gm.seac.achar as libc::c_int);
            achar_gid = cff_glyph_lookup(cffont, achar_name) as libc::c_int;
            bchar_name = t1_get_standard_glyph(gm.seac.bchar as libc::c_int);
            bchar_gid = cff_glyph_lookup(cffont, bchar_name) as libc::c_int;
            if achar_gid < 0i32 {
                dpx_warning(
                    b"Accent char \"%s\" not found. Invalid use of \"seac\" operator.\x00"
                        as *const u8 as *const libc::c_char,
                    achar_name,
                );
                current_block_150 = 1069630499025798221;
            } else if bchar_gid < 0i32 {
                dpx_warning(
                    b"Base char \"%s\" not found. Invalid use of \"seac\" operator.\x00"
                        as *const u8 as *const libc::c_char,
                    bchar_name,
                );
                current_block_150 = 1069630499025798221;
            } else {
                i = 0i32;
                while i < num_glyphs as libc::c_int {
                    if *GIDMap.offset(i as isize) as libc::c_int == achar_gid {
                        break;
                    }
                    i += 1
                }
                if i == num_glyphs as libc::c_int {
                    if verbose > 2i32 {
                        dpx_message(b"/%s\x00" as *const u8 as *const libc::c_char, achar_name);
                    }
                    let fresh2 = num_glyphs;
                    num_glyphs = num_glyphs.wrapping_add(1);
                    *GIDMap.offset(fresh2 as isize) = achar_gid as card16;
                    *(*charset)
                        .data
                        .glyphs
                        .offset((*charset).num_entries as isize) =
                        cff_get_seac_sid(cffont, achar_name) as s_SID;
                    (*charset).num_entries =
                        ((*charset).num_entries as libc::c_int + 1i32) as card16
                }
                i = 0i32;
                while i < num_glyphs as libc::c_int {
                    if *GIDMap.offset(i as isize) as libc::c_int == bchar_gid {
                        break;
                    }
                    i += 1
                }
                if i == num_glyphs as libc::c_int {
                    if verbose > 2i32 {
                        dpx_message(b"/%s\x00" as *const u8 as *const libc::c_char, bchar_name);
                    }
                    let fresh3 = num_glyphs;
                    num_glyphs = num_glyphs.wrapping_add(1);
                    *GIDMap.offset(fresh3 as isize) = bchar_gid as card16;
                    *(*charset)
                        .data
                        .glyphs
                        .offset((*charset).num_entries as isize) =
                        cff_get_seac_sid(cffont, bchar_name) as s_SID;
                    (*charset).num_entries =
                        ((*charset).num_entries as libc::c_int + 1i32) as card16
                }
                current_block_150 = 17100064147490331435;
            }
        } else {
            current_block_150 = 17100064147490331435;
        }
        match current_block_150 {
            17100064147490331435 => *widths.offset(gid_0 as isize) = gm.wx,
            _ => {}
        }
        gid_0 = gid_0.wrapping_add(1)
    }
    (*cstring).count = num_glyphs;
    cff_release_index(*(*cffont).subrs.offset(0));
    let ref mut fresh4 = *(*cffont).subrs.offset(0);
    *fresh4 = 0 as *mut cff_index;
    (*cffont).subrs = mfree((*cffont).subrs as *mut libc::c_void) as *mut *mut cff_index;
    cff_release_index((*cffont).cstrings);
    (*cffont).cstrings = cstring;
    cff_release_charsets((*cffont).charsets);
    (*cffont).charsets = charset;
    if verbose > 2i32 {
        dpx_message(b"]\x00" as *const u8 as *const libc::c_char);
    }
    /* Now we can update the String Index */
    cff_dict_update((*cffont).topdict, cffont);
    cff_dict_update(*(*cffont).private.offset(0), cffont);
    cff_update_string(cffont);
    add_metrics(font, cffont, enc_vec, widths, num_glyphs as libc::c_int);
    offset = write_fontfile(font, cffont, pdfcharset);
    if verbose > 1i32 {
        dpx_message(
            b"[%u glyphs][%d bytes]\x00" as *const u8 as *const libc::c_char,
            num_glyphs as libc::c_int,
            offset,
        );
    }
    pdf_release_obj(pdfcharset);
    cff_close(cffont);
    /* Cleanup */
    if encoding_id < 0i32 && !enc_vec.is_null() {
        code = 0i32;
        while code < 256i32 {
            let ref mut fresh5 = *enc_vec.offset(code as isize);
            *fresh5 =
                mfree(*enc_vec.offset(code as isize) as *mut libc::c_void) as *mut libc::c_char;
            code += 1
        }
        free(enc_vec as *mut libc::c_void);
    }
    free(widths as *mut libc::c_void);
    free(GIDMap as *mut libc::c_void);
    /* Maybe writing Charset is recommended for subsetted font. */
    return 0i32;
}
