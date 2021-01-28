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
#![allow(non_camel_case_types, non_snake_case)]

use std::ptr;

use crate::warn;

use super::{SpcArg, SpcEnv};
use crate::dpx_pdfdraw::pdf_dev_concat;
use crate::dpx_pdfximage::pdf_ximage_findresource;
use bridge::{InFile, TTInputFormat};

use super::util::spc_util_read_dimtrns;
use crate::dpx_mpost::{mps_eop_cleanup, mps_exec_inline, mps_stack_depth};
use crate::dpx_pdfdev::{pdf_dev_put_image, transform_info, transform_info_clear, TMatrix};
use crate::dpx_pdfdraw::{
    pdf_dev_current_depth, pdf_dev_grestore, pdf_dev_grestore_to, pdf_dev_gsave,
};
use crate::dpx_pdfparse::SkipWhite;
use crate::spc_warn;

/* quasi-hack to get the primary input */

use super::SpcHandler;

use crate::dpx_pdfximage::load_options;
static mut BLOCK_PENDING: i32 = 0;
static mut PENDING_X: f64 = 0.0f64;
static mut PENDING_Y: f64 = 0.0f64;
static mut POSITION_SET: i32 = 0;
static mut PS_HEADERS: Vec<String> = Vec::new();

unsafe fn spc_handler_ps_header(spe: &mut SpcEnv, args: &mut SpcArg) -> i32 {
    args.cur.skip_white();
    if args.cur.len() <= 1 || args.cur[0] != b'=' {
        spc_warn!(spe, "No filename specified for PSfile special.");
        return -1;
    }
    args.cur = &args.cur[1..];
    let pro = String::from_utf8_lossy(args.cur).to_string();
    if InFile::open(&pro, TTInputFormat::TEX_PS_HEADER, 0).is_none() {
        spc_warn!(spe, "PS header {} not found.", pro);
        return -1;
    }
    PS_HEADERS.push(pro);
    args.cur = &[];
    0
}
unsafe fn parse_filename<'a>(pp: &mut &'a [u8]) -> Option<&'a str> {
    let mut p = *pp;
    let qchar;
    if p.is_empty() {
        return None;
    } else {
        if p[0] == b'\"' || p[0] == b'\'' {
            qchar = p[0];
            p = &p[1..];
        } else {
            qchar = b' ';
        }
    }
    let mut n = 0;
    let q = p;
    while !p.is_empty() && p[0] != qchar {
        /* nothing */
        n += 1;
        p = &p[1..];
    }
    if qchar != b' ' {
        if p[0] != qchar {
            return None;
        }
        p = &p[1..];
    }
    if q.is_empty() || n == 0 {
        return None;
    }
    let r = Some(std::str::from_utf8(&q[..n]).unwrap());
    *pp = p;
    r
}
/* =filename ... */
unsafe fn spc_handler_ps_file(spe: &mut SpcEnv, args: &mut SpcArg) -> i32 {
    let options: load_options = load_options {
        page_no: 1,
        bbox_type: 0,
        dict: ptr::null_mut(),
    };
    args.cur.skip_white();
    if args.cur.len() <= 1 || args.cur[0] != b'=' {
        spc_warn!(spe, "No filename specified for PSfile special.");
        return -1;
    }
    args.cur = &args.cur[1..];
    if let Some(filename) = parse_filename(&mut args.cur) {
        let mut ti = if let Ok(ti) = spc_util_read_dimtrns(spe, args, 1) {
            ti
        } else {
            return -1;
        };
        let form_id = pdf_ximage_findresource(filename, options);
        if form_id < 0 {
            spc_warn!(spe, "Failed to read image file: {}", filename);
            return -1;
        }
        pdf_dev_put_image(form_id, &mut ti, spe.x_user, spe.y_user);
        0
    } else {
        spc_warn!(spe, "No filename specified for PSfile special.");
        -1
    }
}
/* This isn't correct implementation but dvipdfm supports... */
unsafe fn spc_handler_ps_plotfile(spe: &mut SpcEnv, args: &mut SpcArg) -> i32 {
    let mut error: i32 = 0; /* xscale = 1.0, yscale = -1.0 */
    let mut p = transform_info::new();
    let options: load_options = load_options {
        page_no: 1,
        bbox_type: 0,
        dict: ptr::null_mut(),
    };
    spc_warn!(spe, "\"ps: plotfile\" found (not properly implemented)");
    args.cur.skip_white();
    if let Some(filename) = parse_filename(&mut args.cur) {
        let form_id = pdf_ximage_findresource(filename, options);
        if form_id < 0 {
            spc_warn!(spe, "Could not open PS file: {}", filename);
            error = -1;
        } else {
            transform_info_clear(&mut p);
            p.matrix.m22 = -1.;
            pdf_dev_put_image(form_id, &mut p, 0 as f64, 0 as f64);
        }
        error
    } else {
        spc_warn!(spe, "Expecting filename but not found...");
        return -1;
    }
}
unsafe fn spc_handler_ps_literal(spe: &mut SpcEnv, args: &mut SpcArg) -> i32 {
    let mut error: i32 = 0;
    let x_user;
    let y_user;
    assert!(!args.cur.is_empty());
    if args.cur.starts_with(b":[begin]") {
        BLOCK_PENDING += 1;
        POSITION_SET = 1;
        PENDING_X = spe.x_user;
        x_user = PENDING_X;
        PENDING_Y = spe.y_user;
        y_user = PENDING_Y;
        args.cur = &args.cur[b":[begin]".len()..];
    } else if args.cur.starts_with(b":[end]") {
        if BLOCK_PENDING <= 0 {
            spc_warn!(spe, "No corresponding ::[begin] found.");
            return -1;
        }
        BLOCK_PENDING -= 1;
        POSITION_SET = 0;
        x_user = PENDING_X;
        y_user = PENDING_Y;
        args.cur = &args.cur[b":[end]".len()..];
    } else if !args.cur.is_empty() && args.cur[0] == b':' {
        x_user = if POSITION_SET != 0 {
            PENDING_X
        } else {
            spe.x_user
        };
        y_user = if POSITION_SET != 0 {
            PENDING_Y
        } else {
            spe.y_user
        };
        args.cur = &args.cur[1..];
    } else {
        POSITION_SET = 1;
        PENDING_X = spe.x_user;
        x_user = PENDING_X;
        PENDING_Y = spe.y_user;
        y_user = PENDING_Y
    }
    args.cur.skip_white();
    if !args.cur.is_empty() {
        let st_depth = mps_stack_depth();
        let gs_depth = pdf_dev_current_depth();
        error = mps_exec_inline(&mut args.cur, x_user, y_user);
        if error != 0 {
            spc_warn!(
                spe,
                "Interpreting PS code failed!!! Output might be broken!!!"
            );
            pdf_dev_grestore_to(gs_depth);
        } else if st_depth != mps_stack_depth() {
            spc_warn!(
                spe,
                "Stack not empty after execution of inline PostScript code."
            );
            spc_warn!(
                spe,
                ">> Your macro package makes some assumption on internal behaviour of DVI drivers."
            );
            spc_warn!(spe, ">> It may not compatible with dvipdfmx.");
        }
    }
    error
}
unsafe fn spc_handler_ps_trickscmd(_spe: &mut SpcEnv, args: &mut SpcArg) -> i32 {
    warn!("PSTricks commands are disallowed in Tectonic");
    args.cur = &[];
    -1
}
unsafe fn spc_handler_ps_tricksobj(_spe: &mut SpcEnv, args: &mut SpcArg) -> i32 {
    warn!("PSTricks commands are disallowed in Tectonic");
    args.cur = &[];
    -1
}
unsafe fn spc_handler_ps_default(spe: &mut SpcEnv, args: &mut SpcArg) -> i32 {
    pdf_dev_gsave();
    let st_depth = mps_stack_depth();
    let gs_depth = pdf_dev_current_depth();
    let mut M = TMatrix::create_translation(spe.x_user, spe.y_user);
    pdf_dev_concat(&mut M);
    let error = mps_exec_inline(&mut args.cur, spe.x_user, spe.y_user);
    M.m31 = -spe.x_user;
    M.m32 = -spe.y_user;
    pdf_dev_concat(&mut M);
    if error != 0 {
        spc_warn!(
            spe,
            "Interpreting PS code failed!!! Output might be broken!!!"
        );
    } else if st_depth != mps_stack_depth() {
        spc_warn!(
            spe,
            "Stack not empty after execution of inline PostScript code."
        );
        spc_warn!(
            spe,
            ">> Your macro package makes some assumption on internal behaviour of DVI drivers."
        );
        spc_warn!(spe, ">> It may not compatible with dvipdfmx.");
    }
    pdf_dev_grestore_to(gs_depth);
    pdf_dev_grestore();
    error
}
const DVIPS_HANDLERS: [SpcHandler; 10] = [
    SpcHandler {
        key: "header",
        exec: Some(spc_handler_ps_header),
    },
    SpcHandler {
        key: "PSfile",
        exec: Some(spc_handler_ps_file),
    },
    SpcHandler {
        key: "psfile",
        exec: Some(spc_handler_ps_file),
    },
    SpcHandler {
        key: "ps: plotfile ",
        exec: Some(spc_handler_ps_plotfile),
    },
    SpcHandler {
        key: "PS: plotfile ",
        exec: Some(spc_handler_ps_plotfile),
    },
    SpcHandler {
        key: "PS:",
        exec: Some(spc_handler_ps_literal),
    },
    SpcHandler {
        key: "ps:",
        exec: Some(spc_handler_ps_literal),
    },
    SpcHandler {
        key: "PST:",
        exec: Some(spc_handler_ps_trickscmd),
    },
    SpcHandler {
        key: "pst:",
        exec: Some(spc_handler_ps_tricksobj),
    },
    SpcHandler {
        key: "\" ",
        exec: Some(spc_handler_ps_default),
    },
];

