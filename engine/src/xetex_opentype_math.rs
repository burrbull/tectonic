/* ***************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew

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
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use harfbuzz_sys::*;
use std::ptr;

use crate::xetex_ext::{Font, NativeFont::*};
use crate::xetex_scaledmath::Scaled;

use crate::xetex_layout_interface::GlyphAssembly;
use crate::xetex_layout_interface::{D2Fix, Fix2D};

use crate::xetex_ini::{FONT_LAYOUT_ENGINE, FONT_SIZE};

pub(crate) const HB_OT_MATH_GLYPH_PART_FLAG_EXTENDER: hb_ot_math_glyph_part_flags_t = 1;

/* tectonic/xetex-core.h: core XeTeX types and #includes.
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License.
*/
// defines U_IS_BIG_ENDIAN for us
/* fontconfig */
/* freetype */
/* harfbuzz */
/* Endianness foo */
/* our typedefs */

/* ***************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew
 Copyright (c) 2012-2015 by Khaled Hosny

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
pub(crate) unsafe fn get_ot_math_constant(f: usize, n: i32) -> Scaled {
    let constant = n as hb_ot_math_constant_t;
    if let Font::Native(Otgr(e)) = &FONT_LAYOUT_ENGINE[f] {
        let font = e.get_font();
        let hbFont = font.get_hb_font();
        let rval = hb_ot_math_get_constant(hbFont, constant);
        /* scale according to font size, except the ones that are percentages */
        match constant as u32 {
            0 | 1 | 55 => Scaled::ZERO,
            _ => D2Fix(font.units_to_points(rval as f32) as f64),
        }
    } else {
        Scaled::ZERO
    }
}
/* size of \.{\\atopwithdelims} delimiters in non-displays */
/* height of fraction lines above the baseline */
#[no_mangle]
pub(crate) static mut TeX_sym_to_OT_map: [hb_ot_math_constant_t; 23] = [
    4294967295 as hb_ot_math_constant_t,
    4294967295 as hb_ot_math_constant_t,
    4294967295 as hb_ot_math_constant_t,
    4294967295 as hb_ot_math_constant_t,
    4294967295 as hb_ot_math_constant_t,
    HB_OT_MATH_CONSTANT_ACCENT_BASE_HEIGHT,
    4294967295 as hb_ot_math_constant_t,
    4294967295 as hb_ot_math_constant_t,
    HB_OT_MATH_CONSTANT_FRACTION_NUMERATOR_DISPLAY_STYLE_SHIFT_UP,
    HB_OT_MATH_CONSTANT_FRACTION_NUMERATOR_SHIFT_UP,
    HB_OT_MATH_CONSTANT_STACK_TOP_SHIFT_UP,
    HB_OT_MATH_CONSTANT_FRACTION_DENOMINATOR_DISPLAY_STYLE_SHIFT_DOWN,
    HB_OT_MATH_CONSTANT_FRACTION_DENOMINATOR_SHIFT_DOWN,
    HB_OT_MATH_CONSTANT_SUPERSCRIPT_SHIFT_UP,
    HB_OT_MATH_CONSTANT_SUPERSCRIPT_SHIFT_UP,
    HB_OT_MATH_CONSTANT_SUPERSCRIPT_SHIFT_UP_CRAMPED,
    HB_OT_MATH_CONSTANT_SUBSCRIPT_SHIFT_DOWN,
    HB_OT_MATH_CONSTANT_SUBSCRIPT_SHIFT_DOWN,
    HB_OT_MATH_CONSTANT_SUPERSCRIPT_BASELINE_DROP_MAX,
    HB_OT_MATH_CONSTANT_SUBSCRIPT_BASELINE_DROP_MIN,
    HB_OT_MATH_CONSTANT_DELIMITED_SUB_FORMULA_MIN_HEIGHT,
    4294967295 as hb_ot_math_constant_t,
    HB_OT_MATH_CONSTANT_AXIS_HEIGHT,
];

