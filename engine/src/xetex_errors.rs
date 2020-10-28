#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::io::Write;

use crate::{help, print_cstr, print_nl_cstr};

use crate::cmd::InteractionMode;
use crate::xetex_ini::{
    error_count, file_line_error_style_p, halt_on_error_p, help_line, help_ptr, history,
    interaction, job_name, log_opened, rust_stdout, selector, use_err_help,
};
use crate::xetex_output::{print_chr, print_file_line, print_ln, Int, Nl};
use crate::xetex_xetex0::{close_files_and_terminate, give_err_help, open_log_file, show_context};

use bridge::TTHistory;

use crate::xetex_ini::{cur_input, Selector, INPUT_PTR, INPUT_STACK};

pub(crate) trait Confuse {
    type Output;
    fn confuse(self, message: &str) -> Self::Output;
}

impl<T> Confuse for Option<T> {
    type Output = T;
    fn confuse(self, message: &str) -> Self::Output {
        match self {
            Some(v) => v,
            None => unsafe { confusion(message) },
        }
    }
}

pub(crate) type str_number = i32;
/* tectonic/errors.c -- error handling
 * Copyright 2016 the Tectonic Project
 * Licensed under the MIT License.
*/
/* WEBby error-handling code: */
unsafe fn pre_error_message() {
    /* FKA normalize_selector(): */
    if log_opened {
        selector = Selector::TERM_AND_LOG
    } else {
        selector = Selector::TERM_ONLY
    }
    if job_name == 0i32 {
        open_log_file();
    }
    if interaction == InteractionMode::Batch {
        selector = (u8::from(selector) - 1).into()
    }
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr!("! ");
    };
}
/*82: */
unsafe fn post_error_message(mut need_to_print_it: i32) {
    if interaction == InteractionMode::ErrorStop {
        interaction = InteractionMode::Scroll;
    }
    if need_to_print_it != 0 && log_opened {
        error();
    }
    history = TTHistory::FATAL_ERROR;
    close_files_and_terminate();
    rust_stdout.as_mut().unwrap().flush().unwrap();
}
pub(crate) unsafe fn error() {
    if (history as u32) < (TTHistory::ERROR_ISSUED as u32) {
        history = TTHistory::ERROR_ISSUED
    }
    print_chr('.');
    INPUT_STACK[INPUT_PTR] = cur_input;
    show_context(&INPUT_STACK[..INPUT_PTR + 1]);
    if halt_on_error_p != 0 {
        history = TTHistory::FATAL_ERROR;
        post_error_message(0);
        abort!("halted on potentially-recoverable error as specified");
    }
    /* This used to be where there was a bunch of code if "interaction ==
     * error_stop_mode" that would let the use interactively try to solve the
     * error. */
    error_count += 1;
    if error_count as i32 == 100i32 {
        print_nl_cstr!("(That makes 100 errors; please try again.)");
        history = TTHistory::FATAL_ERROR;
        post_error_message(0);
        panic!("halted after 100 potentially-recoverable errors");
    }
    if interaction != InteractionMode::Batch {
        selector = (u8::from(selector) - 1).into()
    }
    if use_err_help {
        print_ln();
        give_err_help();
    } else {
        while help_ptr > 0 {
            help_ptr -= 1;
            print_nl_cstr!("{}", help_line[help_ptr as usize]);
        }
    }
    print_ln();
    if interaction != InteractionMode::Batch {
        selector = (u8::from(selector) + 1).into()
    }
    print_ln();
}
pub(crate) unsafe fn fatal_error(s: &str) -> ! {
    pre_error_message();
    print_cstr!("Emergency stop{}{}", Nl, s);
    close_files_and_terminate();
    rust_stdout.as_mut().unwrap().flush().unwrap();
    abort!("{}", s);
}
pub(crate) unsafe fn overflow(s: &str, n: usize) -> ! {
    pre_error_message();
    print_cstr!("TeX capacity exceeded, sorry [{}={}]", s, Int(n as i32));
    help!(
        "If you really absolutely need more capacity,",
        "you can ask a wizard to enlarge me."
    );
    post_error_message(1i32);
    panic!("halted on overflow()");
}
pub(crate) unsafe fn confusion(s: &str) -> ! {
    pre_error_message();
    if (history as u32) < (TTHistory::ERROR_ISSUED as u32) {
        print_cstr!("This can\'t happen ({})", s);
        help!("I\'m broken. Please show this to someone who can fix can fix");
    } else {
        print_cstr!("I can\'t go on meeting you like this");
        help!(
            "One of your faux pas seems to have wounded me deeply...",
            "in fact, I\'m barely conscious. Please fix it and try again."
        );
    }
    post_error_message(1i32);
    panic!("halted on confusion()");
}
/* xetex-errors */
pub(crate) unsafe fn pdf_error(t: &str, mut p: &str) -> ! {
    pre_error_message();
    if !t.is_empty() {
        print_cstr!("Error ({}): {}", t, p);
    } else {
        print_cstr!("Error: {}", p);
    }
    post_error_message(1i32);
    panic!("halted on pdf_error()");
}
