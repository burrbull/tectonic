/* ***************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009, 2011 by Jonathan Kew

 SIL Author(s): Jonathan Kew

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE
FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of the copyright holders
shall not be used in advertising or otherwise to promote the sale,
use or other dealings in this Software without prior written
authorization from the copyright holders.
\****************************************************************************/
/*
 *   file name:  XeTeXFontInst.h
 *
 *   created on: 2005-10-22
 *   created by: Jonathan Kew
 *
 *  originally based on PortableFontInstance.h from ICU
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use crate::core_memory::xmalloc;
use harfbuzz_sys::{
    hb_blob_create, hb_blob_t, hb_bool_t, hb_codepoint_t, hb_destroy_func_t,
    hb_face_create_for_tables, hb_face_destroy, hb_face_set_index, hb_face_set_upem, hb_face_t,
    hb_font_create, hb_font_destroy, hb_font_funcs_create,
    hb_font_funcs_set_glyph_contour_point_func, hb_font_funcs_set_glyph_extents_func,
    hb_font_funcs_set_glyph_h_advance_func, hb_font_funcs_set_glyph_h_origin_func,
    hb_font_funcs_set_glyph_name_func, hb_font_funcs_set_glyph_v_advance_func,
    hb_font_funcs_set_glyph_v_origin_func, hb_font_funcs_t, hb_font_set_funcs, hb_font_set_ppem,
    hb_font_set_scale, hb_font_t, hb_glyph_extents_t, hb_position_t, hb_tag_t,
    HB_MEMORY_MODE_WRITABLE,
};
use std::io::Read;

use crate::freetype_sys_patch::{FT_Face_GetCharVariantIndex, FT_Get_Advance, FT_Load_Sfnt_Table};
use freetype::freetype::{
    FT_Attach_Stream, FT_Done_Face, FT_Get_Char_Index, FT_Get_First_Char, FT_Get_Glyph_Name,
    FT_Get_Kerning, FT_Get_Name_Index, FT_Get_Next_Char, FT_Get_Sfnt_Table, FT_Init_FreeType,
    FT_Load_Glyph, FT_New_Memory_Face,
};
use freetype::freetype::{
    FT_Byte, FT_Error, FT_Face, FT_Fixed, FT_Library, FT_Long, FT_Pointer, FT_Sfnt_Tag, FT_UInt,
    FT_ULong, FT_Vector,
};
use freetype_sys::{FT_BBox, FT_Done_Glyph, FT_Get_Glyph, FT_Glyph, FT_Glyph_Get_CBox};

use bridge::{ttstub_input_get_size, InFile};

use std::ptr;

use bridge::TTInputFormat;

#[cfg(not(target_os = "macos"))]
pub(crate) mod imp {}

#[cfg(target_os = "macos")]
#[path = "xetex_font_info_coretext.rs"]
pub(crate) mod imp;

use libc::free;
extern "C" {
    // TODO: NOTE: this api doesn't included in harfbuzz_sys
    fn hb_font_funcs_set_glyph_h_kerning_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_h_kerning_func_t,
        user_data: *mut libc::c_void,
        destroy: hb_destroy_func_t,
    );
    fn hb_font_funcs_set_glyph_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_func_t,
        user_data: *mut libc::c_void,
        destroy: hb_destroy_func_t,
    );
    fn hb_font_funcs_set_glyph_v_kerning_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_v_kerning_func_t,
        user_data: *mut libc::c_void,
        destroy: hb_destroy_func_t,
    );
}
pub(crate) type int32_t = i32;
pub(crate) type uint16_t = u16;
pub(crate) type uint32_t = u32;

pub(crate) type UChar32 = int32_t;
/* quasi-hack to get the primary input */
/* */
/* this #if 0 ... #endif clause is for documentation purposes */

pub(crate) type hb_font_get_glyph_kerning_func_t = Option<
    unsafe extern "C" fn(
        _: *mut hb_font_t,
        _: *mut libc::c_void,
        _: hb_codepoint_t,
        _: hb_codepoint_t,
        _: *mut libc::c_void,
    ) -> hb_position_t,