pub(crate) unsafe fn get_native_mathsy_param(f: usize, n: i32) -> Scaled {
    let mut rval = Scaled::ZERO;
    if n == 6 {
        rval = FONT_SIZE[f as usize];
    } else if n == 21 {
        // XXX not sure what OT parameter we should use here;
        // for now we use 1.5em, clamped to delim1 height
        rval = Scaled((1.5 * FONT_SIZE[f as usize].0 as f64) as i32)
            .min(get_native_mathsy_param(f, 20));
    } else if n
        < (::std::mem::size_of::<[hb_ot_math_constant_t; 23]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<hb_ot_math_constant_t>() as libc::c_ulong)
            as i32
    {
        let ot_index = TeX_sym_to_OT_map[n as usize];
        if ot_index as u32 != 4294967295 {
            rval = get_ot_math_constant(f, ot_index as i32)
        }
    }
    //  fprintf(stderr, " math_sy(%d, %d) returns %.3f\n", f, n, Fix2D(rval));
    rval
}
/* fontdimen IDs for math extension font (family 3) */
/* thickness of \.{\\over} bars */
/* minimum clearance above a displayed op */
/* minimum clearance below a displayed op */
/* minimum baselineskip above displayed op */
/* minimum baselineskip below displayed op */
/* padding above and below displayed limits */
#[no_mangle]
pub(crate) static mut TeX_ext_to_OT_map: [hb_ot_math_constant_t; 14] = [
    4294967295 as hb_ot_math_constant_t,
    4294967295 as hb_ot_math_constant_t,
    4294967295 as hb_ot_math_constant_t,
    4294967295 as hb_ot_math_constant_t,
    4294967295 as hb_ot_math_constant_t,
    HB_OT_MATH_CONSTANT_ACCENT_BASE_HEIGHT,
    4294967295 as hb_ot_math_constant_t,
    4294967295 as hb_ot_math_constant_t,
    HB_OT_MATH_CONSTANT_FRACTION_RULE_THICKNESS,
    HB_OT_MATH_CONSTANT_UPPER_LIMIT_GAP_MIN,
    HB_OT_MATH_CONSTANT_LOWER_LIMIT_GAP_MIN,
    HB_OT_MATH_CONSTANT_UPPER_LIMIT_BASELINE_RISE_MIN,
    HB_OT_MATH_CONSTANT_LOWER_LIMIT_BASELINE_DROP_MIN,
    HB_OT_MATH_CONSTANT_STACK_GAP_MIN,
];
pub(crate) unsafe fn get_native_mathex_param(f: usize, n: i32) -> Scaled {
    let mut rval = Scaled::ZERO;
    if n == 6 {
        rval = FONT_SIZE[f as usize];
    } else if n
        < (::std::mem::size_of::<[hb_ot_math_constant_t; 14]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<hb_ot_math_constant_t>() as libc::c_ulong)
            as i32
    {
        let ot_index = TeX_ext_to_OT_map[n as usize];
        if ot_index as u32 != 4294967295 {
            rval = get_ot_math_constant(f, ot_index as i32)
        }
    }
    //  fprintf(stderr, " math_ex(%d, %d) returns %.3f\n", f, n, Fix2D(rval));
    rval
}
pub(crate) unsafe fn get_ot_math_variant(
    f: usize,
    g: i32,
    v: i32,
    adv: &mut Scaled,
    horiz: i32,
) -> i32 {
    let mut rval = g as hb_codepoint_t;
    *adv = Scaled(-1);
    if let Font::Native(Otgr(e)) = &FONT_LAYOUT_ENGINE[f] {
        let font = e.get_font();
        let hbFont = font.get_hb_font();
        let mut variant: [hb_ot_math_glyph_variant_t; 1] = [hb_ot_math_glyph_variant_t {
            glyph: 0,
            advance: 0,
        }; 1];
        let mut count = 1_u32;
        hb_ot_math_get_glyph_variants(
            hbFont,
            g as hb_codepoint_t,
            (if horiz != 0 {
                HB_DIRECTION_RTL as i32
            } else {
                HB_DIRECTION_TTB as i32
            }) as hb_direction_t,
            v as u32,
            &mut count,
            variant.as_mut_ptr(),
        );
        if count > 0 {
            rval = (*variant.as_mut_ptr()).glyph;
            *adv = D2Fix(font.units_to_points((*variant.as_mut_ptr()).advance as f32) as f64)
        }
    }
    rval as i32
}
pub(crate) unsafe fn get_ot_assembly_ptr(f: usize, g: i32, horiz: i32) -> Option<GlyphAssembly> {
    let mut rval = None;
    if let Font::Native(Otgr(e)) = &FONT_LAYOUT_ENGINE[f] {
        let font = e.get_font();
        let hbFont = font.get_hb_font();
        let mut count = hb_ot_math_get_glyph_assembly(
            hbFont,
            g as hb_codepoint_t,
            (if horiz != 0 {
                HB_DIRECTION_RTL as i32
            } else {
                HB_DIRECTION_TTB as i32
            }) as hb_direction_t,
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        if count > 0 {
            let mut parts = Vec::with_capacity(count as _);
            parts.set_len(count as _);
            let mut a = GlyphAssembly { parts };
            hb_ot_math_get_glyph_assembly(
                hbFont,
                g as hb_codepoint_t,
                (if horiz != 0 {
                    HB_DIRECTION_RTL as i32
                } else {
                    HB_DIRECTION_TTB as i32
                }) as hb_direction_t,
                0,
                &mut count,
                a.parts.as_mut_ptr(),
                ptr::null_mut(),
            );
            rval = Some(a);
        }
    }
    rval
}
pub(crate) unsafe fn get_ot_math_ital_corr(f: usize, g: i32) -> Scaled {
    if let Font::Native(Otgr(e)) = &FONT_LAYOUT_ENGINE[f] {
        let font = e.get_font();
        let hbFont = font.get_hb_font();
        let rval = hb_ot_math_get_glyph_italics_correction(hbFont, g as hb_codepoint_t);
        D2Fix(font.units_to_points(rval as f32) as f64)
    } else {
        Scaled::ZERO
    }
}
pub(crate) unsafe fn get_ot_math_accent_pos(f: usize, g: i32) -> Scaled {
    if let Font::Native(Otgr(e)) = &FONT_LAYOUT_ENGINE[f] {
        let font = e.get_font();
        let hbFont = font.get_hb_font();
        let rval = hb_ot_math_get_glyph_top_accent_attachment(hbFont, g as hb_codepoint_t);
        D2Fix(font.units_to_points(rval as f32) as f64)
    } else {
        Scaled::INFINITY
    }
}
pub(crate) unsafe fn ot_min_connector_overlap(f: usize) -> Scaled {
    if let Font::Native(Otgr(e)) = &FONT_LAYOUT_ENGINE[f] {
        let font = e.get_font();
        let hbFont = font.get_hb_font();
        let rval = hb_ot_math_get_min_connector_overlap(hbFont, HB_DIRECTION_RTL);
        D2Fix(font.units_to_points(rval as f32) as f64)
    } else {
        Scaled::ZERO
    }
}
unsafe fn getMathKernAt(f: usize, g: i32, side: hb_ot_math_kern_t, height: i32) -> Scaled {
    if let Font::Native(Otgr(e)) = &FONT_LAYOUT_ENGINE[f] {
        let font = e.get_font();
        let hbFont = font.get_hb_font();
        Scaled(hb_ot_math_get_glyph_kerning(
            hbFont,
            g as hb_codepoint_t,
            side,
            height,
        ))
    } else {
        Scaled::ZERO
    }
}
unsafe fn glyph_height(f: usize, g: i32) -> f32 {
    if let Font::Native(Otgr(engine)) = &FONT_LAYOUT_ENGINE[f] {
        let (rval, _) = engine.get_glyph_height_depth(g as u32);
        rval
    } else {
        0.
    }
}
unsafe fn glyph_depth(f: usize, g: i32) -> f32 {
    if let Font::Native(Otgr(engine)) = &FONT_LAYOUT_ENGINE[f] {
        let (_, rval) = engine.get_glyph_height_depth(g as u32);
        rval
    } else {
        0.
    }
}
pub(crate) unsafe fn get_ot_math_kern(
    f: usize,
    g: i32,
    sf: usize,
    sg: i32,
    cmd: i32,
    shift: Scaled,
) -> Scaled {
    if let Font::Native(Otgr(e)) = &FONT_LAYOUT_ENGINE[f] {
        let font = e.get_font();
        let mut rval;
        if cmd == 0 {
            // superscript
            let corr_height_top = font.points_to_units(glyph_height(f, g));
            let corr_height_bot =
                -font.points_to_units((glyph_depth(sf, sg) as f64 + Fix2D(shift)) as f32);
            let kern = getMathKernAt(
                f,
                g,
                HB_OT_MATH_KERN_TOP_RIGHT,
                corr_height_top as libc::c_int,
            )
            .0;
            let skern = getMathKernAt(
                sf,
                sg,
                HB_OT_MATH_KERN_BOTTOM_LEFT,
                corr_height_top as libc::c_int,
            )
            .0;
            rval = kern + skern;
            let kern = getMathKernAt(
                f,
                g,
                HB_OT_MATH_KERN_TOP_RIGHT,
                corr_height_bot as libc::c_int,
            )
            .0;
            let skern = getMathKernAt(
                sf,
                sg,
                HB_OT_MATH_KERN_BOTTOM_LEFT,
                corr_height_bot as libc::c_int,
            )
            .0;
            if kern + skern < rval {
                rval = kern + skern
            }
        } else if cmd == 1 {
            // subscript
            let corr_height_top =
                font.points_to_units((glyph_height(sf, sg) as f64 - Fix2D(shift)) as f32);
            let corr_height_bot = -font.points_to_units(glyph_depth(f, g));
            let kern = getMathKernAt(
                f,
                g,
                HB_OT_MATH_KERN_BOTTOM_RIGHT,
                corr_height_top as libc::c_int,
            )
            .0;
            let skern = getMathKernAt(
                sf,
                sg,
                HB_OT_MATH_KERN_TOP_LEFT,
                corr_height_top as libc::c_int,
            )
            .0;
            rval = kern + skern;
            let kern = getMathKernAt(
                f,
                g,
                HB_OT_MATH_KERN_BOTTOM_RIGHT,
                corr_height_bot as libc::c_int,
            )
            .0;
            let skern = getMathKernAt(
                sf,
                sg,
                HB_OT_MATH_KERN_TOP_LEFT,
                corr_height_bot as libc::c_int,
            )
            .0;
            if kern + skern < rval {
                rval = kern + skern
            }
        } else {
            unreachable!()
            // we should not reach here
        }
        D2Fix(font.units_to_points(rval as f32) as f64)
    } else {
        Scaled::ZERO
    }
}
pub(crate) fn ot_part_count(a: &GlyphAssembly) -> i32 {
    a.parts.len() as i32
}
pub(crate) unsafe fn ot_part_glyph(a: &GlyphAssembly, i: i32) -> i32 {
    a.parts[i as usize].glyph as i32
}
pub(crate) unsafe fn ot_part_is_extender(a: &GlyphAssembly, i: i32) -> bool {
    a.parts[i as usize].flags as u32 & HB_OT_MATH_GLYPH_PART_FLAG_EXTENDER as u32 != 0
}
pub(crate) unsafe fn ot_part_start_connector(f: usize, a: &GlyphAssembly, i: i32) -> Scaled {
    if let Font::Native(Otgr(e)) = &FONT_LAYOUT_ENGINE[f] {
        let font = e.get_font();
        D2Fix(font.units_to_points(a.parts[i as usize].start_connector_length as f32) as f64)
    } else {
        Scaled::ZERO
    }
}
pub(crate) unsafe fn ot_part_end_connector(f: usize, a: &GlyphAssembly, i: i32) -> Scaled {
    if let Font::Native(Otgr(e)) = &FONT_LAYOUT_ENGINE[f] {
        let font = e.get_font();
        D2Fix(font.units_to_points(a.parts[i as usize].end_connector_length as f32) as f64)
    } else {
        Scaled::ZERO
    }
}
pub(crate) unsafe fn ot_part_full_advance(f: usize, a: &GlyphAssembly, i: i32) -> Scaled {
    if let Font::Native(Otgr(e)) = &FONT_LAYOUT_ENGINE[f as usize] {
        let font = e.get_font();
        D2Fix(font.units_to_points(a.parts[i as usize].full_advance as f32) as f64)
    } else {
        Scaled::ZERO
    }
}