pub(crate) unsafe fn spc_dvips_at_begin_document() -> i32 {
    /* This function used to start the global_defs temp file. */
    0
}

pub(crate) unsafe fn spc_dvips_at_end_document() -> i32 {
    PS_HEADERS.clear();
    0
}

pub(crate) unsafe fn spc_dvips_at_begin_page() -> i32 {
    /* This function used do some things related to now-removed PSTricks functionality. */
    0
}

pub(crate) unsafe fn spc_dvips_at_end_page() -> i32 {
    mps_eop_cleanup();
    0
}
pub(crate) fn spc_dvips_check_special(mut buf: &[u8]) -> bool {
    buf.skip_white();
    if buf.is_empty() {
        return false;
    }
    for handler in DVIPS_HANDLERS.iter() {
        if buf.starts_with(handler.key.as_bytes()) {
            return true;
        }
    }
    false
}

pub(crate) unsafe fn spc_dvips_setup_handler(
    handle: &mut SpcHandler,
    spe: &mut SpcEnv,
    args: &mut SpcArg,
) -> i32 {
    args.cur.skip_white();
    let key = args.cur;
    while !args.cur.is_empty() && (args.cur[0] as u8).is_ascii_alphabetic() {
        args.cur = &args.cur[1..];
    }
    /* Test for "ps:". The "ps::" special is subsumed under this case.  */
    if !args.cur.is_empty() && args.cur[0] == b':' {
        args.cur = &args.cur[1..];
        if args.cur.starts_with(b" plotfile ") {
            args.cur = &args.cur[b" plotfile ".len()..];
        }
    } else if args.cur.len() > 1 && args.cur[0] == b'\"' && args.cur[1] == b' ' {
        args.cur = &args.cur[2..];
    }
    let keylen = key.len() - args.cur.len();
    if keylen < 1 {
        spc_warn!(spe, "Not ps: special???");
        return -1;
    }
    for handler in DVIPS_HANDLERS.iter() {
        if &key[..keylen] == handler.key.as_bytes() {
            args.cur.skip_white();
            args.command = Some(handler.key);
            *handle = SpcHandler {
                key: "ps:",
                exec: handler.exec,
            };
            return 0;
        }
    }
    -1
}