>;
pub(crate) type hb_font_get_glyph_h_kerning_func_t = hb_font_get_glyph_kerning_func_t;
pub(crate) type hb_font_get_glyph_func_t = Option<
    unsafe extern "C" fn(
        _: *mut hb_font_t,
        _: *mut libc::c_void,
        _: hb_codepoint_t,
        _: hb_codepoint_t,
        _: *mut hb_codepoint_t,
        _: *mut libc::c_void,
    ) -> hb_bool_t,
>;
pub(crate) type hb_font_get_glyph_v_kerning_func_t = hb_font_get_glyph_kerning_func_t;

pub(crate) type GlyphID = uint16_t;

use crate::xetex_scaledmath::Scaled;

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub(crate) struct GlyphBBox {
    pub(crate) xMin: f32,
    pub(crate) yMin: f32,
    pub(crate) xMax: f32,
    pub(crate) yMax: f32,
}

impl GlyphBBox {
    pub const fn zero() -> Self {
        GlyphBBox {
            xMin: 0.,
            yMin: 0.,
            xMax: 0.,
            yMax: 0.,
        }
    }
}

// create specific subclasses for each supported platform
// false = horizontal, true = vertical
// font filename
// face index
#[derive(Clone)]
pub(crate) struct XeTeXFontInst {
    pub(crate) m_unitsPerEM: u16,
    pub(crate) m_pointSize: f32,
    pub(crate) m_ascent: f32,
    pub(crate) m_descent: f32,
    pub(crate) m_capHeight: f32,
    pub(crate) m_xHeight: f32,
    pub(crate) m_italicAngle: f32,
    pub(crate) m_vertical: bool,
    pub(crate) m_filename: String,
    pub(crate) m_index: u32,
    pub(crate) m_ftFace: FT_Face,
    pub(crate) m_backingData: Vec<u8>,
    pub(crate) m_backingData2: Vec<u8>,
    pub(crate) m_hbFont: *mut hb_font_t,
}

/* Return NAME with any leading path stripped off.  This returns a
pointer into NAME.  For example, `basename ("/foo/bar.baz")'
returns "bar.baz".  */
fn xbasename(name: &str) -> &str {
    let mut base = name;
    let mut p = base;
    while !p.is_empty() {
        if let Some(stripped) = p.strip_prefix('/') {
            base = stripped;
        }
        p = &p[1..];
    }
    base
}
#[no_mangle]
pub(crate) static mut gFreeTypeLibrary: FT_Library = 0 as FT_Library;
static mut hbFontFuncs: *mut hb_font_funcs_t = 0 as *mut hb_font_funcs_t;

impl XeTeXFontInst {
    pub(crate) unsafe fn base_ctor(
        pathname: &str,
        index: i32,
        pointSize: f32,
        status: &mut bool,
    ) -> Self {
        let mut res = Self {
            m_unitsPerEM: 0,
            m_pointSize: pointSize,
            m_ascent: 0.,
            m_descent: 0.,
            m_capHeight: 0.,
            m_xHeight: 0.,
            m_italicAngle: 0.,
            m_vertical: false,
            m_filename: String::new(),
            m_index: 0,
            m_ftFace: 0 as FT_Face,
            m_backingData: Vec::new(),
            m_backingData2: Vec::new(),
            m_hbFont: ptr::null_mut(),
        };
        if !pathname.is_empty() {
            res.initialize(pathname, index, status);
        };
        res
    }
    pub(crate) unsafe fn create(
        pathname: &str,
        index: i32,
        pointSize: f32,
        status: &mut bool,
    ) -> Box<Self> {
        Box::new(Self::base_ctor(pathname, index, pointSize, status))
    }
}

impl Drop for XeTeXFontInst {
    fn drop(&mut self) {
        unsafe {
            if !self.m_ftFace.is_null() {
                FT_Done_Face(self.m_ftFace);
                self.m_ftFace = 0 as FT_Face
            }
            hb_font_destroy(self.m_hbFont);
        }
    }
}

/* HarfBuzz font functions */
unsafe extern "C" fn _get_glyph(
    mut _hbf: *mut hb_font_t,
    font_data: *mut libc::c_void,
    ch: hb_codepoint_t,
    vs: hb_codepoint_t,
    gid: *mut hb_codepoint_t,
    mut _p: *mut libc::c_void,
) -> hb_bool_t {
    let face = font_data as FT_Face;
    *gid = 0 as hb_codepoint_t;
    if vs != 0 {
        *gid = FT_Face_GetCharVariantIndex(face, ch as FT_ULong, vs as FT_ULong)
    }
    if *gid == 0 {
        *gid = FT_Get_Char_Index(face, ch as FT_ULong)
    }
    (*gid != 0) as hb_bool_t
}
unsafe extern "C" fn _get_glyph_advance(face: FT_Face, gid: FT_UInt, vertical: bool) -> FT_Fixed {
    let mut advance = 0;
    let mut flags = 1 << 0;
    if vertical {
        flags |= 1 << 4;
    }
    let error = FT_Get_Advance(face, gid, flags, &mut advance);
    if error != 0 {
        advance = 0;
    }
    /* FreeType's vertical metrics grows downward */
    if vertical {
        -advance
    } else {
        advance
    }
}
unsafe extern "C" fn _get_glyph_h_advance(
    mut _hbf: *mut hb_font_t,
    font_data: *mut libc::c_void,
    gid: hb_codepoint_t,
    mut _p: *mut libc::c_void,
) -> hb_position_t {
    _get_glyph_advance(font_data as FT_Face, gid, false) as hb_position_t
}
unsafe extern "C" fn _get_glyph_v_advance(
    mut _hbf: *mut hb_font_t,
    font_data: *mut libc::c_void,
    gid: hb_codepoint_t,
    mut _p: *mut libc::c_void,
) -> hb_position_t {
    _get_glyph_advance(font_data as FT_Face, gid, true) as hb_position_t
}
unsafe extern "C" fn _get_glyph_h_origin(
    mut _hbf: *mut hb_font_t,
    mut _font_data: *mut libc::c_void,
    mut _gid: hb_codepoint_t,
    mut _x: *mut hb_position_t,
    mut _y: *mut hb_position_t,
    mut _p: *mut libc::c_void,
) -> hb_bool_t {
    // horizontal origin is (0, 0)
    1
}
unsafe extern "C" fn _get_glyph_v_origin(
    mut _hbf: *mut hb_font_t,
    mut _font_data: *mut libc::c_void,
    mut _gid: hb_codepoint_t,
    mut _x: *mut hb_position_t,
    mut _y: *mut hb_position_t,
    mut _p: *mut libc::c_void,
) -> hb_bool_t {
    // vertical origin is (0, 0) for now
    1
}
unsafe extern "C" fn _get_glyph_h_kerning(
    mut _hbf: *mut hb_font_t,
    font_data: *mut libc::c_void,
    gid1: hb_codepoint_t,
    gid2: hb_codepoint_t,
    mut _p: *mut libc::c_void,
) -> hb_position_t {
    use freetype_sys::FT_KERNING_UNSCALED;
    let face = font_data as FT_Face;
    let mut kerning: FT_Vector = FT_Vector { x: 0, y: 0 };
    let error = FT_Get_Kerning(
        face,
        gid1,
        gid2,
        FT_KERNING_UNSCALED as i32 as FT_UInt,
        &mut kerning,
    );
    if error != 0 {
        0
    } else {
        kerning.x as hb_position_t
    }
}
unsafe extern "C" fn _get_glyph_v_kerning(
    mut _hbf: *mut hb_font_t,
    mut _font_data: *mut libc::c_void,
    mut _gid1: hb_codepoint_t,
    mut _gid2: hb_codepoint_t,
    mut _p: *mut libc::c_void,
) -> hb_position_t {
    /* FreeType does not support vertical kerning */
    0
}
unsafe extern "C" fn _get_glyph_extents(
    mut _hbf: *mut hb_font_t,
    font_data: *mut libc::c_void,
    gid: hb_codepoint_t,
    mut extents: *mut hb_glyph_extents_t,
    mut _p: *mut libc::c_void,
) -> hb_bool_t {
    let face = font_data as FT_Face;
    let error = FT_Load_Glyph(face, gid, 1 << 0);
    if error == 0 {
        (*extents).x_bearing = (*(*face).glyph).metrics.horiBearingX as hb_position_t;
        (*extents).y_bearing = (*(*face).glyph).metrics.horiBearingY as hb_position_t;
        (*extents).width = (*(*face).glyph).metrics.width as hb_position_t;
        (*extents).height = -(*(*face).glyph).metrics.height as hb_position_t
    }
    (error == 0) as i32
}
unsafe extern "C" fn _get_glyph_contour_point(
    mut _hbf: *mut hb_font_t,
    font_data: *mut libc::c_void,
    gid: hb_codepoint_t,
    point_index: u32,
    x: *mut hb_position_t,
    y: *mut hb_position_t,
    mut _p: *mut libc::c_void,
) -> hb_bool_t {
    use freetype_sys::FT_GLYPH_FORMAT_OUTLINE;
    let face = font_data as FT_Face;
    let mut ret = false;
    let error = FT_Load_Glyph(face, gid, 1 << 0);
    if error == 0
        && (*(*face).glyph).format as u32 == FT_GLYPH_FORMAT_OUTLINE as u32
        && point_index < (*(*face).glyph).outline.n_points as u32
    {
        *x = (*(*(*face).glyph).outline.points.offset(point_index as isize)).x as hb_position_t;
        *y = (*(*(*face).glyph).outline.points.offset(point_index as isize)).y as hb_position_t;
        ret = true;
    }
    ret as hb_bool_t
}
unsafe extern "C" fn _get_glyph_name(
    mut _hbf: *mut hb_font_t,
    font_data: *mut libc::c_void,
    gid: hb_codepoint_t,
    name: *mut i8,
    size: u32,
    mut _p: *mut libc::c_void,
) -> hb_bool_t {
    let face = font_data as FT_Face;
    let mut ret = FT_Get_Glyph_Name(face, gid, name as FT_Pointer, size) == 0;
    if ret && (size != 0 && *name == 0) {
        ret = false;
    }
    ret as hb_bool_t
}
unsafe extern "C" fn _get_font_funcs() -> *mut hb_font_funcs_t {
    static mut funcs: *mut hb_font_funcs_t = ptr::null_mut();
    if funcs.is_null() {
        funcs = hb_font_funcs_create()
    }
    hb_font_funcs_set_glyph_func(
        funcs,
        Some(
            _get_glyph
                as unsafe extern "C" fn(
                    _: *mut hb_font_t,
                    _: *mut libc::c_void,
                    _: hb_codepoint_t,
                    _: hb_codepoint_t,
                    _: *mut hb_codepoint_t,
                    _: *mut libc::c_void,
                ) -> hb_bool_t,
        ),
        ptr::null_mut(),
        None,
    );
    hb_font_funcs_set_glyph_h_advance_func(
        funcs,
        Some(_get_glyph_h_advance),
        ptr::null_mut(),
        None,
    );
    hb_font_funcs_set_glyph_v_advance_func(
        funcs,
        Some(_get_glyph_v_advance),
        ptr::null_mut(),
        None,
    );
    hb_font_funcs_set_glyph_h_origin_func(funcs, Some(_get_glyph_h_origin), ptr::null_mut(), None);
    hb_font_funcs_set_glyph_v_origin_func(funcs, Some(_get_glyph_v_origin), ptr::null_mut(), None);
    hb_font_funcs_set_glyph_h_kerning_func(
        funcs,
        Some(
            _get_glyph_h_kerning
                as unsafe extern "C" fn(
                    _: *mut hb_font_t,
                    _: *mut libc::c_void,
                    _: hb_codepoint_t,
                    _: hb_codepoint_t,
                    _: *mut libc::c_void,
                ) -> hb_position_t,
        ),
        ptr::null_mut(),
        None,
    );
    hb_font_funcs_set_glyph_v_kerning_func(
        funcs,
        Some(
            _get_glyph_v_kerning
                as unsafe extern "C" fn(
                    _: *mut hb_font_t,
                    _: *mut libc::c_void,
                    _: hb_codepoint_t,
                    _: hb_codepoint_t,
                    _: *mut libc::c_void,
                ) -> hb_position_t,
        ),
        ptr::null_mut(),
        None,
    );
    hb_font_funcs_set_glyph_extents_func(funcs, Some(_get_glyph_extents), ptr::null_mut(), None);
    hb_font_funcs_set_glyph_contour_point_func(
        funcs,
        Some(_get_glyph_contour_point),
        ptr::null_mut(),
        None,
    );
    hb_font_funcs_set_glyph_name_func(funcs, Some(_get_glyph_name), ptr::null_mut(), None);
    funcs
}
unsafe extern "C" fn _get_table(
    mut _hfc: *mut hb_face_t,
    tag: hb_tag_t,
    user_data: *mut libc::c_void,
) -> *mut hb_blob_t {
    let face = user_data as FT_Face;
    let mut length = 0;
    let mut blob = ptr::null_mut();
    let error = FT_Load_Sfnt_Table(face, tag as FT_ULong, 0, ptr::null_mut(), &mut length);
    if error == 0 {
        let table = xmalloc(length.wrapping_mul(::std::mem::size_of::<i8>() as libc::c_ulong) as _)
            as *mut FT_Byte;
        if !table.is_null() {
            let error = FT_Load_Sfnt_Table(face, tag as FT_ULong, 0, table, &mut length);
            if error == 0 {
                blob = hb_blob_create(
                    table as *const i8,
                    length as u32,
                    HB_MEMORY_MODE_WRITABLE,
                    table as *mut libc::c_void,
                    Some(free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                )
            } else {
                free(table as *mut libc::c_void);
            }
        }
    }
    blob
}
impl XeTeXFontInst {
    pub(crate) unsafe fn initialize(&mut self, pathname: &str, index: i32, status: &mut bool) {
        use freetype::freetype::FT_Open_Args;
        use freetype_sys::{TT_Postscript, TT_OS2};
        if gFreeTypeLibrary.is_null() {
            let error = FT_Init_FreeType(&mut gFreeTypeLibrary);
            if error != 0 {
                abort!("FreeType initialization failed, error {}", error);
            }
        }
        // Here we emulate some logic that was originally in find_native_font();
        let handle = InFile::open(pathname, TTInputFormat::OPENTYPE, 0)
            .or_else(|| InFile::open(pathname, TTInputFormat::TRUETYPE, 0))
            .or_else(|| InFile::open(pathname, TTInputFormat::TYPE1, 0));
        if handle.is_none() {
            *status = true;
            return;
        }
        let mut handle = handle.unwrap();
        let sz = ttstub_input_get_size(&mut handle) as usize;
        self.m_backingData = vec![0; sz];
        if handle
            .read_exact(self.m_backingData.as_mut_slice())
            .is_err()
        {
            abort!("failed to read font file");
        }
        let _error = FT_New_Memory_Face(
            gFreeTypeLibrary,
            self.m_backingData.as_ptr(),
            sz as FT_Long,
            index as FT_Long,
            &mut self.m_ftFace,
        );
        if (*self.m_ftFace).face_flags & 1 << 0 == 0 {
            *status = true;
            return;
        }
        /* for non-sfnt-packaged fonts (presumably Type 1), see if there is an AFM file we can attach */
        if index == 0 && (*self.m_ftFace).face_flags & 1 << 3 == 0 {
            // Tectonic: this code used to use kpse_find_file and FT_Attach_File
            // to try to find metrics for this font. Thanks to the existence of
            // FT_Attach_Stream we can emulate this behavior while going through
            // the Rust I/O layer.
            let mut afm = xbasename(pathname).to_string();
            if let Some(p) = afm.bytes().rposition(|b| b == b'.') {
                if let [b'p', b'f', _] = afm[p + 1..].to_lowercase().as_bytes() {
                    afm.truncate(p);
                    afm += ".afm";
                }
            }
            let afm_handle = InFile::open(&afm, TTInputFormat::AFM, 0);
            if let Some(mut afm_handle) = afm_handle {
                let sz = ttstub_input_get_size(&mut afm_handle) as usize;
                self.m_backingData2 = vec![0; sz];
                if afm_handle
                    .read_exact(self.m_backingData2.as_mut_slice())
                    .is_err()
                {
                    abort!("failed to read AFM file");
                }
                let mut open_args: FT_Open_Args = FT_Open_Args {
                    flags: 0,
                    memory_base: ptr::null(),
                    memory_size: 0,
                    pathname: ptr::null_mut(),
                    stream: ptr::null_mut(),
                    driver: ptr::null_mut(),
                    num_params: 0,
                    params: ptr::null_mut(),
                };
                open_args.flags = 0x1;
                open_args.memory_base = self.m_backingData2.as_ptr();
                open_args.memory_size = sz as FT_Long;
                FT_Attach_Stream(self.m_ftFace, &mut open_args);
            }
        }
        self.m_filename = pathname.to_string();
        self.m_index = index as uint32_t;
        self.m_unitsPerEM = (*self.m_ftFace).units_per_EM;
        self.m_ascent = self.units_to_points((*self.m_ftFace).ascender as f32);
        self.m_descent = self.units_to_points((*self.m_ftFace).descender as f32);
        let postTable = self.get_font_table_ft(FT_Sfnt_Tag::FT_SFNT_POST) as *mut TT_Postscript;
        if !postTable.is_null() {
            self.m_italicAngle = Scaled((*postTable).italicAngle as i32).into();
        }
        let os2Table = self.get_font_table_ft(FT_Sfnt_Tag::FT_SFNT_OS2) as *mut TT_OS2;
        if !os2Table.is_null() {
            self.m_capHeight = self.units_to_points((*os2Table).sCapHeight as f32);
            self.m_xHeight = self.units_to_points((*os2Table).sxHeight as f32)
        }
        // Set up HarfBuzz font
        let hbFace = hb_face_create_for_tables(
            Some(
                _get_table
                    as unsafe extern "C" fn(
                        _: *mut hb_face_t,
                        _: hb_tag_t,
                        _: *mut libc::c_void,
                    ) -> *mut hb_blob_t,
            ),
            self.m_ftFace as *mut libc::c_void,
            None,
        );
        hb_face_set_index(hbFace, index as u32);
        hb_face_set_upem(hbFace, self.m_unitsPerEM as u32);
        self.m_hbFont = hb_font_create(hbFace);
        hb_face_destroy(hbFace);
        if hbFontFuncs.is_null() {
            hbFontFuncs = _get_font_funcs()
        }
        hb_font_set_funcs(
            self.m_hbFont,
            hbFontFuncs,
            self.m_ftFace as *mut libc::c_void,
            None,
        );
        hb_font_set_scale(
            self.m_hbFont,
            self.m_unitsPerEM as i32,
            self.m_unitsPerEM as i32,
        );
        // We don’t want device tables adjustments
        hb_font_set_ppem(self.m_hbFont, 0, 0);
    }

    pub(crate) unsafe fn set_layout_dir_vertical(&mut self, vertical: bool) {
        self.m_vertical = vertical;
    }

    #[cfg(target_os = "macos")]
    pub(crate) unsafe fn get_font_table(&self, tag: u32) -> *mut libc::c_void {
        let mut tmpLength = 0;
        let mut error: FT_Error = FT_Load_Sfnt_Table(
            self.m_ftFace,
            tag as FT_ULong,
            0,
            0 as *mut FT_Byte,
            &mut tmpLength,
        );
        if error != 0 {
            return 0 as *mut libc::c_void;
        }
        let table =
            xmalloc(tmpLength.wrapping_mul(::std::mem::size_of::<i8>() as libc::c_ulong) as _);
        if !table.is_null() {
            error = FT_Load_Sfnt_Table(
                self.m_ftFace,
                tag as FT_ULong,
                0,
                table as *mut FT_Byte,
                &mut tmpLength,
            );
            if error != 0 {
                free(table);
                return 0 as *mut libc::c_void;
            }
        }
        table
    }
    pub(crate) unsafe fn get_font_table_ft(&self, tag: FT_Sfnt_Tag) -> *mut libc::c_void {
        FT_Get_Sfnt_Table(self.m_ftFace, tag)
    }

    pub(crate) unsafe fn get_glyph_bounds(&self, gid: GlyphID) -> Option<GlyphBBox> {
        use freetype_sys::FT_GLYPH_BBOX_UNSCALED;
        let mut bbox = GlyphBBox::default();
        let mut error: FT_Error = FT_Load_Glyph(self.m_ftFace, gid as FT_UInt, 1 << 0);
        if error != 0 {
            return None;
        }
        let mut glyph: FT_Glyph = 0 as FT_Glyph;
        error = FT_Get_Glyph(
            (*self.m_ftFace).glyph as freetype_sys::FT_GlyphSlot,
            &mut glyph,
        );
        if error == 0 {
            let mut ft_bbox = FT_BBox {
                xMin: 0,
                yMin: 0,
                xMax: 0,
                yMax: 0,
            };
            FT_Glyph_Get_CBox(
                glyph,
                FT_GLYPH_BBOX_UNSCALED as i32 as FT_UInt,
                &mut ft_bbox,
            );
            bbox.xMin = self.units_to_points(ft_bbox.xMin as f32);
            bbox.yMin = self.units_to_points(ft_bbox.yMin as f32);
            bbox.xMax = self.units_to_points(ft_bbox.xMax as f32);
            bbox.yMax = self.units_to_points(ft_bbox.yMax as f32);
            FT_Done_Glyph(glyph);
        };
        Some(bbox)
    }

    pub(crate) unsafe fn map_char_to_glyph(&self, ch: char) -> GlyphID {
        FT_Get_Char_Index(self.m_ftFace, ch as FT_ULong) as GlyphID
    }

    pub(crate) unsafe fn get_num_glyphs(&self) -> u16 {
        (*self.m_ftFace).num_glyphs as u16
    }

    pub(crate) unsafe fn get_glyph_width(&self, gid: GlyphID) -> f32 {
        self.units_to_points(_get_glyph_advance(self.m_ftFace, gid as FT_UInt, false) as f32)
    }

    pub(crate) unsafe fn get_glyph_height_depth(&self, gid: GlyphID) -> (f32, f32) {
        let bbox = self.get_glyph_bounds(gid).unwrap_or(GlyphBBox::zero());
        (bbox.yMax, -bbox.yMin)
    }

    pub(crate) unsafe fn get_glyph_sidebearings(&self, gid: GlyphID) -> (f32, f32) {
        let width: f32 = self.get_glyph_width(gid);
        let bbox = self.get_glyph_bounds(gid).unwrap_or(GlyphBBox::zero());
        (bbox.xMin, width - bbox.xMax)
    }

    pub(crate) unsafe fn get_glyph_ital_corr(&self, gid: GlyphID) -> f32 {
        let width: f32 = self.get_glyph_width(gid);
        let bbox = self.get_glyph_bounds(gid).unwrap_or(GlyphBBox::zero());
        if bbox.xMax > width {
            bbox.xMax - width
        } else {
            0.
        }
    }

    pub(crate) unsafe fn map_glyph_to_index(&self, glyphName: *const i8) -> GlyphID {
        FT_Get_Name_Index(self.m_ftFace, glyphName as *mut i8) as GlyphID
    }

    pub(crate) unsafe fn get_glyph_name(&self, gid: GlyphID) -> String {
        if (*self.m_ftFace).face_flags & 1 << 9 != 0 {
            static mut buffer: [i8; 256] = [0; 256];
            FT_Get_Glyph_Name(
                self.m_ftFace,
                gid as FT_UInt,
                buffer.as_mut_ptr() as FT_Pointer,
                256,
            );
            crate::c_pointer_to_str(*buffer.as_mut_ptr() as *mut i8).to_string()
        } else {
            String::new()
        }
    }

    pub(crate) unsafe fn get_first_char_code(&self) -> UChar32 {
        let mut gindex: FT_UInt = 0;
        FT_Get_First_Char(self.m_ftFace, &mut gindex) as UChar32
    }

    pub(crate) unsafe fn get_last_char_code(&self) -> UChar32 {
        let mut gindex: FT_UInt = 0;
        let mut ch: UChar32 = FT_Get_First_Char(self.m_ftFace, &mut gindex) as UChar32;
        let mut prev: UChar32 = ch;
        while gindex != 0 {
            prev = ch;
            ch = FT_Get_Next_Char(self.m_ftFace, ch as FT_ULong, &mut gindex) as UChar32
        }
        prev
    }

    pub(crate) fn get_hb_font(&self) -> *mut hb_font_t {
        self.m_hbFont
    }

    pub(crate) fn units_to_points(&self, units: f32) -> f32 {
        (units * self.m_pointSize) / (self.m_unitsPerEM as f32)
    }

    pub(crate) fn points_to_units(&self, points: f32) -> f32 {
        (points * (self.m_unitsPerEM as f32)) / self.m_pointSize
    }
}
