#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![allow(clippy::many_single_char_names)]

extern crate tectonic_bridge as bridge;

use std::io::Write;

use crate::core_memory::{xmalloc, xmalloc_array, xrealloc};

use std::slice;

mod core_memory {
    use bridge::size_t;
    pub(crate) unsafe fn xmalloc(mut size: size_t) -> *mut libc::c_void {
        let size = size as libc::size_t; //FIXME

        let mut new_mem: *mut libc::c_void = libc::malloc(if size != 0 { size } else { 1 });
        if new_mem.is_null() {
            panic!("xmalloc request for {} bytes failed", size,);
        }
        new_mem
    }
    pub(crate) unsafe fn xrealloc(
        mut old_ptr: *mut libc::c_void,
        mut size: size_t,
    ) -> *mut libc::c_void {
        let size = size as libc::size_t; //FIXME
        let mut new_mem: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
        if old_ptr.is_null() {
            new_mem = xmalloc(size as size_t)
        } else {
            new_mem = libc::realloc(old_ptr, if size != 0 { size } else { 1 });
            if new_mem.is_null() {
                panic!("xrealloc() to {} bytes failed", size,);
            }
        }
        new_mem
    }

    #[inline]
    pub(crate) unsafe fn xmalloc_array<T>(size: usize) -> *mut T {
        xmalloc(((size + 1) * std::mem::size_of::<T>()) as _) as *mut T
    }
}

use bridge::{
    ttstub_input_getc, ttstub_output_close, ttstub_output_open, ttstub_output_open_stdout,
    ttstub_output_putc, InFile, OutputHandleWrapper, TTHistory, TTInputFormat,
};
use libc::{free, strcpy, strlen};
use std::panic;
use std::ptr;

pub(crate) type str_number = i32;
/*22: */
pub(crate) type pool_pointer = i32;
pub(crate) type bib_number = usize;

#[repr(C)]
pub(crate) struct peekable_input_t {
    pub(crate) handle: InFile,
    pub(crate) peek_char: i32,
    pub(crate) saw_eof: bool,
}
pub(crate) type buf_pointer = i32;
pub(crate) type buf_type = *mut u8;
pub(crate) type hash_loc = i32;
pub(crate) type str_ilk = u8;
pub(crate) type hash_pointer = i32;
pub(crate) type cite_number = i32;
pub(crate) type str_ent_loc = i32;
pub(crate) type lit_stk_loc = i32;
pub(crate) type int_ent_loc = i32;
pub(crate) type field_loc = i32;
pub(crate) type wiz_fn_loc = i32;
pub(crate) type hash_ptr2 = i32;
pub(crate) type fn_def_loc = i32;
pub(crate) type aux_number = i32;
pub(crate) type pds_len = u8;
pub(crate) type pds_type = *const i8;
pub(crate) type blt_in_range = i32;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum IdType {
    Illegal,
    Legal,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum StkType {
    Int,
    Str,
    Fn,
    FieldMissing,
    Empty,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum FnClass {
    BuiltIn,
    WizDefined,
    IntLiteral,
    StrLiteral,
    Field,
    IntEntryVar,
    StrEntryVar,
    IntGlobalVar,
    StrGlobalVar,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ScanResult {
    IdNull,
    SpecifiedCharAdjacent,
    OtherCharAdjacent,
    WhiteAdjacent,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum LexType {
    /// The unrecognized |ASCII_code|s
    Illegal,
    /// Things like |space|s that you can't see
    WhiteSpace,
    /// The upper- and lower-case letters
    Alpha,
    /// The ten digits
    Numeric,
    /// Things sometimes treated like |white_space|
    SepChar,
    /// When none of the above applies
    OtherLex,
}

const hash_base: i32 = 1;
const quote_next_fn: i32 = hash_base - 1;
const BUF_SIZE: i32 = 20000;
const min_print_line: i32 = 3;
const max_print_line: i32 = 79;
const aux_stack_size: i32 = 20;
const MAX_BIBFILES: usize = 20;
const POOL_SIZE: i32 = 65000;
const MAX_STRINGS: i32 = 35307;
const MAX_CITES: i32 = 750;
const WIZ_FN_SPACE: i32 = 3000;
const SINGLE_FN_SPACE: i32 = 100;
const ENT_STR_SIZE: i32 = 250;
const GLOB_STR_SIZE: i32 = 20000;
const MAX_GLOB_STRS: i32 = 10;
const MAX_FIELDS: i32 = 17250;
const LIT_STK_SIZE: i32 = 100;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum ConversionType {
    TitleLowers,
    AllLowers,
    AllUppers,
    BadConversion,
}

unsafe fn peekable_open(
    mut path: *const i8,
    mut format: TTInputFormat,
) -> Option<peekable_input_t> {
    InFile::open(
        std::ffi::CStr::from_ptr(path).to_str().unwrap(),
        format,
        0i32,
    )
    .map(|handle| peekable_input_t {
        handle,
        peek_char: -1,
        saw_eof: false,
    })
}
unsafe fn peekable_getc(peekable: &mut peekable_input_t) -> i32 {
    let mut rv: i32 = 0;
    if peekable.peek_char != -1i32 {
        rv = peekable.peek_char;
        peekable.peek_char = -1i32;
        return rv;
    }
    rv = ttstub_input_getc(&mut peekable.handle);
    if rv == -1i32 {
        peekable.saw_eof = true
    }
    rv
}
unsafe fn peekable_ungetc(peekable: &mut peekable_input_t, mut c: i32) {
    /* TODO: assert c != EOF */
    peekable.peek_char = c;
}
/* eofeoln.c, adapted for Rusty I/O */
unsafe fn tectonic_eof(peekable: Option<&mut peekable_input_t>) -> bool {
    /* Check for EOF following Pascal semantics. */
    let mut c: i32 = 0;
    if peekable.is_none() {
        return true;
    }
    let peekable = peekable.unwrap();
    if peekable.saw_eof {
        return true;
    }
    c = peekable_getc(peekable);
    if c == -1i32 {
        return true;
    }
    peekable_ungetc(peekable, c);
    false
}
unsafe fn eoln(peekable: &mut peekable_input_t) -> bool {
    let mut c: i32 = 0;
    if peekable.saw_eof {
        return true;
    }
    c = peekable_getc(peekable);
    if c != -1i32 {
        peekable_ungetc(peekable, c);
    }
    c == '\n' as i32 || c == '\r' as i32 || c == -1i32
}

lazy_static::lazy_static! {
    static ref id_class: [IdType; 256] = {
        let mut ic = [IdType::Legal; 256];
        for i in 0..32 {
            ic[i] = IdType::Illegal;
        }
        ic[32] = IdType::Illegal;
        ic[9] = IdType::Illegal;
        ic[34] = IdType::Illegal;
        ic[35] = IdType::Illegal;
        ic[37] = IdType::Illegal;
        ic[39] = IdType::Illegal;
        ic[40] = IdType::Illegal;
        ic[41] = IdType::Illegal;
        ic[44] = IdType::Illegal;
        ic[61] = IdType::Illegal;
        ic[123] = IdType::Illegal;
        ic[125] = IdType::Illegal;
        ic
    };

    static ref lex_class: [LexType; 256] = {
        let mut lc = [LexType::Illegal; 256];
        for i in 0..128 {
            lc[i] = LexType::OtherLex;
        }
        for i in 128..256 {
            lc[i] = LexType::Alpha;
        }
        for i in 0..32 {
            lc[i] = LexType::Illegal;
        }
        lc[127] = LexType::Illegal;
        lc[9] = LexType::WhiteSpace;
        lc[13] = LexType::WhiteSpace;
        lc[b' ' as usize] = LexType::WhiteSpace;
        lc[b'~' as usize] = LexType::SepChar;
        lc[b'-' as usize] = LexType::SepChar;
        for i in 48..58 {
            lc[i] = LexType::Numeric;
        }
        for i in 65..91 {
            lc[i] = LexType::Alpha;
        }
        for i in 97..123 {
            lc[i] = LexType::Alpha;
        }
        lc
    };
}

static mut standard_output: Option<OutputHandleWrapper> = None;
static mut pool_size: i32 = 0;
static mut MAX_BIB_FILES: usize = 0;
static mut max_cites: i32 = 0;
static mut wiz_fn_space: i32 = 0;
static mut ent_str_size: i32 = 0;
static mut glob_str_size: i32 = 0;
static mut max_glob_strs: i32 = 0;
static mut max_fields: i32 = 0;
static mut lit_stk_size: i32 = 0;
static mut max_strings: i32 = 0;
static mut hash_size: i32 = 0;
static mut hash_prime: i32 = 0;
static mut hash_max: i32 = 0;
static mut end_of_def: i32 = 0;
static mut undefined: i32 = 0;
/*fatal_message */
static mut history: TTHistory = TTHistory::SPOTLESS;
static mut err_count: i32 = 0;
static mut char_width: [i32; 256] = [0; 256];
static mut string_width: i32 = 0;
static mut name_of_file: *mut u8 = ptr::null_mut();
static mut name_length: i32 = 0;
static mut name_ptr: i32 = 0;
static mut buf_size: i32 = 0;
static mut buffer: buf_type = ptr::null_mut();
static mut last: buf_pointer = 0;
static mut sv_buffer: buf_type = ptr::null_mut();
static mut str_pool: *mut u8 = ptr::null_mut();
static mut str_start: *mut pool_pointer = ptr::null_mut();
static mut pool_ptr: pool_pointer = 0;
static mut str_ptr: str_number = 0;
static mut hash_next: *mut hash_pointer = ptr::null_mut();
static mut hash_text: *mut str_number = ptr::null_mut();
static mut hash_ilk: *mut str_ilk = ptr::null_mut();
static mut ilk_info: *mut i32 = ptr::null_mut();
static mut hash_used: i32 = 0;
static mut hash_found: bool = false;
static mut dummy_loc: hash_loc = 0;
static mut s_aux_extension: str_number = 0;
static mut s_log_extension: str_number = 0;
static mut s_bbl_extension: str_number = 0;
static mut s_bst_extension: str_number = 0;
static mut s_bib_extension: str_number = 0;
static mut s_bst_area: str_number = 0;
static mut s_bib_area: str_number = 0;
static mut pre_def_loc: hash_loc = 0;
static mut command_num: i32 = 0;
static mut buf_ptr1: buf_pointer = 0;
static mut buf_ptr2: buf_pointer = 0;
static mut aux_name_length: i32 = 0;
static mut aux_file: [Option<peekable_input_t>; 21] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None,
];
static mut aux_list: [str_number; 21] = [0; 21];
static mut aux_ptr: aux_number = 0;
static mut aux_ln_stack: [i32; 21] = [0; 21];
static mut top_lev_str: str_number = 0;
static mut log_file: Option<OutputHandleWrapper> = None;
static mut bbl_file: Option<OutputHandleWrapper> = None;
static mut bib_list: *mut str_number = ptr::null_mut();
static mut bib_ptr: bib_number = 0;
static mut num_bib_files: bib_number = 0;
static mut bib_seen: bool = false;
static mut bib_file: Vec<Option<peekable_input_t>> = Vec::new();
static mut bst_seen: bool = false;
static mut bst_str: str_number = 0;
static mut bst_file: Option<peekable_input_t> = None;
static mut cite_list: *mut str_number = ptr::null_mut();
static mut cite_ptr: cite_number = 0;
static mut entry_cite_ptr: cite_number = 0;
static mut num_cites: cite_number = 0;
static mut old_num_cites: cite_number = 0;
static mut citation_seen: bool = false;
static mut cite_loc: hash_loc = 0;
static mut lc_cite_loc: hash_loc = 0;
static mut lc_xcite_loc: hash_loc = 0;
static mut all_entries: bool = false;
static mut all_marker: cite_number = 0;
static mut bbl_line_num: i32 = 0;
static mut bst_line_num: i32 = 0;
static mut fn_loc: hash_loc = 0;
static mut wiz_loc: hash_loc = 0;
static mut literal_loc: hash_loc = 0;
static mut macro_name_loc: hash_loc = 0;
static mut macro_def_loc: hash_loc = 0;
static mut fn_type: *mut FnClass = ptr::null_mut();
static mut wiz_def_ptr: wiz_fn_loc = 0;
static mut wiz_functions: *mut hash_ptr2 = ptr::null_mut();
static mut int_ent_ptr: int_ent_loc = 0;
static mut entry_ints: *mut i32 = ptr::null_mut();
static mut num_ent_ints: int_ent_loc = 0;
static mut str_ent_ptr: str_ent_loc = 0;
static mut entry_strs: *mut u8 = ptr::null_mut();
static mut num_ent_strs: str_ent_loc = 0;
static mut str_glb_ptr: i32 = 0;
static mut glb_str_ptr: *mut str_number = ptr::null_mut();
static mut global_strs: *mut u8 = ptr::null_mut();
static mut glb_str_end: *mut i32 = ptr::null_mut();
static mut num_glb_strs: i32 = 0;
static mut field_ptr: field_loc = 0;
static mut field_parent_ptr: field_loc = 0;
static mut field_end_ptr: field_loc = 0;
static mut cite_parent_ptr: cite_number = 0;
static mut cite_xptr: cite_number = 0;
static mut field_info: *mut str_number = ptr::null_mut();
static mut num_fields: field_loc = 0;
static mut num_pre_defined_fields: field_loc = 0;
static mut crossref_num: field_loc = 0;
static mut entry_seen: bool = false;
static mut read_seen: bool = false;
static mut read_performed: bool = false;
static mut reading_completed: bool = false;
static mut read_completed: bool = false;
static mut impl_fn_num: i32 = 0;
static mut bib_line_num: i32 = 0;
static mut entry_type_loc: hash_loc = 0;
static mut type_list: *mut hash_ptr2 = ptr::null_mut();
static mut type_exists: bool = false;
static mut entry_exists: *mut bool = ptr::null_mut();
static mut store_entry: bool = false;
static mut field_name_loc: hash_loc = 0;
static mut field_val_loc: hash_loc = 0;
static mut store_field: bool = false;
static mut right_outer_delim: u8 = 0;
static mut at_bib_command: bool = false;
static mut cur_macro_loc: hash_loc = 0;
static mut cite_info: *mut str_number = ptr::null_mut();
static mut cite_hash_found: bool = false;
static mut preamble_ptr: bib_number = 0;
static mut num_preamble_strings: bib_number = 0;
static mut bib_brace_level: i32 = 0;
static mut lit_stack: *mut i32 = ptr::null_mut();
static mut lit_stk_type: *mut StkType = ptr::null_mut();
static mut lit_stk_ptr: lit_stk_loc = 0;
static mut cmd_str_ptr: str_number = 0;
static mut ent_chr_ptr: i32 = 0;
static mut glob_chr_ptr: i32 = 0;
static mut ex_buf: buf_type = ptr::null_mut();
static mut ex_buf_ptr: buf_pointer = 0;
static mut ex_buf_length: buf_pointer = 0;
static mut out_buf: buf_type = ptr::null_mut();
static mut out_buf_ptr: buf_pointer = 0;
static mut out_buf_length: buf_pointer = 0;
static mut mess_with_entries: bool = false;
static mut sort_cite_ptr: cite_number = 0;
static mut sort_key_num: str_ent_loc = 0;
static mut brace_level: i32 = 0;
static mut b_equals: hash_loc = 0;
static mut b_greater_than: hash_loc = 0;
static mut b_less_than: hash_loc = 0;
static mut b_plus: hash_loc = 0;
static mut b_minus: hash_loc = 0;
static mut b_concatenate: hash_loc = 0;
static mut b_gets: hash_loc = 0;
static mut b_add_period: hash_loc = 0;
static mut b_call_type: hash_loc = 0;
static mut b_change_case: hash_loc = 0;
static mut b_chr_to_int: hash_loc = 0;
static mut b_cite: hash_loc = 0;
static mut b_duplicate: hash_loc = 0;
static mut b_empty: hash_loc = 0;
static mut b_format_name: hash_loc = 0;
static mut b_if: hash_loc = 0;
static mut b_int_to_chr: hash_loc = 0;
static mut b_int_to_str: hash_loc = 0;
static mut b_missing: hash_loc = 0;
static mut b_newline: hash_loc = 0;
static mut b_num_names: hash_loc = 0;
static mut b_pop: hash_loc = 0;
static mut b_preamble: hash_loc = 0;
static mut b_purify: hash_loc = 0;
static mut b_quote: hash_loc = 0;
static mut b_skip: hash_loc = 0;
static mut b_stack: hash_loc = 0;
static mut b_substring: hash_loc = 0;
static mut b_swap: hash_loc = 0;
static mut b_text_length: hash_loc = 0;
static mut b_text_prefix: hash_loc = 0;
static mut b_top_stack: hash_loc = 0;
static mut b_type: hash_loc = 0;
static mut b_warning: hash_loc = 0;
static mut b_while: hash_loc = 0;
static mut b_width: hash_loc = 0;
static mut b_write: hash_loc = 0;
static mut b_default: hash_loc = 0;
static mut s_null: str_number = 0;
static mut s_default: str_number = 0;
static mut s_preamble: *mut str_number = ptr::null_mut();
static mut pop_lit1: i32 = 0;
static mut pop_lit2: i32 = 0;
static mut pop_lit3: i32 = 0;
static mut pop_typ1: StkType = StkType::Int;
static mut pop_typ2: StkType = StkType::Int;
static mut pop_typ3: StkType = StkType::Int;
static mut sp_ptr: pool_pointer = 0;
static mut sp_xptr1: pool_pointer = 0;
static mut sp_xptr2: pool_pointer = 0;
static mut sp_end: pool_pointer = 0;
static mut sp_length: pool_pointer = 0;
static mut sp2_length: pool_pointer = 0;
static mut sp_brace_level: i32 = 0;
static mut ex_buf_xptr: buf_pointer = 0;
static mut ex_buf_yptr: buf_pointer = 0;
static mut control_seq_loc: hash_loc = 0;
static mut num_names: i32 = 0;
static mut name_bf_ptr: buf_pointer = 0;
static mut name_bf_xptr: buf_pointer = 0;
static mut name_bf_yptr: buf_pointer = 0;
static mut name_tok: *mut buf_pointer = ptr::null_mut();
static mut name_sep_char: *mut u8 = ptr::null_mut();
static mut first_start: buf_pointer = 0;
static mut first_end: buf_pointer = 0;
static mut last_end: buf_pointer = 0;
static mut von_start: buf_pointer = 0;
static mut von_end: buf_pointer = 0;
static mut jr_end: buf_pointer = 0;
static mut verbose: bool = false;

pub struct BibtexConfig {
    pub min_crossrefs: i32,
}
/*:473*/
/*12: *//*3: */

unsafe fn putc_log(c: i32) {
    ttstub_output_putc(log_file.as_mut().unwrap(), c); /* note: global! */
    ttstub_output_putc(standard_output.as_mut().unwrap(), c);
}

macro_rules! log {
    ($($arg:tt)*) => {{
        use std::io::Write;
        log_file.as_mut().unwrap().write_fmt(format_args!($($arg)*)).unwrap();
        standard_output.as_mut().unwrap().write_fmt(format_args!($($arg)*)).unwrap();
    }}
}

unsafe fn mark_warning() {
    if history == TTHistory::WARNING_ISSUED {
        err_count += 1
    } else if history == TTHistory::SPOTLESS {
        history = TTHistory::WARNING_ISSUED;
        err_count = 1i32
    };
}
unsafe fn mark_error() {
    if (history as i32) < (TTHistory::ERROR_ISSUED as i32) {
        history = TTHistory::ERROR_ISSUED;
        err_count = 1i32
    } else {
        err_count += 1
    };
}
unsafe fn mark_fatal() {
    history = TTHistory::FATAL_ERROR;
}
unsafe fn print_overflow() {
    log!("Sorry---you\'ve exceeded BibTeX\'s ");
    mark_fatal();
}
unsafe fn print_confusion() {
    log!("---this can\'t happen\n");
    log!("*Please notify the BibTeX maintainer*\n");
    mark_fatal();
}
unsafe fn buffer_overflow() {
    buffer = xrealloc(
        buffer as *mut libc::c_void,
        ((buf_size + 20000i32 + 1i32) as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as _,
    ) as *mut u8;
    sv_buffer = xrealloc(
        sv_buffer as *mut libc::c_void,
        ((buf_size + 20000i32 + 1i32) as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as _,
    ) as *mut u8;
    ex_buf = xrealloc(
        ex_buf as *mut libc::c_void,
        ((buf_size + 20000i32 + 1i32) as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as _,
    ) as *mut u8;
    out_buf = xrealloc(
        out_buf as *mut libc::c_void,
        ((buf_size + 20000i32 + 1i32) as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as _,
    ) as *mut u8;
    name_tok = xrealloc(
        name_tok as *mut libc::c_void,
        ((buf_size + 20000i32 + 1i32) as u64)
            .wrapping_mul(::std::mem::size_of::<buf_pointer>() as u64) as _,
    ) as *mut buf_pointer;
    name_sep_char = xrealloc(
        name_sep_char as *mut libc::c_void,
        ((buf_size + 20000i32 + 1i32) as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as _,
    ) as *mut u8;
    buf_size += 20000i32;
}
unsafe fn input_ln(peekable: &mut Option<peekable_input_t>) -> bool {
    last = 0i32;
    if tectonic_eof(peekable.as_mut()) {
        return false;
    }
    let peekable = peekable.as_mut().unwrap();
    while !eoln(peekable) {
        if last >= buf_size {
            buffer_overflow();
        }
        *buffer.offset(last as isize) = peekable_getc(peekable) as u8;
        last += 1
    }
    peekable_getc(peekable);
    while last > 0i32 {
        if !(lex_class[*buffer.offset((last - 1i32) as isize) as usize] == LexType::WhiteSpace) {
            break;
        }
        /*white_space */
        last -= 1
    }
    true
}
unsafe fn out_pool_str(handle: &mut OutputHandleWrapper, mut s: str_number) {
    let mut i: pool_pointer = 0;
    if s < 0i32 || s >= str_ptr + 3i32 || s >= max_strings {
        log!("Illegal string number:{}", s);
        print_confusion();
        panic!();
    }
    i = *str_start.offset(s as isize);
    while i < *str_start.offset((s + 1i32) as isize) {
        ttstub_output_putc(handle, *str_pool.offset(i as isize) as i32);
        i += 1
    }
}
unsafe fn print_a_pool_str(mut s: str_number) {
    out_pool_str(standard_output.as_mut().unwrap(), s);
    out_pool_str(log_file.as_mut().unwrap(), s);
}
unsafe fn pool_overflow() {
    str_pool = xrealloc(
        str_pool as *mut libc::c_void,
        ((pool_size as i64 + 65000 + 1i32 as i64) as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64) as _,
    ) as *mut u8;
    pool_size = (pool_size as i64 + 65000) as i32;
}
unsafe fn out_token(handle: &mut OutputHandleWrapper) {
    for i in buf_ptr1..buf_ptr2 {
        ttstub_output_putc(handle, *buffer.offset(i as isize) as i32);
    }
}
unsafe fn print_a_token() {
    out_token(standard_output.as_mut().unwrap());
    out_token(log_file.as_mut().unwrap());
}
unsafe fn print_bad_input_line() {
    let mut bf_ptr: buf_pointer = 0;
    log!(" : ");
    bf_ptr = 0i32;
    while bf_ptr < buf_ptr2 {
        if lex_class[*buffer.offset(bf_ptr as isize) as usize] == LexType::WhiteSpace {
            /*white_space */
            putc_log(' ' as i32);
        } else {
            putc_log(*buffer.offset(bf_ptr as isize) as i32);
        }
        bf_ptr += 1
    }
    putc_log('\n' as i32);
    log!(" : ");
    bf_ptr = 0i32;
    loop {
        let fresh1 = bf_ptr;
        bf_ptr += 1;
        if fresh1 >= buf_ptr2 {
            break;
        }
        putc_log(' ' as i32);
    }
    bf_ptr = buf_ptr2;
    while bf_ptr < last {
        if lex_class[*buffer.offset(bf_ptr as isize) as usize] == LexType::WhiteSpace {
            /*white_space */
            putc_log(' ' as i32);
        } else {
            putc_log(*buffer.offset(bf_ptr as isize) as i32);
        }
        bf_ptr += 1
    }
    putc_log('\n' as i32);
    bf_ptr = 0i32;
    while bf_ptr < buf_ptr2
        && lex_class[*buffer.offset(bf_ptr as isize) as usize] == LexType::WhiteSpace
    {
        /*white_space */
        bf_ptr += 1
    } /*empty */
    if bf_ptr == buf_ptr2 {
        log!("(Error may have been on previous line)\n");
        /*any_value */
    }
    mark_error();
}
unsafe fn print_skipping_whatever_remains() {
    log!("I\'m skipping whatever remains of this ");
}
unsafe fn sam_wrong_file_name_print() {
    let mut output = standard_output.as_mut().unwrap();
    write!(output, "I couldn\'t open file name `").unwrap();
    name_ptr = 0i32;
    while name_ptr <= name_length {
        let fresh2 = name_ptr;
        name_ptr += 1;
        ttstub_output_putc(output, *name_of_file.offset(fresh2 as isize) as i32);
    }
    ttstub_output_putc(output, '\'' as i32);
    ttstub_output_putc(output, '\n' as i32);
}
unsafe fn print_aux_name() {
    print_a_pool_str(aux_list[aux_ptr as usize]);
    putc_log('\n' as i32);
}
unsafe fn log_pr_aux_name() {
    let lg = log_file.as_mut().unwrap();
    out_pool_str(lg, aux_list[aux_ptr as usize]);
    ttstub_output_putc(lg, '\n' as i32);
}
unsafe fn aux_err_print() {
    log!("---line {} of file ", aux_ln_stack[aux_ptr as usize]);
    print_aux_name();
    print_bad_input_line();
    print_skipping_whatever_remains();
    log!("command\n");
}
unsafe fn aux_err_illegal_another_print(cmd_num: i32) {
    log!("Illegal, another \\bib");
    match cmd_num {
        0 => log!("data"),
        1 => log!("style"),
        _ => {
            log!("Illegal auxiliary-file command");
            print_confusion();
            panic!();
        }
    }
    log!(" command");
}
unsafe fn aux_err_no_right_brace_print() {
    log!("No \"}}\"");
}
unsafe fn aux_err_stuff_after_right_brace_print() {
    log!("Stuff after \"}}\"");
}
unsafe fn aux_err_white_space_in_argument_print() {
    log!("White space in argument");
}
unsafe fn print_bib_name() {
    print_a_pool_str(*bib_list.add(bib_ptr));
    print_a_pool_str(s_bib_extension);
    putc_log('\n' as i32);
}
unsafe fn log_pr_bib_name() {
    let lg = log_file.as_mut().unwrap();
    out_pool_str(lg, *bib_list.add(bib_ptr));
    out_pool_str(lg, s_bib_extension);
    ttstub_output_putc(lg, '\n' as i32);
}
unsafe fn print_bst_name() {
    print_a_pool_str(bst_str);
    print_a_pool_str(s_bst_extension);
    putc_log('\n' as i32);
}
unsafe fn log_pr_bst_name() {
    let lg = log_file.as_mut().unwrap();
    out_pool_str(lg, bst_str);
    out_pool_str(lg, s_bst_extension);
    ttstub_output_putc(lg, '\n' as i32);
}
unsafe fn hash_cite_confusion() {
    log!("Cite hash error");
    print_confusion();
    panic!();
}
unsafe fn check_cite_overflow(mut last_cite: cite_number) {
    if last_cite == max_cites {
        cite_list = xrealloc(
            cite_list as *mut libc::c_void,
            ((max_cites + 750i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<str_number>() as u64) as _,
        ) as *mut str_number;
        type_list = xrealloc(
            type_list as *mut libc::c_void,
            ((max_cites + 750i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64) as _,
        ) as *mut hash_ptr2;
        entry_exists = xrealloc(
            entry_exists as *mut libc::c_void,
            ((max_cites + 750i32 + 1i32) as u64).wrapping_mul(::std::mem::size_of::<bool>() as u64)
                as _,
        ) as *mut bool;
        cite_info = xrealloc(
            cite_info as *mut libc::c_void,
            ((max_cites + 750i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<str_number>() as u64) as _,
        ) as *mut str_number;
        max_cites += 750i32;
        while last_cite < max_cites {
            *type_list.offset(last_cite as isize) = 0i32;
            *cite_info.offset(last_cite as isize) = 0i32;
            last_cite += 1i32
        }
    };
}
unsafe fn aux_end1_err_print() {
    log!("I found no ");
}
unsafe fn aux_end2_err_print() {
    log!("---while reading file ");
    print_aux_name();
    mark_error();
}
unsafe fn bst_ln_num_print() {
    log!("--line {} of file ", bst_line_num);
    print_bst_name();
}
unsafe fn bst_err_print_and_look_for_blank_line() {
    putc_log('-' as i32);
    bst_ln_num_print();
    print_bad_input_line();
    while last != 0 {
        if !input_ln(&mut bst_file) {
            panic!();
        } else {
            bst_line_num += 1;
        }
    }
    buf_ptr2 = last;
}
unsafe fn bst_warn_print() {
    bst_ln_num_print();
    mark_warning();
}
unsafe fn eat_bst_print() {
    log!("Illegal end of style file in command: ");
}
unsafe fn unknwn_function_class_confusion() {
    log!("Unknown function class");
    print_confusion();
    panic!();
}
unsafe fn print_fn_class(fn_loc_0: hash_loc) {
    use FnClass::*;
    match *fn_type.offset(fn_loc_0 as isize) {
        BuiltIn => log!("built-in"),
        WizDefined => log!("wizard-defined"),
        IntLiteral => log!("integer-literal"),
        StrLiteral => log!("string-literal"),
        Field => log!("field"),
        IntEntryVar => log!("integer-entry-variable"),
        StrEntryVar => log!("string-entry-variable"),
        IntGlobalVar => log!("integer-global-variable"),
        StrGlobalVar => log!("string-global-variable"),
    };
}
/*:159*/
/*160: */
unsafe fn id_scanning_confusion() {
    log!("Identifier scanning error");
    print_confusion();
    panic!();
}
unsafe fn bst_id_print(scan_result: ScanResult) {
    if scan_result == ScanResult::IdNull {
        log!(
            "\"{}\" begins identifier, command: ",
            *buffer.offset(buf_ptr2 as isize) as char
        );
    } else if scan_result == ScanResult::OtherCharAdjacent {
        log!(
            "\"{}\" immediately follows identifier, command: ",
            *buffer.offset(buf_ptr2 as isize) as char
        );
    } else {
        id_scanning_confusion();
    };
}
unsafe fn bst_left_brace_print() {
    log!("\"{{\" is missing in command: ");
}
unsafe fn bst_right_brace_print() {
    log!("\"}}\" is missing in command: ");
}
unsafe fn already_seen_function_print(mut seen_fn_loc: hash_loc) {
    print_a_pool_str(*hash_text.offset(seen_fn_loc as isize));
    log!(" is already a type \"");
    print_fn_class(seen_fn_loc);
    log!("\" function name\n");
    bst_err_print_and_look_for_blank_line();
}
unsafe fn bib_ln_num_print() {
    log!("--line {} of file ", bib_line_num);
    print_bib_name();
}
unsafe fn bib_err_print() {
    putc_log('-' as i32);
    bib_ln_num_print();
    print_bad_input_line();
    print_skipping_whatever_remains();
    if at_bib_command {
        log!("command\n");
    } else {
        log!("entry\n");
    };
}
unsafe fn bib_warn_print() {
    bib_ln_num_print();
    mark_warning();
}
unsafe fn check_field_overflow(mut total_fields: i32) {
    let mut f_ptr: field_loc = 0;
    let mut start_fields: field_loc = 0;
    if total_fields > max_fields {
        start_fields = max_fields;
        field_info = xrealloc(
            field_info as *mut libc::c_void,
            ((total_fields + 17250i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<str_number>() as u64) as _,
        ) as *mut str_number;
        max_fields = total_fields + 17250i32;
        let mut for_end: i32 = 0;
        f_ptr = start_fields;
        for_end = max_fields - 1i32;
        if f_ptr <= for_end {
            loop {
                *field_info.offset(f_ptr as isize) = 0i32;
                let fresh3 = f_ptr;
                f_ptr += 1;
                if fresh3 >= for_end {
                    break;
                }
                /*missing */
            }
        }
    };
}
unsafe fn eat_bib_print() {
    log!("Illegal end of database file");
    bib_err_print();
}
unsafe fn bib_one_of_two_print(mut char1: u8, mut char2: u8) {
    log!(
        "I was expecting a `{}' or a `{}'",
        char1 as char,
        char2 as char
    );
    bib_err_print();
}
unsafe fn bib_equals_sign_print() {
    log!("I was expecting an \"=\"");
    bib_err_print();
}
unsafe fn bib_unbalanced_braces_print() {
    log!("Unbalanced braces");
    bib_err_print();
}
unsafe fn bib_field_too_long_print() {
    log!("Your field is more than {} characters", buf_size);
    bib_err_print();
}
unsafe fn macro_warn_print() {
    log!("Warning--string name \"");
    print_a_token();
    log!("\" is ");
}
unsafe fn bib_id_print(scan_result: ScanResult) {
    if scan_result == ScanResult::IdNull {
        log!("You\'re missing ");
    } else if scan_result == ScanResult::OtherCharAdjacent {
        log!(
            "\"{}\" immediately follows ",
            *buffer.offset(buf_ptr2 as isize) as char
        );
    } else {
        id_scanning_confusion();
    };
}
unsafe fn bib_cmd_confusion() {
    log!("Unknown database-file command");
    print_confusion();
    panic!();
}
unsafe fn cite_key_disappeared_confusion() {
    log!("A cite key disappeared");
    print_confusion();
    panic!();
}
unsafe fn bad_cross_reference_print(mut s: str_number) {
    log!("--entry \"");
    print_a_pool_str(*cite_list.offset(cite_ptr as isize));
    putc_log('\"' as i32);
    putc_log('\n' as i32);
    log!("refers to entry \"");
    print_a_pool_str(s);
}
unsafe fn nonexistent_cross_reference_error() {
    log!("A bad cross reference-");
    bad_cross_reference_print(*field_info.offset(field_ptr as isize));
    log!("\", which doesn\'t exist\n");
    mark_error();
}
unsafe fn print_missing_entry(mut s: str_number) {
    log!("Warning--I didn\'t find a database entry for \"");
    print_a_pool_str(s);
    putc_log('\"' as i32);
    putc_log('\n' as i32);
    mark_warning();
}
unsafe fn bst_ex_warn_print() {
    if mess_with_entries {
        log!(" for entry ");
        print_a_pool_str(*cite_list.offset(cite_ptr as isize));
    }
    putc_log('\n' as i32);
    log!("while executing-");
    bst_ln_num_print();
    mark_error();
}
unsafe fn bst_mild_ex_warn_print() {
    if mess_with_entries {
        log!(" for entry ");
        print_a_pool_str(*cite_list.offset(cite_ptr as isize));
    }
    putc_log('\n' as i32);
    log!("while executing");
    bst_warn_print();
}
unsafe fn bst_cant_mess_with_entries_print() {
    log!("You can\'t mess with entries here");
    bst_ex_warn_print();
}
unsafe fn illegl_literal_confusion() {
    log!("Illegal literal type");
    print_confusion();
    panic!();
}
unsafe fn unknwn_literal_confusion() {
    log!("Unknown literal type");
    print_confusion();
    panic!();
}
unsafe fn print_stk_lit(mut stk_lt: i32, mut stk_tp: StkType) {
    match stk_tp as i32 {
        0 => log!("{} is an integer literal", stk_lt),
        1 => {
            putc_log('\"' as i32);
            print_a_pool_str(stk_lt);
            log!("\" is a string literal");
        }
        2 => {
            putc_log('`' as i32);
            print_a_pool_str(*hash_text.offset(stk_lt as isize));
            log!("\' is a function literal");
        }
        3 => {
            putc_log('`' as i32);
            print_a_pool_str(stk_lt);
            log!("\' is a missing field");
        }
        4 => illegl_literal_confusion(),
        _ => unknwn_literal_confusion(),
    };
}
unsafe fn print_lit(mut stk_lt: i32, mut stk_tp: StkType) {
    match stk_tp as i32 {
        0 => log!("{}\n", stk_lt),
        1 => {
            print_a_pool_str(stk_lt);
            putc_log('\n' as i32);
        }
        2 => {
            print_a_pool_str(*hash_text.offset(stk_lt as isize));
            putc_log('\n' as i32);
        }
        3 => {
            print_a_pool_str(stk_lt);
            putc_log('\n' as i32);
        }
        4 => illegl_literal_confusion(),
        _ => unknwn_literal_confusion(),
    };
}
unsafe fn output_bbl_line() {
    let bbl = bbl_file.as_mut().unwrap();
    if out_buf_length != 0 {
        while out_buf_length > 0i32 {
            if !(lex_class[*out_buf.offset((out_buf_length - 1i32) as isize) as usize]
                == LexType::WhiteSpace)
            {
                break;
            }
            /*white_space */
            out_buf_length -= 1i32
        }
        if out_buf_length == 0i32 {
            return;
        }
        out_buf_ptr = 0i32;
        while out_buf_ptr < out_buf_length {
            ttstub_output_putc(bbl, *out_buf.offset(out_buf_ptr as isize) as i32);
            out_buf_ptr += 1
        }
    }
    ttstub_output_putc(bbl, '\n' as i32);
    bbl_line_num += 1;
    out_buf_length = 0i32;
}
unsafe fn bst_1print_string_size_exceeded() {
    log!("Warning--you\'ve exceeded ");
}
unsafe fn bst_2print_string_size_exceeded() {
    log!("-string-size,");
    bst_mild_ex_warn_print();
    log!("*Please notify the bibstyle designer*\n");
}
unsafe fn braces_unbalanced_complaint(mut pop_lit_var: str_number) {
    log!("Warning--\"");
    print_a_pool_str(pop_lit_var);
    log!("\" isn\'t a brace-balanced string");
    bst_mild_ex_warn_print();
}

unsafe fn start_name(mut file_name: str_number) {
    let mut p_ptr: pool_pointer = 0;
    free(name_of_file as *mut libc::c_void);
    name_of_file = xmalloc_array(
        (*str_start.offset(file_name as isize + 1) - *str_start.offset(file_name as isize) + 1)
            as usize,
    );
    name_ptr = 0i32;
    p_ptr = *str_start.offset(file_name as isize);
    while p_ptr < *str_start.offset((file_name + 1i32) as isize) {
        *name_of_file.offset(name_ptr as isize) = *str_pool.offset(p_ptr as isize);
        name_ptr += 1;
        p_ptr += 1
    }
    name_length =
        *str_start.offset((file_name + 1i32) as isize) - *str_start.offset(file_name as isize);
    *name_of_file.offset(name_length as isize) = 0i32 as u8;
}
unsafe fn add_extension(mut ext: str_number) {
    let mut p_ptr: pool_pointer = 0;
    name_ptr = name_length;
    p_ptr = *str_start.offset(ext as isize);
    while p_ptr < *str_start.offset((ext + 1i32) as isize) {
        *name_of_file.offset(name_ptr as isize) = *str_pool.offset(p_ptr as isize);
        name_ptr += 1;
        p_ptr += 1
    }
    name_length += *str_start.offset((ext + 1i32) as isize) - *str_start.offset(ext as isize);
    *name_of_file.offset(name_length as isize) = 0i32 as u8;
}
unsafe fn make_string() -> str_number {
    if str_ptr == max_strings {
        print_overflow();
        log!("number of strings {}\n", max_strings);
        panic!();
    }
    str_ptr += 1i32;
    *str_start.offset(str_ptr as isize) = pool_ptr;
    str_ptr - 1i32
}

unsafe fn get_string_from_pool(s: str_number) -> &'static [u8] {
    let start = *str_start.offset(s as isize);
    let end = *str_start.offset((s + 1) as isize);
    let len = end - start;
    slice::from_raw_parts(str_pool.offset(start as isize), len as usize)
}

unsafe fn str_eq_buf(s: str_number, buf: &[u8]) -> bool {
    let s = get_string_from_pool(s);
    s == buf
}

unsafe fn str_eq_str(s1: str_number, s2: str_number) -> bool {
    get_string_from_pool(s1) == get_string_from_pool(s2)
}

unsafe fn lower_case(mut buf: buf_type, mut bf_ptr: buf_pointer, mut len: buf_pointer) {
    let mut i: buf_pointer = 0;
    if len > 0i32 {
        let mut for_end: i32 = 0;
        i = bf_ptr;
        for_end = bf_ptr + len - 1i32;
        if i <= for_end {
            loop {
                if *buf.offset(i as isize) as i32 >= 'A' as i32
                    && *buf.offset(i as isize) as i32 <= 'Z' as i32
                {
                    *buf.offset(i as isize) = (*buf.offset(i as isize) as i32 + 32i32) as u8
                }
                let fresh4 = i;
                i += 1;
                if fresh4 >= for_end {
                    break;
                }
            }
        }
    };
}
unsafe fn upper_case(mut buf: buf_type, mut bf_ptr: buf_pointer, mut len: buf_pointer) {
    let mut i: buf_pointer = 0;
    if len > 0i32 {
        let mut for_end: i32 = 0;
        i = bf_ptr;
        for_end = bf_ptr + len - 1i32;
        if i <= for_end {
            loop {
                if *buf.offset(i as isize) as i32 >= 'a' as i32
                    && *buf.offset(i as isize) as i32 <= 'z' as i32
                {
                    *buf.offset(i as isize) = (*buf.offset(i as isize) as i32 - 32i32) as u8
                }
                let fresh5 = i;
                i += 1;
                if fresh5 >= for_end {
                    break;
                }
            }
        }
    };
}
unsafe fn str_lookup(
    mut buf: buf_type,
    mut j: buf_pointer,
    mut l: buf_pointer,
    mut ilk: str_ilk,
    mut insert_it: bool,
) -> hash_loc {
    let mut h: i32 = 0;
    let mut p: hash_loc = 0;
    let mut k: buf_pointer = 0;
    let mut str_num: str_number = 0;
    h = 0i32;
    k = j;
    while k < j + l {
        h = h + h + *buf.offset(k as isize) as i32;
        while h >= hash_prime {
            h -= hash_prime
        }
        k += 1i32
    }
    p = h + 1i32;
    hash_found = false;
    str_num = 0i32;
    loop {
        if *hash_text.offset(p as isize) > 0i32 {
            let s = std::slice::from_raw_parts(buf.offset(j as isize), l as usize);
            if str_eq_buf(*hash_text.offset(p as isize), s) {
                if *hash_ilk.offset(p as isize) as i32 == ilk as i32 {
                    hash_found = true;
                    return p;
                /* str_found */
                } else {
                    str_num = *hash_text.offset(p as isize)
                }
            }
        }
        if *hash_next.offset(p as isize) == 0i32 {
            /*empty */
            if !insert_it {
                return p;
            } /* str_not_found */
            if *hash_text.offset(p as isize) > 0i32 {
                loop {
                    if hash_used == 1i32 {
                        print_overflow();
                        log!("hash size {}\n", hash_size);
                        panic!();
                    }
                    hash_used -= 1i32;
                    if *hash_text.offset(hash_used as isize) == 0i32 {
                        break;
                    }
                }
                *hash_next.offset(p as isize) = hash_used;
                p = hash_used
            }
            if str_num > 0i32 {
                *hash_text.offset(p as isize) = str_num
            } else {
                while pool_ptr + l > pool_size {
                    pool_overflow();
                }
                k = j;
                while k < j + l {
                    *str_pool.offset(pool_ptr as isize) = *buf.offset(k as isize);
                    pool_ptr += 1i32;
                    k += 1i32
                }
                *hash_text.offset(p as isize) = make_string()
            }
            *hash_ilk.offset(p as isize) = ilk;
            return p;
        }
        p = *hash_next.offset(p as isize)
    }
}
unsafe fn pre_define(pds: pds_type, len: pds_len, ilk: str_ilk) {
    let mut i: pds_len = 1;
    let for_end = len as i32;
    if i as i32 <= for_end {
        loop {
            *buffer.offset(i as isize) = *pds.offset((i as i32 - 1i32) as isize) as u8;
            let fresh6 = i;
            i = i.wrapping_add(1);
            if (fresh6 as i32) >= for_end {
                break;
            }
        }
    }
    pre_def_loc = str_lookup(buffer, 1i32, len as buf_pointer, ilk, true);
}
unsafe fn int_to_ASCII(
    mut the_int: i32,
    mut int_buf: buf_type,
    mut int_begin: buf_pointer,
    mut int_end: *mut buf_pointer,
) {
    let mut int_ptr: buf_pointer = 0;
    let mut int_xptr: buf_pointer = 0;
    let mut int_tmp_val: u8 = 0;
    int_ptr = int_begin;
    if the_int < 0i32 {
        if int_ptr == buf_size {
            buffer_overflow();
        }
        /* str_found */
        *int_buf.offset(int_ptr as isize) = 45i32 as u8; /*minus_sign */
        int_ptr += 1i32;
        the_int = -the_int
    }
    int_xptr = int_ptr;
    loop {
        if int_ptr == buf_size {
            buffer_overflow();
        }
        *int_buf.offset(int_ptr as isize) = ('0' as i32 + the_int % 10i32) as u8;
        int_ptr += 1i32;
        the_int /= 10i32;
        if the_int == 0i32 {
            break;
        }
    }
    *int_end = int_ptr;
    int_ptr -= 1i32;
    while int_xptr < int_ptr {
        int_tmp_val = *int_buf.offset(int_xptr as isize);
        *int_buf.offset(int_xptr as isize) = *int_buf.offset(int_ptr as isize);
        *int_buf.offset(int_ptr as isize) = int_tmp_val;
        int_ptr -= 1i32;
        int_xptr += 1i32
    }
}
unsafe fn add_database_cite(mut new_cite: *mut cite_number) {
    check_cite_overflow(*new_cite);
    check_field_overflow(num_fields * (*new_cite + 1i32));
    *cite_list.offset(*new_cite as isize) = *hash_text.offset(cite_loc as isize);
    *ilk_info.offset(cite_loc as isize) = *new_cite;
    *ilk_info.offset(lc_cite_loc as isize) = cite_loc;
    *new_cite += 1i32;
}
unsafe fn find_cite_locs_for_this_cite_key(mut cite_str: str_number) -> bool {
    ex_buf_ptr = 0i32;
    let mut tmp_ptr = *str_start.offset(cite_str as isize);
    let mut tmp_end_ptr = *str_start.offset((cite_str + 1i32) as isize);
    while tmp_ptr < tmp_end_ptr {
        *ex_buf.offset(ex_buf_ptr as isize) = *str_pool.offset(tmp_ptr as isize);
        ex_buf_ptr += 1i32;
        tmp_ptr += 1i32
    }
    cite_loc = str_lookup(
        ex_buf,
        0i32,
        *str_start.offset((cite_str + 1i32) as isize) - *str_start.offset(cite_str as isize),
        9i32 as str_ilk,
        false,
    );
    cite_hash_found = hash_found;
    lower_case(
        ex_buf,
        0i32,
        *str_start.offset((cite_str + 1i32) as isize) - *str_start.offset(cite_str as isize),
    );
    lc_cite_loc = str_lookup(
        ex_buf,
        0i32,
        *str_start.offset((cite_str + 1i32) as isize) - *str_start.offset(cite_str as isize),
        10i32 as str_ilk,
        false,
    );
    hash_found
}
unsafe fn swap(mut swap1: cite_number, mut swap2: cite_number) {
    let mut innocent_bystander: cite_number = 0;
    innocent_bystander = *cite_info.offset(swap2 as isize);
    *cite_info.offset(swap2 as isize) = *cite_info.offset(swap1 as isize);
    *cite_info.offset(swap1 as isize) = innocent_bystander;
}
unsafe fn less_than(mut arg1: cite_number, mut arg2: cite_number) -> bool {
    let mut char_ptr: i32 = 0;
    let mut ptr1: str_ent_loc = 0;
    let mut ptr2: str_ent_loc = 0;
    let mut char1: u8 = 0;
    let mut char2: u8 = 0;
    ptr1 = arg1 * num_ent_strs + sort_key_num;
    ptr2 = arg2 * num_ent_strs + sort_key_num;
    char_ptr = 0i32;
    loop {
        char1 = *entry_strs.offset((ptr1 * (ent_str_size + 1i32) + char_ptr) as isize);
        char2 = *entry_strs.offset((ptr2 * (ent_str_size + 1i32) + char_ptr) as isize);
        if char1 as i32 == 127i32 {
            /*end_of_string */
            if char2 as i32 == 127i32 {
                /*end_of_string */
                if arg1 < arg2 {
                    return true;
                } else if arg1 > arg2 {
                    return false;
                } else {
                    log!("Duplicate sort key");
                    print_confusion();
                    panic!();
                }
            } else {
                return true;
            }
        } else if char2 as i32 == 127i32 {
            /*end_of_string */
            return false;
        } else if (char1 as i32) < char2 as i32 {
            return true;
        } else if char1 as i32 > char2 as i32 {
            return false;
        }
        char_ptr += 1i32
    }
}
unsafe fn quick_sort(mut left_end: cite_number, mut right_end: cite_number) {
    let mut left: cite_number = 0;
    let mut right: cite_number = 0;
    let mut insert_ptr: cite_number = 0;
    let mut middle: cite_number = 0;
    let mut partition: cite_number = 0;
    if right_end - left_end < 10i32 {
        /*short_list */
        /*305: */
        let mut for_end: i32 = 0; /*built_in */
        insert_ptr = left_end + 1i32; /*n_aux_citation */
        for_end = right_end; /*n_aux_bibdata */
        if insert_ptr <= for_end {
            loop {
                let mut for_end_0: i32 = 0; /*n_aux_bibstyle */
                right = insert_ptr; /*n_aux_input */
                for_end_0 = left_end + 1i32; /*n_bst_entry */
                if right >= for_end_0 {
                    while !less_than(
                        *cite_info.offset((right - 1i32) as isize),
                        *cite_info.offset(right as isize),
                    ) {
                        swap(right - 1i32, right); /*n_bst_execute */
                        let fresh7 = right; /*n_bst_function */
                        right -= 1; /*n_bst_integers */
                        if fresh7 <= for_end_0 {
                            break; /*n_bst_iterate */
                        }
                    }
                } /*n_bst_macro */
                let fresh8 = insert_ptr; /*n_bst_read */
                insert_ptr += 1; /*n_bst_reverse */
                if fresh8 >= for_end {
                    break; /*n_bst_sort */
                }
            }
        }
    } else {
        left = left_end + 4i32; /*n_bst_strings */
        middle = (left_end + right_end) / 2i32; /*n_bib_comment */
        right = right_end - 4i32; /*n_bib_preamble */
        if less_than(
            *cite_info.offset(left as isize),
            *cite_info.offset(middle as isize),
        ) {
            if less_than(
                *cite_info.offset(middle as isize),
                *cite_info.offset(right as isize),
            ) {
                swap(left_end, middle); /*n_bib_string */
            } else if less_than(
                *cite_info.offset(left as isize),
                *cite_info.offset(right as isize),
            ) {
                swap(left_end, right); /*str_literal */
            } else {
                swap(left_end, left); /*str_literal */
            }
        } else if less_than(
            *cite_info.offset(right as isize),
            *cite_info.offset(middle as isize),
        ) {
            swap(left_end, middle); /*n_i */
        } else if less_than(
            *cite_info.offset(right as isize),
            *cite_info.offset(left as isize),
        ) {
            swap(left_end, right); /*n_j */
        } else {
            swap(left_end, left); /*n_oe */
        } /*n_oe_upper */
        partition = *cite_info.offset(left_end as isize); /*n_ae */
        left = left_end + 1i32; /*n_ae_upper */
        right = right_end; /*n_aa */
        loop {
            while less_than(*cite_info.offset(left as isize), partition) {
                left += 1i32
            } /*n_aa_upper */
            while less_than(partition, *cite_info.offset(right as isize)) {
                right -= 1i32
            } /*n_o */
            if left < right {
                swap(left, right); /*n_o_upper */
                left += 1i32; /*n_l */
                right -= 1i32
            } /*n_l_upper */
            if left == right + 1i32 {
                break; /*n_ss */
            }
        } /*field */
        swap(left_end, right); /*str_entry_var */
        quick_sort(left_end, right - 1i32); /*int_global_var */
        quick_sort(left, right_end); /*int_global_var */
    };
}
unsafe fn build_in(
    pds: pds_type,
    len: pds_len,
    fn_hash_loc: &mut hash_loc,
    blt_in_num: blt_in_range,
) {
    pre_define(pds, len, 11i32 as str_ilk);
    *fn_hash_loc = pre_def_loc;
    *fn_type.offset(*fn_hash_loc as isize) = FnClass::BuiltIn;
    *ilk_info.offset(*fn_hash_loc as isize) = blt_in_num;
}
unsafe fn pre_def_certain_strings() {
    pre_define(b".aux        \x00" as *const u8 as *const i8, 4, 7);
    s_aux_extension = *hash_text.offset(pre_def_loc as isize);
    pre_define(b".bbl        \x00" as *const u8 as *const i8, 4, 7);
    s_bbl_extension = *hash_text.offset(pre_def_loc as isize);
    pre_define(b".blg        \x00" as *const u8 as *const i8, 4, 7);
    s_log_extension = *hash_text.offset(pre_def_loc as isize);
    pre_define(b".bst        \x00" as *const u8 as *const i8, 4, 7);
    s_bst_extension = *hash_text.offset(pre_def_loc as isize);
    pre_define(b".bib        \x00" as *const u8 as *const i8, 4, 7);
    s_bib_extension = *hash_text.offset(pre_def_loc as isize);
    pre_define(b"texinputs:  \x00" as *const u8 as *const i8, 10, 8);
    s_bst_area = *hash_text.offset(pre_def_loc as isize);
    pre_define(b"texbib:     \x00" as *const u8 as *const i8, 7, 8);
    s_bib_area = *hash_text.offset(pre_def_loc as isize);
    pre_define(b"\\citation   \x00" as *const u8 as *const i8, 9, 2);
    *ilk_info.offset(pre_def_loc as isize) = 2i32;
    pre_define(b"\\bibdata    \x00" as *const u8 as *const i8, 8, 2);
    *ilk_info.offset(pre_def_loc as isize) = 0i32;
    pre_define(b"\\bibstyle   \x00" as *const u8 as *const i8, 9, 2);
    *ilk_info.offset(pre_def_loc as isize) = 1i32;
    pre_define(b"\\@input     \x00" as *const u8 as *const i8, 7, 2);
    *ilk_info.offset(pre_def_loc as isize) = 3i32;
    pre_define(b"entry       \x00" as *const u8 as *const i8, 5, 4);
    *ilk_info.offset(pre_def_loc as isize) = 0i32;
    pre_define(b"execute     \x00" as *const u8 as *const i8, 7, 4);
    *ilk_info.offset(pre_def_loc as isize) = 1i32;
    pre_define(b"function    \x00" as *const u8 as *const i8, 8, 4);
    *ilk_info.offset(pre_def_loc as isize) = 2i32;
    pre_define(b"integers    \x00" as *const u8 as *const i8, 8, 4);
    *ilk_info.offset(pre_def_loc as isize) = 3i32;
    pre_define(b"iterate     \x00" as *const u8 as *const i8, 7, 4);
    *ilk_info.offset(pre_def_loc as isize) = 4i32;
    pre_define(b"macro       \x00" as *const u8 as *const i8, 5, 4);
    *ilk_info.offset(pre_def_loc as isize) = 5i32;
    pre_define(b"read        \x00" as *const u8 as *const i8, 4, 4);
    *ilk_info.offset(pre_def_loc as isize) = 6i32;
    pre_define(b"reverse     \x00" as *const u8 as *const i8, 7, 4);
    *ilk_info.offset(pre_def_loc as isize) = 7i32;
    pre_define(b"sort        \x00" as *const u8 as *const i8, 4, 4);
    *ilk_info.offset(pre_def_loc as isize) = 8i32;
    pre_define(b"strings     \x00" as *const u8 as *const i8, 7, 4);
    *ilk_info.offset(pre_def_loc as isize) = 9i32;
    pre_define(b"comment     \x00" as *const u8 as *const i8, 7, 12);
    *ilk_info.offset(pre_def_loc as isize) = 0i32;
    pre_define(b"preamble    \x00" as *const u8 as *const i8, 8, 12);
    *ilk_info.offset(pre_def_loc as isize) = 1i32;
    pre_define(b"string      \x00" as *const u8 as *const i8, 6, 12);
    *ilk_info.offset(pre_def_loc as isize) = 2i32;
    build_in(
        b"=           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        &mut b_equals,
        0i32,
    );
    build_in(
        b">           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        &mut b_greater_than,
        1i32,
    );
    build_in(
        b"<           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        &mut b_less_than,
        2i32,
    );
    build_in(
        b"+           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        &mut b_plus,
        3i32,
    );
    build_in(
        b"-           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        &mut b_minus,
        4i32,
    );
    build_in(
        b"*           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        &mut b_concatenate,
        5i32,
    );
    build_in(
        b":=          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        &mut b_gets,
        6i32,
    );
    build_in(
        b"add.period$ \x00" as *const u8 as *const i8,
        11i32 as pds_len,
        &mut b_add_period,
        7i32,
    );
    build_in(
        b"call.type$  \x00" as *const u8 as *const i8,
        10i32 as pds_len,
        &mut b_call_type,
        8i32,
    );
    build_in(
        b"change.case$\x00" as *const u8 as *const i8,
        12i32 as pds_len,
        &mut b_change_case,
        9i32,
    );
    build_in(
        b"chr.to.int$ \x00" as *const u8 as *const i8,
        11i32 as pds_len,
        &mut b_chr_to_int,
        10i32,
    );
    build_in(
        b"cite$       \x00" as *const u8 as *const i8,
        5i32 as pds_len,
        &mut b_cite,
        11i32,
    );
    build_in(
        b"duplicate$  \x00" as *const u8 as *const i8,
        10i32 as pds_len,
        &mut b_duplicate,
        12i32,
    );
    build_in(
        b"empty$      \x00" as *const u8 as *const i8,
        6i32 as pds_len,
        &mut b_empty,
        13i32,
    );
    build_in(
        b"format.name$\x00" as *const u8 as *const i8,
        12i32 as pds_len,
        &mut b_format_name,
        14i32,
    );
    build_in(
        b"if$         \x00" as *const u8 as *const i8,
        3i32 as pds_len,
        &mut b_if,
        15i32,
    );
    build_in(
        b"int.to.chr$ \x00" as *const u8 as *const i8,
        11i32 as pds_len,
        &mut b_int_to_chr,
        16i32,
    );
    build_in(
        b"int.to.str$ \x00" as *const u8 as *const i8,
        11i32 as pds_len,
        &mut b_int_to_str,
        17i32,
    );
    build_in(
        b"missing$    \x00" as *const u8 as *const i8,
        8i32 as pds_len,
        &mut b_missing,
        18i32,
    );
    build_in(
        b"newline$    \x00" as *const u8 as *const i8,
        8i32 as pds_len,
        &mut b_newline,
        19i32,
    );
    build_in(
        b"num.names$  \x00" as *const u8 as *const i8,
        10i32 as pds_len,
        &mut b_num_names,
        20i32,
    );
    build_in(
        b"pop$        \x00" as *const u8 as *const i8,
        4i32 as pds_len,
        &mut b_pop,
        21i32,
    );
    build_in(
        b"preamble$   \x00" as *const u8 as *const i8,
        9i32 as pds_len,
        &mut b_preamble,
        22i32,
    );
    build_in(
        b"purify$     \x00" as *const u8 as *const i8,
        7i32 as pds_len,
        &mut b_purify,
        23i32,
    );
    build_in(
        b"quote$      \x00" as *const u8 as *const i8,
        6i32 as pds_len,
        &mut b_quote,
        24i32,
    );
    build_in(
        b"skip$       \x00" as *const u8 as *const i8,
        5i32 as pds_len,
        &mut b_skip,
        25i32,
    );
    build_in(
        b"stack$      \x00" as *const u8 as *const i8,
        6i32 as pds_len,
        &mut b_stack,
        26i32,
    );
    build_in(
        b"substring$  \x00" as *const u8 as *const i8,
        10i32 as pds_len,
        &mut b_substring,
        27i32,
    );
    build_in(
        b"swap$       \x00" as *const u8 as *const i8,
        5i32 as pds_len,
        &mut b_swap,
        28i32,
    );
    build_in(
        b"text.length$\x00" as *const u8 as *const i8,
        12i32 as pds_len,
        &mut b_text_length,
        29i32,
    );
    build_in(
        b"text.prefix$\x00" as *const u8 as *const i8,
        12i32 as pds_len,
        &mut b_text_prefix,
        30i32,
    );
    build_in(
        b"top$        \x00" as *const u8 as *const i8,
        4i32 as pds_len,
        &mut b_top_stack,
        31i32,
    );
    build_in(
        b"type$       \x00" as *const u8 as *const i8,
        5i32 as pds_len,
        &mut b_type,
        32i32,
    );
    build_in(
        b"warning$    \x00" as *const u8 as *const i8,
        8i32 as pds_len,
        &mut b_warning,
        33i32,
    );
    build_in(
        b"while$      \x00" as *const u8 as *const i8,
        6i32 as pds_len,
        &mut b_while,
        34i32,
    );
    build_in(
        b"width$      \x00" as *const u8 as *const i8,
        6i32 as pds_len,
        &mut b_width,
        35i32,
    );
    build_in(
        b"write$      \x00" as *const u8 as *const i8,
        6i32 as pds_len,
        &mut b_write,
        36i32,
    );
    pre_define(
        b"            \x00" as *const u8 as *const i8,
        0i32 as pds_len,
        0i32 as str_ilk,
    );
    s_null = *hash_text.offset(pre_def_loc as isize);
    *fn_type.offset(pre_def_loc as isize) = FnClass::StrLiteral;
    pre_define(
        b"default.type\x00" as *const u8 as *const i8,
        12i32 as pds_len,
        0i32 as str_ilk,
    );
    s_default = *hash_text.offset(pre_def_loc as isize);
    *fn_type.offset(pre_def_loc as isize) = FnClass::StrLiteral;
    b_default = b_skip;
    preamble_ptr = 0;
    pre_define(
        b"i           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 0i32;
    pre_define(
        b"j           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 1i32;
    pre_define(
        b"oe          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 2i32;
    pre_define(
        b"OE          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 3i32;
    pre_define(
        b"ae          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 4i32;
    pre_define(
        b"AE          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 5i32;
    pre_define(
        b"aa          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 6i32;
    pre_define(
        b"AA          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 7i32;
    pre_define(
        b"o           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 8i32;
    pre_define(
        b"O           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 9i32;
    pre_define(
        b"l           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 10i32;
    pre_define(
        b"L           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 11i32;
    pre_define(
        b"ss          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 12i32;
    pre_define(
        b"crossref    \x00" as *const u8 as *const i8,
        8i32 as pds_len,
        11i32 as str_ilk,
    );
    *fn_type.offset(pre_def_loc as isize) = FnClass::Field;
    *ilk_info.offset(pre_def_loc as isize) = num_fields;
    crossref_num = num_fields;
    num_fields += 1i32;
    num_pre_defined_fields = num_fields;
    pre_define(
        b"sort.key$   \x00" as *const u8 as *const i8,
        9i32 as pds_len,
        11i32 as str_ilk,
    );
    *fn_type.offset(pre_def_loc as isize) = FnClass::StrEntryVar;
    *ilk_info.offset(pre_def_loc as isize) = num_ent_strs;
    sort_key_num = num_ent_strs;
    num_ent_strs += 1i32;
    pre_define(
        b"entry.max$  \x00" as *const u8 as *const i8,
        10i32 as pds_len,
        11i32 as str_ilk,
    );
    *fn_type.offset(pre_def_loc as isize) = FnClass::IntGlobalVar;
    *ilk_info.offset(pre_def_loc as isize) = ent_str_size;
    pre_define(
        b"global.max$ \x00" as *const u8 as *const i8,
        11i32 as pds_len,
        11i32 as str_ilk,
    );
    *fn_type.offset(pre_def_loc as isize) = FnClass::IntGlobalVar;
    *ilk_info.offset(pre_def_loc as isize) = glob_str_size;
}
unsafe fn scan1(mut char1: u8) -> bool {
    buf_ptr1 = buf_ptr2;
    while buf_ptr2 < last && *buffer.offset(buf_ptr2 as isize) != char1 {
        buf_ptr2 += 1;
    }
    buf_ptr2 < last
}
unsafe fn scan1_white(mut char1: u8) -> bool {
    buf_ptr1 = buf_ptr2;
    while buf_ptr2 < last
        && lex_class[*buffer.offset(buf_ptr2 as isize) as usize] != LexType::WhiteSpace
        && *buffer.offset(buf_ptr2 as isize) != char1
    {
        buf_ptr2 += 1i32
    }
    buf_ptr2 < last
}
unsafe fn scan2(mut char1: u8, mut char2: u8) -> bool {
    buf_ptr1 = buf_ptr2;
    while buf_ptr2 < last
        && *buffer.offset(buf_ptr2 as isize) != char1
        && *buffer.offset(buf_ptr2 as isize) != char2
    {
        buf_ptr2 += 1;
    }
    buf_ptr2 < last
}
unsafe fn scan2_white(mut char1: u8, mut char2: u8) -> bool {
    buf_ptr1 = buf_ptr2;
    while buf_ptr2 < last
        && *buffer.offset(buf_ptr2 as isize) != char1
        && *buffer.offset(buf_ptr2 as isize) != char2
        && lex_class[*buffer.offset(buf_ptr2 as isize) as usize] != LexType::WhiteSpace
    {
        buf_ptr2 += 1i32
    }
    buf_ptr2 < last
}
unsafe fn scan3(mut char1: u8, mut char2: u8, mut char3: u8) -> bool {
    buf_ptr1 = buf_ptr2;
    while buf_ptr2 < last
        && *buffer.offset(buf_ptr2 as isize) as i32 != char1 as i32
        && *buffer.offset(buf_ptr2 as isize) as i32 != char2 as i32
        && *buffer.offset(buf_ptr2 as isize) as i32 != char3 as i32
    {
        buf_ptr2 += 1i32
    }
    buf_ptr2 < last
}
unsafe fn scan_alpha() -> bool {
    buf_ptr1 = buf_ptr2;
    while buf_ptr2 < last && lex_class[*buffer.offset(buf_ptr2 as isize) as usize] == LexType::Alpha
    {
        buf_ptr2 += 1i32
    }
    buf_ptr2 - buf_ptr1 != 0
}
unsafe fn scan_identifier(mut char1: u8, mut char2: u8, mut char3: u8) -> ScanResult {
    let scan_result;
    buf_ptr1 = buf_ptr2;
    if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] != LexType::Numeric {
        while buf_ptr2 < last
            && id_class[*buffer.offset(buf_ptr2 as isize) as usize] == IdType::Legal
        {
            buf_ptr2 += 1;
        }
    }
    if buf_ptr2 - buf_ptr1 == 0i32 {
        scan_result = ScanResult::IdNull;
    } else if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] == LexType::WhiteSpace
        || buf_ptr2 == last
    {
        scan_result = ScanResult::WhiteAdjacent;
    } else if *buffer.offset(buf_ptr2 as isize) == char1
        || *buffer.offset(buf_ptr2 as isize) == char2
        || *buffer.offset(buf_ptr2 as isize) == char3
    {
        scan_result = ScanResult::SpecifiedCharAdjacent;
    } else {
        scan_result = ScanResult::OtherCharAdjacent;
    };
    scan_result
}
unsafe fn scan_nonneg_integer() -> Result<&'static [u8], ()> {
    buf_ptr1 = buf_ptr2;
    while buf_ptr2 < last && char::from(*buffer.offset(buf_ptr2 as isize)).is_ascii_digit() {
        buf_ptr2 += 1;
    }
    // If nothing was read, the pointers are the same and false should be returned.
    let len = (buf_ptr2 - buf_ptr1) as usize;
    if len != 0 {
        Ok(slice::from_raw_parts(buffer.offset(buf_ptr1 as isize), len))
    } else {
        Err(())
    }
}
unsafe fn scan_integer() -> Result<i32, ()> {
    let sign_length;
    buf_ptr1 = buf_ptr2;
    if *buffer.offset(buf_ptr2 as isize) == b'-' {
        sign_length = 1;
        buf_ptr2 += 1;
    } else {
        sign_length = 0;
    }
    let mut token_value = 0;
    while buf_ptr2 < last {
        if let Some(d) = char::from(*buffer.offset(buf_ptr2 as isize)).to_digit(10) {
            token_value = token_value * 10 + d as i32;
            buf_ptr2 += 1;
        } else {
            break;
        }
    }
    if sign_length == 1 {
        token_value = -token_value;
    }
    if buf_ptr2 != sign_length + buf_ptr1 {
        Ok(token_value)
    } else {
        Err(())
    }
}
unsafe fn scan_white_space() -> bool {
    while buf_ptr2 < last
        && lex_class[*buffer.offset(buf_ptr2 as isize) as usize] == LexType::WhiteSpace
    {
        buf_ptr2 += 1;
    }
    buf_ptr2 < last
}
unsafe fn eat_bst_white_space() -> bool {
    loop {
        if scan_white_space() && *buffer.offset(buf_ptr2 as isize) != b'%' {
            /*comment */
            return true;
        }
        if !input_ln(&mut bst_file) {
            return false;
        }
        bst_line_num += 1;
        buf_ptr2 = 0i32
    }
}
unsafe fn skip_token_print() {
    putc_log('-' as i32);
    bst_ln_num_print();
    mark_error();
    scan2_white(b'{', b'%');
}
unsafe fn print_recursion_illegal() {
    log!("Curse you, wizard, before you recurse me:\n");
    log!("function ");
    print_a_token();
    log!(" is illegal in its own definition\n");
    skip_token_print();
}
unsafe fn skp_token_unknown_function_print() {
    print_a_token();
    log!(" is an unknown function");
    skip_token_print();
}
unsafe fn skip_illegal_stuff_after_token_print() {
    log!(
        "\"{}\" can't follow a literal",
        *buffer.offset(buf_ptr2 as isize) as char
    );
    skip_token_print();
}
unsafe fn scan_fn_def(mut fn_hash_loc: hash_loc) {
    let mut singl_function: *mut hash_ptr2 = ptr::null_mut();
    let mut single_fn_space: i32 = 0;
    let mut single_ptr: fn_def_loc = 0;
    let mut copy_ptr: fn_def_loc = 0;
    let mut end_of_num: buf_pointer = 0;
    let mut impl_fn_loc: hash_loc = 0;
    single_fn_space = SINGLE_FN_SPACE;
    singl_function = xmalloc(
        ((single_fn_space + 1i32) as u64).wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64)
            as _,
    ) as *mut hash_ptr2;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("function");
        bst_err_print_and_look_for_blank_line();
        return exit(singl_function);
    }
    single_ptr = 0;
    loop {
        if *buffer.offset(buf_ptr2 as isize) as i32 == 125 {
            /*right_brace */
            break;
        }
        match *buffer.offset(buf_ptr2 as isize) as i32 {
            35 => {
                buf_ptr2 += 1;
                let token_value = match scan_integer() {
                    Err(_) => {
                        log!("Illegal integer in integer literal");
                        skip_token_print();
                        lab25(singl_function);
                        continue;
                    }
                    Ok(t) => t,
                };
                literal_loc =
                    str_lookup(buffer, buf_ptr1, buf_ptr2 - buf_ptr1, 1i32 as str_ilk, true); /*integer_ilk */
                if !hash_found {
                    *fn_type.offset(literal_loc as isize) = FnClass::IntLiteral;
                    *ilk_info.offset(literal_loc as isize) = token_value
                }
                if buf_ptr2 < last
                    && lex_class[*buffer.offset(buf_ptr2 as isize) as usize] != LexType::WhiteSpace
                    && *buffer.offset(buf_ptr2 as isize) != b'}'
                    && *buffer.offset(buf_ptr2 as isize) != b'%'
                {
                    skip_illegal_stuff_after_token_print();
                    lab25(singl_function);
                    continue;
                }
                *singl_function.offset(single_ptr as isize) = literal_loc;
                if single_ptr == single_fn_space {
                    singl_function = xrealloc(
                        singl_function as *mut libc::c_void,
                        ((single_fn_space + 100i32 + 1i32) as u64)
                            .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64)
                            as _,
                    ) as *mut hash_ptr2;
                    single_fn_space += 100i32
                }
                single_ptr += 1
            }
            34 => {
                buf_ptr2 += 1;
                if !scan1(34) {
                    log!("No `\"\' to end string literal");
                    skip_token_print();
                    lab25(singl_function);
                    continue;
                }

                literal_loc =
                    str_lookup(buffer, buf_ptr1, buf_ptr2 - buf_ptr1, 0i32 as str_ilk, true);

                *fn_type.offset(literal_loc as isize) = FnClass::StrLiteral;
                buf_ptr2 += 1;
                if buf_ptr2 < last
                    && lex_class[*buffer.offset(buf_ptr2 as isize) as usize] != LexType::WhiteSpace
                    && *buffer.offset(buf_ptr2 as isize) != b'}'
                    && *buffer.offset(buf_ptr2 as isize) != b'%'
                {
                    skip_illegal_stuff_after_token_print();
                    lab25(singl_function);
                    continue;
                }
                *singl_function.offset(single_ptr as isize) = literal_loc;
                if single_ptr == single_fn_space {
                    singl_function = xrealloc(
                        singl_function as *mut libc::c_void,
                        ((single_fn_space + 100i32 + 1i32) as u64)
                            .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64)
                            as _,
                    ) as *mut hash_ptr2;
                    single_fn_space += 100i32
                }
                single_ptr += 1
            }
            39 => {
                buf_ptr2 += 1;
                scan2_white(125 /*right_brace */, 37 /*comment */);
                lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
                fn_loc = str_lookup(
                    buffer,
                    buf_ptr1,
                    buf_ptr2 - buf_ptr1,
                    11, /*bst_fn_ilk */
                    false,
                );
                if !hash_found {
                    skp_token_unknown_function_print();
                    lab25(singl_function);
                    continue;
                } else if fn_loc == wiz_loc {
                    /*194: */
                    print_recursion_illegal();
                    lab25(singl_function);
                    continue;
                } else {
                    *singl_function.offset(single_ptr as isize) = 1i32 - 1i32;
                    if single_ptr == single_fn_space {
                        singl_function = xrealloc(
                            singl_function as *mut libc::c_void,
                            ((single_fn_space + 100i32 + 1i32) as u64)
                                .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64)
                                as _,
                        ) as *mut hash_ptr2;
                        single_fn_space += 100i32
                    }
                    single_ptr += 1;
                    *singl_function.offset(single_ptr as isize) = fn_loc;
                    if single_ptr == single_fn_space {
                        singl_function = xrealloc(
                            singl_function as *mut libc::c_void,
                            ((single_fn_space + 100i32 + 1i32) as u64)
                                .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64)
                                as _,
                        ) as *mut hash_ptr2;
                        single_fn_space += 100i32
                    }
                    single_ptr += 1
                }
            }
            123 => {
                *ex_buf.offset(0) = 39 /*single_quote */;
                int_to_ASCII(impl_fn_num, ex_buf, 1, &mut end_of_num);
                impl_fn_loc = str_lookup(ex_buf, 0, end_of_num, 11 /*bst_fn_ilk */, true);
                if hash_found {
                    log!("Already encountered implicit function");
                    print_confusion();
                    panic!();
                }
                impl_fn_num += 1;
                *fn_type.offset(impl_fn_loc as isize) = FnClass::WizDefined;
                *singl_function.offset(single_ptr as isize) = quote_next_fn;
                if single_ptr == single_fn_space {
                    singl_function = xrealloc(
                        singl_function as *mut libc::c_void,
                        ((single_fn_space + 100i32 + 1i32) as u64)
                            .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64)
                            as _,
                    ) as *mut hash_ptr2;
                    single_fn_space += 100i32
                }
                single_ptr += 1;

                *singl_function.offset(single_ptr as isize) = impl_fn_loc;
                if single_ptr == single_fn_space {
                    singl_function = xrealloc(
                        singl_function as *mut libc::c_void,
                        ((single_fn_space + 100i32 + 1i32) as u64)
                            .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64)
                            as _,
                    ) as *mut hash_ptr2;
                    single_fn_space += 100i32
                }
                single_ptr += 1;

                buf_ptr2 += 1;
                scan_fn_def(impl_fn_loc);
            }
            _ => {
                scan2_white(125 /*right_brace */, 37 /*comment */);
                lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
                fn_loc = str_lookup(
                    buffer,
                    buf_ptr1,
                    buf_ptr2 - buf_ptr1,
                    11, /*bst_fn_ilk */
                    false,
                );
                if !hash_found {
                    skp_token_unknown_function_print();
                    lab25(singl_function);
                    continue;
                } else if fn_loc == wiz_loc {
                    print_recursion_illegal();
                    lab25(singl_function);
                    continue;
                } else {
                    *singl_function.offset(single_ptr as isize) = fn_loc;
                    if single_ptr == single_fn_space {
                        singl_function = xrealloc(
                            singl_function as *mut libc::c_void,
                            ((single_fn_space + 100i32 + 1i32) as u64)
                                .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64)
                                as _,
                        ) as *mut hash_ptr2;
                        single_fn_space += 100i32
                    }
                    single_ptr += 1
                }
            }
        }
        unsafe fn lab25(singl_function: *mut i32) {
            /*next_token */
            if !eat_bst_white_space() {
                eat_bst_print();
                log!("function");
                bst_err_print_and_look_for_blank_line();
                return exit(singl_function);
            }
        }
        lab25(singl_function);
        continue;
    }
    *singl_function.offset(single_ptr as isize) = end_of_def;
    if single_ptr == single_fn_space {
        singl_function = xrealloc(
            singl_function as *mut libc::c_void,
            ((single_fn_space + 100i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64) as _,
        ) as *mut hash_ptr2;
        single_fn_space += 100i32
    }
    single_ptr += 1i32;
    while single_ptr + wiz_def_ptr > wiz_fn_space {
        wiz_functions = xrealloc(
            wiz_functions as *mut libc::c_void,
            ((wiz_fn_space + 3000i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64) as _,
        ) as *mut hash_ptr2;
        wiz_fn_space += 3000i32
    }
    *ilk_info.offset(fn_hash_loc as isize) = wiz_def_ptr;
    copy_ptr = 0i32;
    while copy_ptr < single_ptr {
        *wiz_functions.offset(wiz_def_ptr as isize) = *singl_function.offset(copy_ptr as isize);
        copy_ptr += 1i32;
        wiz_def_ptr += 1i32
    }
    buf_ptr2 += 1i32;

    unsafe fn exit(singl_function: *mut i32) {
        free(singl_function as *mut libc::c_void);
    }
    exit(singl_function);
}
unsafe fn eat_bib_white_space() -> bool {
    while !scan_white_space() {
        if !input_ln(&mut bib_file[bib_ptr]) {
            return false;
        }
        bib_line_num += 1i32;
        buf_ptr2 = 0i32
    }
    true
}
unsafe fn compress_bib_white() -> bool {
    if ex_buf_ptr == buf_size {
        bib_field_too_long_print();
        return false;
    } else {
        *ex_buf.offset(ex_buf_ptr as isize) = 32i32 as u8;
        ex_buf_ptr += 1i32
    }
    while !scan_white_space() {
        if !input_ln(&mut bib_file[bib_ptr]) {
            eat_bib_print();
            return false;
        }
        bib_line_num += 1i32;
        buf_ptr2 = 0i32
    }
    true
}
unsafe fn scan_balanced_braces(right_str_delim: u8) -> bool {
    buf_ptr2 += 1i32;
    if (lex_class[*buffer.offset(buf_ptr2 as isize) as usize] == LexType::WhiteSpace
        || buf_ptr2 == last)
        && !compress_bib_white()
    {
        return false;
    }
    if ex_buf_ptr > 1i32 && *ex_buf.offset((ex_buf_ptr - 1i32) as isize) as i32 == 32i32 {
        /*space */
        if *ex_buf.offset((ex_buf_ptr - 2i32) as isize) as i32 == 32i32 {
            /*space */
            ex_buf_ptr -= 1i32
        }
    } /*255: */
    bib_brace_level = 0i32;
    if store_field {
        /*257: */
        while *buffer.offset(buf_ptr2 as isize) as i32 != right_str_delim as i32 {
            match *buffer.offset(buf_ptr2 as isize) as i32 {
                123 => {
                    bib_brace_level += 1i32; /*left_brace */
                    if ex_buf_ptr == buf_size {
                        bib_field_too_long_print(); /*right_brace */
                        return false;
                    } else {
                        *ex_buf.offset(ex_buf_ptr as isize) = 123i32 as u8; /*left_brace */
                        ex_buf_ptr += 1i32
                    }
                    buf_ptr2 += 1i32;
                    if (lex_class[*buffer.offset(buf_ptr2 as isize) as usize]
                        == LexType::WhiteSpace
                        || buf_ptr2 == last)
                        && !compress_bib_white()
                    {
                        return false;
                    }
                    loop {
                        match *buffer.offset(buf_ptr2 as isize) as i32 {
                            125 => {
                                bib_brace_level -= 1i32;
                                if ex_buf_ptr == buf_size {
                                    bib_field_too_long_print();
                                    return false;
                                } else {
                                    *ex_buf.offset(ex_buf_ptr as isize) = 125i32 as u8;
                                    ex_buf_ptr += 1i32
                                }
                                buf_ptr2 += 1i32;
                                if (lex_class[*buffer.offset(buf_ptr2 as isize) as usize]
                                    == LexType::WhiteSpace
                                    || buf_ptr2 == last)
                                    && !compress_bib_white()
                                {
                                    return false;
                                }
                                if bib_brace_level == 0i32 {
                                    break;
                                }
                            }
                            123 => {
                                bib_brace_level += 1i32;
                                if ex_buf_ptr == buf_size {
                                    bib_field_too_long_print();
                                    return false;
                                } else {
                                    *ex_buf.offset(ex_buf_ptr as isize) = 123i32 as u8;
                                    ex_buf_ptr += 1i32
                                }
                                buf_ptr2 += 1i32;
                                if (lex_class[*buffer.offset(buf_ptr2 as isize) as usize]
                                    == LexType::WhiteSpace
                                    || buf_ptr2 == last)
                                    && !compress_bib_white()
                                {
                                    return false;
                                }
                            }
                            _ => {
                                if ex_buf_ptr == buf_size {
                                    bib_field_too_long_print();
                                    return false;
                                } else {
                                    *ex_buf.offset(ex_buf_ptr as isize) =
                                        *buffer.offset(buf_ptr2 as isize);
                                    ex_buf_ptr += 1i32
                                }
                                buf_ptr2 += 1i32;
                                if (lex_class[*buffer.offset(buf_ptr2 as isize) as usize]
                                    == LexType::WhiteSpace
                                    || buf_ptr2 == last)
                                    && !compress_bib_white()
                                {
                                    return false;
                                }
                            }
                        }
                    }
                }
                125 => {
                    bib_unbalanced_braces_print();
                    return false;
                }
                _ => {
                    if ex_buf_ptr == buf_size {
                        bib_field_too_long_print();
                        return false;
                    } else {
                        *ex_buf.offset(ex_buf_ptr as isize) = *buffer.offset(buf_ptr2 as isize);
                        ex_buf_ptr += 1i32
                    }
                    buf_ptr2 += 1i32;
                    if (lex_class[*buffer.offset(buf_ptr2 as isize) as usize]
                        == LexType::WhiteSpace
                        || buf_ptr2 == last)
                        && !compress_bib_white()
                    {
                        return false;
                    }
                }
            }
        }
    } else {
        while *buffer.offset(buf_ptr2 as isize) as i32 != right_str_delim as i32 {
            if *buffer.offset(buf_ptr2 as isize) as i32 == 123i32 {
                /*left_brace */
                bib_brace_level += 1i32;
                buf_ptr2 += 1i32;
                if !eat_bib_white_space() {
                    eat_bib_print();
                    return false;
                }
                while bib_brace_level > 0i32 {
                    /*256: */
                    if *buffer.offset(buf_ptr2 as isize) as i32 == 125i32 {
                        /*right_brace */
                        bib_brace_level -= 1i32;
                        buf_ptr2 += 1i32;
                        if !eat_bib_white_space() {
                            eat_bib_print();
                            return false;
                        }
                    } else if *buffer.offset(buf_ptr2 as isize) as i32 == 123i32 {
                        /*left_brace */
                        bib_brace_level += 1i32;
                        buf_ptr2 += 1i32;
                        if !eat_bib_white_space() {
                            eat_bib_print();
                            return false;
                        }
                    } else {
                        buf_ptr2 += 1i32;
                        if !scan2(125i32 as u8, 123i32 as u8) && !eat_bib_white_space() {
                            eat_bib_print();
                            return false;
                        }
                    }
                }
            } else if *buffer.offset(buf_ptr2 as isize) as i32 == 125i32 {
                /*right_brace */
                bib_unbalanced_braces_print(); /*right_brace */
                return false;
            } else {
                buf_ptr2 += 1i32; /*double_quote */
                if !scan3(right_str_delim, 123i32 as u8, 125i32 as u8) && !eat_bib_white_space() {
                    eat_bib_print();
                    return false;
                }
            }
        }
    }
    buf_ptr2 += 1i32;
    true
}
unsafe fn scan_a_field_token_and_eat_white() -> bool {
    match *buffer.offset(buf_ptr2 as isize) {
        b'{' => {
            if !scan_balanced_braces(b'}') {
                return false;
            }
        }
        b'"' => {
            if !scan_balanced_braces(b'"') {
                return false;
            }
        }
        c if c.is_ascii_digit() => match scan_nonneg_integer() {
            Err(_) => {
                log!("A digit disappeared");
                print_confusion();
                panic!();
            }
            Ok(integer_str) => {
                if store_field {
                    for c in integer_str.iter() {
                        if ex_buf_ptr == buf_size {
                            bib_field_too_long_print();
                            return false;
                        } else {
                            *ex_buf.offset(ex_buf_ptr as isize) = *c;
                            ex_buf_ptr += 1i32
                        }
                    }
                }
            }
        },
        _ => {
            let scan_result = scan_identifier(44i32 as u8, right_outer_delim, 35i32 as u8);
            if scan_result == ScanResult::WhiteAdjacent
                || scan_result == ScanResult::SpecifiedCharAdjacent
            {
            } else {
                bib_id_print(scan_result);
                log!("a field part");
                bib_err_print();
                return false;
            }
            if store_field {
                lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
                macro_name_loc = str_lookup(
                    buffer,
                    buf_ptr1,
                    buf_ptr2 - buf_ptr1,
                    13i32 as str_ilk,
                    false,
                );
                let mut store_token = true;
                if at_bib_command && command_num == 2i32 {
                    /*n_bib_string */
                    if macro_name_loc == cur_macro_loc {
                        store_token = false;
                        macro_warn_print();
                        log!("used in its own definition\n");
                        bib_warn_print();
                    }
                }
                if !hash_found {
                    store_token = false;
                    macro_warn_print();
                    log!("undefined\n");
                    bib_warn_print();
                }
                if store_token {
                    /*261: */
                    let mut tmp_ptr =
                        *str_start.offset(*ilk_info.offset(macro_name_loc as isize) as isize); /*space */
                    let mut tmp_end_ptr = *str_start
                        .offset((*ilk_info.offset(macro_name_loc as isize) + 1i32) as isize);
                    if ex_buf_ptr == 0i32
                        && tmp_ptr < tmp_end_ptr
                        && lex_class[*str_pool.offset(tmp_ptr as isize) as usize]
                            == LexType::WhiteSpace
                    {
                        if ex_buf_ptr == buf_size {
                            bib_field_too_long_print();
                            return false;
                        } else {
                            *ex_buf.offset(ex_buf_ptr as isize) = 32i32 as u8;
                            ex_buf_ptr += 1i32
                        }
                        tmp_ptr += 1i32;
                        while tmp_ptr < tmp_end_ptr
                            && lex_class[*str_pool.offset(tmp_ptr as isize) as usize]
                                == LexType::WhiteSpace
                        {
                            tmp_ptr += 1i32
                        }
                    }
                    while tmp_ptr < tmp_end_ptr {
                        if lex_class[*str_pool.offset(tmp_ptr as isize) as usize]
                            != LexType::WhiteSpace
                        {
                            /*white_space */
                            if ex_buf_ptr == buf_size {
                                bib_field_too_long_print();
                                return false;
                            } else {
                                *ex_buf.offset(ex_buf_ptr as isize) =
                                    *str_pool.offset(tmp_ptr as isize);
                                ex_buf_ptr += 1i32
                            }
                        } else if *ex_buf.offset((ex_buf_ptr - 1i32) as isize) as i32 != 32i32 {
                            /*space */
                            if ex_buf_ptr == buf_size {
                                bib_field_too_long_print(); /*space */
                                return false;
                            } else {
                                *ex_buf.offset(ex_buf_ptr as isize) = 32i32 as u8;
                                ex_buf_ptr += 1i32
                            }
                        }
                        tmp_ptr += 1i32
                    }
                }
            }
        }
    }
    if !eat_bib_white_space() {
        eat_bib_print();
        return false;
    }
    true
}
unsafe fn scan_and_store_the_field_value_and_eat_white() -> bool {
    ex_buf_ptr = 0i32;
    if !scan_a_field_token_and_eat_white() {
        return false;
    }
    while *buffer.offset(buf_ptr2 as isize) as i32 == 35i32 {
        /*concat_char */
        buf_ptr2 += 1i32;
        if !eat_bib_white_space() {
            eat_bib_print();
            return false;
        }
        if !scan_a_field_token_and_eat_white() {
            return false;
        }
    }
    if store_field {
        /*262: */
        if !at_bib_command
            && ex_buf_ptr > 0i32
            && *ex_buf.offset((ex_buf_ptr - 1i32) as isize) as i32 == 32i32
        {
            /*space */
            ex_buf_ptr -= 1i32
        } /*str_literal */
        if !at_bib_command && *ex_buf.offset(0) as i32 == 32i32 && ex_buf_ptr > 0i32 {
            ex_buf_xptr = 1i32
        } else {
            ex_buf_xptr = 0i32
        } /*264: */
        field_val_loc = str_lookup(
            ex_buf,
            ex_buf_xptr,
            ex_buf_ptr - ex_buf_xptr,
            0i32 as str_ilk,
            true,
        );
        *fn_type.offset(field_val_loc as isize) = FnClass::StrLiteral;
        if at_bib_command {
            /*263: */
            match command_num {
                1 => {
                    *s_preamble.add(preamble_ptr) = *hash_text.offset(field_val_loc as isize);
                    preamble_ptr += 1;
                }
                2 => {
                    *ilk_info.offset(cur_macro_loc as isize) =
                        *hash_text.offset(field_val_loc as isize)
                }
                _ => bib_cmd_confusion(),
            }
        } else {
            field_ptr = entry_cite_ptr * num_fields + *ilk_info.offset(field_name_loc as isize);
            if field_ptr >= max_fields {
                log!("field_info index is out of range");
                print_confusion();
                panic!();
            }
            if *field_info.offset(field_ptr as isize) != 0i32 {
                /*missing */
                log!("Warning--I\'m ignoring ");
                print_a_pool_str(*cite_list.offset(entry_cite_ptr as isize));
                log!("\'s extra \"");
                print_a_pool_str(*hash_text.offset(field_name_loc as isize));
                log!("\" field\n");
                bib_warn_print();
            } else {
                *field_info.offset(field_ptr as isize) = *hash_text.offset(field_val_loc as isize);
                if *ilk_info.offset(field_name_loc as isize) == crossref_num && !all_entries {
                    /*265: */
                    let mut tmp_ptr = ex_buf_xptr;
                    while tmp_ptr < ex_buf_ptr {
                        *out_buf.offset(tmp_ptr as isize) = *ex_buf.offset(tmp_ptr as isize);
                        tmp_ptr += 1i32
                    }
                    lower_case(out_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr);
                    lc_cite_loc = str_lookup(
                        out_buf,
                        ex_buf_xptr,
                        ex_buf_ptr - ex_buf_xptr,
                        10i32 as str_ilk,
                        true,
                    );
                    if hash_found {
                        cite_loc = *ilk_info.offset(lc_cite_loc as isize);
                        if *ilk_info.offset(cite_loc as isize) >= old_num_cites {
                            *cite_info.offset(*ilk_info.offset(cite_loc as isize) as isize) =
                                *cite_info.offset(*ilk_info.offset(cite_loc as isize) as isize)
                                    + 1i32
                        }
                    } else {
                        cite_loc = str_lookup(
                            ex_buf,
                            ex_buf_xptr,
                            ex_buf_ptr - ex_buf_xptr,
                            9i32 as str_ilk,
                            true,
                        );
                        if hash_found {
                            hash_cite_confusion();
                        }
                        add_database_cite(&mut cite_ptr);
                        *cite_info.offset(*ilk_info.offset(cite_loc as isize) as isize) = 1i32
                    }
                }
            }
        }
    }
    true
}
unsafe fn decr_brace_level(mut pop_lit_var: str_number) {
    if brace_level == 0i32 {
        braces_unbalanced_complaint(pop_lit_var);
    } else {
        brace_level -= 1i32
    };
}
unsafe fn check_brace_level(mut pop_lit_var: str_number) {
    if brace_level > 0i32 {
        braces_unbalanced_complaint(pop_lit_var);
    };
}
unsafe fn name_scan_for_and(mut pop_lit_var: str_number) {
    brace_level = 0i32;
    let mut preceding_white = false;
    let mut and_found = false;
    while !and_found && ex_buf_ptr < ex_buf_length {
        match *ex_buf.offset(ex_buf_ptr as isize) as i32 {
            97 | 65 => {
                ex_buf_ptr += 1i32;
                if preceding_white {
                    /*387: */
                    if ex_buf_ptr <= ex_buf_length - 3i32
                        && (*ex_buf.offset(ex_buf_ptr as isize) as i32 == 'n' as i32
                            || *ex_buf.offset(ex_buf_ptr as isize) as i32 == 'N' as i32)
                        && (*ex_buf.offset((ex_buf_ptr + 1i32) as isize) as i32 == 'd' as i32
                            || *ex_buf.offset((ex_buf_ptr + 1i32) as isize) as i32 == 'D' as i32)
                        && lex_class[*ex_buf.offset((ex_buf_ptr + 2i32) as isize) as usize]
                            == LexType::WhiteSpace
                    {
                        ex_buf_ptr += 2;
                        and_found = true;
                    }
                }
                preceding_white = false
            }
            123 => {
                brace_level += 1i32;
                ex_buf_ptr += 1i32;
                while brace_level > 0i32 && ex_buf_ptr < ex_buf_length {
                    if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 125i32 {
                        /*right_brace */
                        brace_level -= 1i32
                    } else if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 123i32 {
                        /*left_brace */
                        brace_level += 1i32
                    }
                    ex_buf_ptr += 1i32
                }
                preceding_white = false
            }
            125 => {
                decr_brace_level(pop_lit_var);
                ex_buf_ptr += 1i32;
                preceding_white = false
            }
            _ => {
                if lex_class[*ex_buf.offset(ex_buf_ptr as isize) as usize] == LexType::WhiteSpace {
                    /*white_space */
                    ex_buf_ptr += 1i32;
                    preceding_white = true
                } else {
                    ex_buf_ptr += 1i32;
                    preceding_white = false
                }
            }
        }
    }
    check_brace_level(pop_lit_var);
}
unsafe fn von_token_found() -> bool {
    let mut nm_brace_level = 0i32;
    while name_bf_ptr < name_bf_xptr {
        if *sv_buffer.offset(name_bf_ptr as isize) as i32 >= 'A' as i32
            && *sv_buffer.offset(name_bf_ptr as isize) as i32 <= 'Z' as i32
        {
            return false;
        } else if *sv_buffer.offset(name_bf_ptr as isize) as i32 >= 'a' as i32
            && *sv_buffer.offset(name_bf_ptr as isize) as i32 <= 'z' as i32
        {
            return true;
        } else if *sv_buffer.offset(name_bf_ptr as isize) as i32 == 123i32 {
            /*left_brace */
            nm_brace_level += 1i32; /*401: */
            name_bf_ptr += 1i32;
            if name_bf_ptr + 2i32 < name_bf_xptr
                && *sv_buffer.offset(name_bf_ptr as isize) as i32 == 92i32
            {
                /*399: */
                name_bf_ptr += 1i32;
                name_bf_yptr = name_bf_ptr;
                while name_bf_ptr < name_bf_xptr
                    && char::from(*sv_buffer.offset(name_bf_ptr as isize)).is_ascii_alphabetic()
                {
                    name_bf_ptr += 1;
                }
                control_seq_loc = str_lookup(
                    sv_buffer,
                    name_bf_yptr,
                    name_bf_ptr - name_bf_yptr,
                    14i32 as str_ilk,
                    false,
                );
                if hash_found {
                    /*400: */
                    match *ilk_info.offset(control_seq_loc as isize) {
                        3 | 5 | 7 | 9 | 11 => return false,
                        0 | 1 | 2 | 4 | 6 | 8 | 10 | 12 => return true,
                        _ => {
                            log!("Control-sequence hash error");
                            print_confusion();
                            panic!();
                        }
                    }
                }
                while name_bf_ptr < name_bf_xptr && nm_brace_level > 0i32 {
                    if *sv_buffer.offset(name_bf_ptr as isize) as i32 >= 'A' as i32
                        && *sv_buffer.offset(name_bf_ptr as isize) as i32 <= 'Z' as i32
                    {
                        return false;
                    } else if *sv_buffer.offset(name_bf_ptr as isize) as i32 >= 'a' as i32
                        && *sv_buffer.offset(name_bf_ptr as isize) as i32 <= 'z' as i32
                    {
                        return true;
                    } else if *sv_buffer.offset(name_bf_ptr as isize) as i32 == 125i32 {
                        /*right_brace */
                        nm_brace_level -= 1i32
                    } else if *sv_buffer.offset(name_bf_ptr as isize) as i32 == 123i32 {
                        /*left_brace */
                        nm_brace_level += 1i32
                    }
                    name_bf_ptr += 1i32
                }
                return false;
            } else {
                while nm_brace_level > 0i32 && name_bf_ptr < name_bf_xptr {
                    if *sv_buffer.offset(name_bf_ptr as isize) as i32 == 125i32 {
                        /*right_brace */
                        nm_brace_level -= 1i32
                    } else if *sv_buffer.offset(name_bf_ptr as isize) as i32 == 123i32 {
                        /*left_brace */
                        nm_brace_level += 1i32
                    }
                    name_bf_ptr += 1i32
                }
            }
        } else {
            name_bf_ptr += 1i32
        }
    }
    false
}
unsafe fn von_name_ends_and_last_name_starts_stuff() {
    von_end = last_end - 1i32;
    while von_end > von_start {
        name_bf_ptr = *name_tok.offset((von_end - 1i32) as isize);
        name_bf_xptr = *name_tok.offset(von_end as isize);
        if von_token_found() {
            return;
        }
        von_end -= 1i32
    }
}
unsafe fn skip_stuff_at_sp_brace_level_greater_than_one() {
    while sp_brace_level > 1i32 && sp_ptr < sp_end {
        if *str_pool.offset(sp_ptr as isize) as i32 == 125i32 {
            /*right_brace */
            sp_brace_level -= 1i32
        } else if *str_pool.offset(sp_ptr as isize) as i32 == 123i32 {
            /*left_brace */
            sp_brace_level += 1i32
        }
        sp_ptr += 1i32
    }
}
unsafe fn brace_lvl_one_letters_complaint() {
    log!("The format string \"");
    print_a_pool_str(pop_lit1);
    log!("\" has an illegal brace-level-1 letter");
    bst_ex_warn_print();
}
unsafe fn enough_text_chars(mut enough_chars: buf_pointer) -> bool {
    let mut num_text_chars = 0i32;
    ex_buf_yptr = ex_buf_xptr;
    while ex_buf_yptr < ex_buf_ptr && num_text_chars < enough_chars {
        ex_buf_yptr += 1i32;
        if *ex_buf.offset((ex_buf_yptr - 1i32) as isize) as i32 == 123i32 {
            /*left_brace */
            brace_level += 1i32;
            if brace_level == 1i32
                && ex_buf_yptr < ex_buf_ptr
                && *ex_buf.offset(ex_buf_yptr as isize) as i32 == 92i32
            {
                /*backslash */
                ex_buf_yptr += 1i32;
                while ex_buf_yptr < ex_buf_ptr && brace_level > 0i32 {
                    if *ex_buf.offset(ex_buf_yptr as isize) as i32 == 125i32 {
                        /*right_brace */
                        brace_level -= 1i32
                    } else if *ex_buf.offset(ex_buf_yptr as isize) as i32 == 123i32 {
                        /*left_brace */
                        brace_level += 1i32
                    }
                    ex_buf_yptr += 1i32
                }
            }
        } else if *ex_buf.offset((ex_buf_yptr - 1i32) as isize) as i32 == 125i32 {
            /*right_brace */
            brace_level -= 1i32
        }
        num_text_chars += 1i32
    }
    num_text_chars >= enough_chars
}
unsafe fn figure_out_the_formatted_name() {
    ex_buf_ptr = 0i32;
    sp_brace_level = 0i32;
    sp_ptr = *str_start.offset(pop_lit1 as isize);
    sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
    let mut last_token = 0;
    let mut cur_token = 0;
    while sp_ptr < sp_end {
        if *str_pool.offset(sp_ptr as isize) as i32 == 123i32 {
            /*left_brace */
            sp_brace_level += 1i32;
            sp_ptr += 1i32;
            sp_xptr1 = sp_ptr;
            let mut alpha_found = false;
            let mut double_letter = false;
            let mut end_of_group = false;
            let mut to_be_written = true;
            while !end_of_group && sp_ptr < sp_end {
                if (*str_pool.offset(sp_ptr as isize)).is_ascii_alphabetic() {
                    sp_ptr += 1i32;
                    if alpha_found {
                        brace_lvl_one_letters_complaint();
                        to_be_written = false
                    } else {
                        match *str_pool.offset((sp_ptr - 1i32) as isize) as i32 {
                            102 | 70 => {
                                cur_token = first_start;
                                last_token = first_end;
                                if cur_token == last_token {
                                    to_be_written = false
                                }
                                if *str_pool.offset(sp_ptr as isize) as i32 == 'f' as i32
                                    || *str_pool.offset(sp_ptr as isize) as i32 == 'F' as i32
                                {
                                    double_letter = true
                                }
                            }
                            118 | 86 => {
                                cur_token = von_start;
                                last_token = von_end;
                                if cur_token == last_token {
                                    to_be_written = false
                                }
                                if *str_pool.offset(sp_ptr as isize) as i32 == 'v' as i32
                                    || *str_pool.offset(sp_ptr as isize) as i32 == 'V' as i32
                                {
                                    double_letter = true
                                }
                            }
                            108 | 76 => {
                                cur_token = von_end;
                                last_token = last_end;
                                if cur_token == last_token {
                                    to_be_written = false
                                }
                                if *str_pool.offset(sp_ptr as isize) as i32 == 'l' as i32
                                    || *str_pool.offset(sp_ptr as isize) as i32 == 'L' as i32
                                {
                                    double_letter = true
                                }
                            }
                            106 | 74 => {
                                cur_token = last_end;
                                last_token = jr_end;
                                if cur_token == last_token {
                                    to_be_written = false
                                }
                                if *str_pool.offset(sp_ptr as isize) as i32 == 'j' as i32
                                    || *str_pool.offset(sp_ptr as isize) as i32 == 'J' as i32
                                {
                                    double_letter = true
                                }
                            }
                            _ => {
                                brace_lvl_one_letters_complaint();
                                to_be_written = false
                            }
                        }
                        if double_letter {
                            sp_ptr += 1i32
                        }
                    }
                    alpha_found = true
                } else if *str_pool.offset(sp_ptr as isize) as i32 == 125i32 {
                    /*right_brace */
                    sp_brace_level -= 1i32;
                    sp_ptr += 1i32;
                    end_of_group = true
                } else if *str_pool.offset(sp_ptr as isize) as i32 == 123i32 {
                    /*left_brace */
                    sp_brace_level += 1i32;
                    sp_ptr += 1i32;
                    skip_stuff_at_sp_brace_level_greater_than_one();
                } else {
                    sp_ptr += 1i32
                }
            }
            if !(end_of_group && to_be_written) {
                continue;
            }
            /*412: */
            ex_buf_xptr = ex_buf_ptr;
            sp_ptr = sp_xptr1;
            sp_brace_level = 1i32;
            while sp_brace_level > 0i32 {
                if (*str_pool.offset(sp_ptr as isize)).is_ascii_alphabetic()
                    && sp_brace_level == 1i32
                {
                    sp_ptr += 1i32;
                    if double_letter {
                        sp_ptr += 1i32
                    }
                    let mut use_default = true;
                    sp_xptr2 = sp_ptr;
                    if *str_pool.offset(sp_ptr as isize) as i32 == 123i32 {
                        /*left_brace */
                        use_default = false; /*416: */
                        sp_brace_level += 1i32;
                        sp_ptr += 1i32;
                        sp_xptr1 = sp_ptr;
                        skip_stuff_at_sp_brace_level_greater_than_one();
                        sp_xptr2 = sp_ptr - 1i32
                    }
                    while cur_token < last_token {
                        if double_letter {
                            /*415: */
                            name_bf_ptr = *name_tok.offset(cur_token as isize);
                            name_bf_xptr = *name_tok.offset((cur_token + 1i32) as isize);
                            if ex_buf_length + (name_bf_xptr - name_bf_ptr) > buf_size {
                                buffer_overflow();
                            }
                            while name_bf_ptr < name_bf_xptr {
                                *ex_buf.offset(ex_buf_ptr as isize) =
                                    *sv_buffer.offset(name_bf_ptr as isize);
                                ex_buf_ptr += 1i32;
                                name_bf_ptr += 1i32
                            }
                        } else {
                            name_bf_ptr = *name_tok.offset(cur_token as isize);
                            name_bf_xptr = *name_tok.offset((cur_token + 1i32) as isize);
                            while name_bf_ptr < name_bf_xptr {
                                if (*sv_buffer.offset(name_bf_ptr as isize)).is_ascii_alphabetic() {
                                    if ex_buf_ptr == buf_size {
                                        buffer_overflow();
                                    }
                                    *ex_buf.offset(ex_buf_ptr as isize) =
                                        *sv_buffer.offset(name_bf_ptr as isize);
                                    ex_buf_ptr += 1i32;
                                    break;
                                } else {
                                    if name_bf_ptr + 1i32 < name_bf_xptr
                                        && *sv_buffer.offset(name_bf_ptr as isize) as i32 == 123i32
                                        && *sv_buffer.offset((name_bf_ptr + 1i32) as isize) as i32
                                            == 92i32
                                    {
                                        /*backslash */
                                        /*417: */
                                        if ex_buf_ptr + 2i32 > buf_size {
                                            buffer_overflow(); /*left_brace */
                                        } /*backslash */
                                        *ex_buf.offset(ex_buf_ptr as isize) = 123i32 as u8;
                                        ex_buf_ptr += 1i32;
                                        *ex_buf.offset(ex_buf_ptr as isize) = 92i32 as u8;
                                        ex_buf_ptr += 1i32;
                                        name_bf_ptr += 2i32;
                                        let mut nm_brace_level = 1i32;
                                        while name_bf_ptr < name_bf_xptr && nm_brace_level > 0i32 {
                                            if *sv_buffer.offset(name_bf_ptr as isize) as i32
                                                == 125i32
                                            {
                                                /*right_brace */
                                                nm_brace_level -= 1i32
                                            } else if *sv_buffer.offset(name_bf_ptr as isize) as i32
                                                == 123i32
                                            {
                                                /*left_brace */
                                                nm_brace_level += 1i32
                                            }
                                            if ex_buf_ptr == buf_size {
                                                buffer_overflow();
                                            }
                                            *ex_buf.offset(ex_buf_ptr as isize) =
                                                *sv_buffer.offset(name_bf_ptr as isize);
                                            ex_buf_ptr += 1i32;
                                            name_bf_ptr += 1i32
                                        }
                                        break;
                                    }
                                    name_bf_ptr += 1i32
                                }
                            }
                        }
                        cur_token += 1i32;
                        if cur_token < last_token {
                            /*418: */
                            if use_default {
                                if !double_letter {
                                    if ex_buf_ptr == buf_size {
                                        buffer_overflow(); /*period */
                                    }
                                    *ex_buf.offset(ex_buf_ptr as isize) = 46i32 as u8;
                                    ex_buf_ptr += 1i32
                                }
                                if lex_class[*name_sep_char.offset(cur_token as isize) as usize]
                                    == LexType::SepChar
                                {
                                    /*sep_char */
                                    if ex_buf_ptr == buf_size {
                                        buffer_overflow(); /*tie */
                                    } /*space */
                                    *ex_buf.offset(ex_buf_ptr as isize) =
                                        *name_sep_char.offset(cur_token as isize);
                                    ex_buf_ptr += 1i32
                                } else if cur_token == last_token - 1i32 || !enough_text_chars(3i32)
                                {
                                    if ex_buf_ptr == buf_size {
                                        buffer_overflow();
                                    }
                                    *ex_buf.offset(ex_buf_ptr as isize) = 126i32 as u8;
                                    ex_buf_ptr += 1i32
                                } else {
                                    if ex_buf_ptr == buf_size {
                                        buffer_overflow();
                                    }
                                    *ex_buf.offset(ex_buf_ptr as isize) = 32i32 as u8;
                                    ex_buf_ptr += 1i32
                                }
                            } else {
                                if ex_buf_length + (sp_xptr2 - sp_xptr1) > buf_size {
                                    buffer_overflow();
                                }
                                sp_ptr = sp_xptr1;
                                while sp_ptr < sp_xptr2 {
                                    *ex_buf.offset(ex_buf_ptr as isize) =
                                        *str_pool.offset(sp_ptr as isize);
                                    ex_buf_ptr += 1i32;
                                    sp_ptr += 1i32
                                }
                            }
                        }
                    }
                    if !use_default {
                        sp_ptr = sp_xptr2 + 1i32
                    }
                } else if *str_pool.offset(sp_ptr as isize) as i32 == 125i32 {
                    /*right_brace */
                    sp_brace_level -= 1i32; /*right_brace */
                    sp_ptr += 1i32;
                    if sp_brace_level > 0i32 {
                        if ex_buf_ptr == buf_size {
                            buffer_overflow();
                        }
                        *ex_buf.offset(ex_buf_ptr as isize) = 125i32 as u8;
                        ex_buf_ptr += 1i32
                    }
                } else if *str_pool.offset(sp_ptr as isize) as i32 == 123i32 {
                    /*left_brace */
                    sp_brace_level += 1i32; /*left_brace */
                    sp_ptr += 1i32;
                    if ex_buf_ptr == buf_size {
                        buffer_overflow();
                    }
                    *ex_buf.offset(ex_buf_ptr as isize) = 123i32 as u8;
                    ex_buf_ptr += 1i32
                } else {
                    if ex_buf_ptr == buf_size {
                        buffer_overflow();
                    }
                    *ex_buf.offset(ex_buf_ptr as isize) = *str_pool.offset(sp_ptr as isize);
                    ex_buf_ptr += 1i32;
                    sp_ptr += 1i32
                }
            }
            if ex_buf_ptr > 0i32 && *ex_buf.offset((ex_buf_ptr - 1i32) as isize) as i32 == 126i32 {
                /*tie */
                /*420: */
                ex_buf_ptr -= 1i32; /*space */
                if *ex_buf.offset((ex_buf_ptr - 1i32) as isize) as i32 != 126i32 {
                    if !enough_text_chars(3i32) {
                        ex_buf_ptr += 1i32
                    } else {
                        *ex_buf.offset(ex_buf_ptr as isize) = 32i32 as u8;
                        ex_buf_ptr += 1i32
                    }
                }
            }
        } else if *str_pool.offset(sp_ptr as isize) as i32 == 125i32 {
            /*right_brace */
            braces_unbalanced_complaint(pop_lit1);
            sp_ptr += 1i32
        } else {
            if ex_buf_ptr == buf_size {
                buffer_overflow();
            }
            *ex_buf.offset(ex_buf_ptr as isize) = *str_pool.offset(sp_ptr as isize);
            ex_buf_ptr += 1i32;
            sp_ptr += 1i32
        }
    }
    if sp_brace_level > 0i32 {
        braces_unbalanced_complaint(pop_lit1);
    }
    ex_buf_length = ex_buf_ptr;
}
unsafe fn push_lit_stk(mut push_lt: i32, mut push_type: StkType) {
    *lit_stack.offset(lit_stk_ptr as isize) = push_lt;
    *lit_stk_type.offset(lit_stk_ptr as isize) = push_type;
    if lit_stk_ptr == lit_stk_size {
        lit_stack = xrealloc(
            lit_stack as *mut libc::c_void,
            ((lit_stk_size + 100i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<i32>() as u64) as _,
        ) as *mut i32;
        lit_stk_type = xrealloc(
            lit_stk_type as *mut libc::c_void,
            ((lit_stk_size + 100i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<StkType>() as u64) as _,
        ) as *mut StkType;
        lit_stk_size += 100i32
    }
    lit_stk_ptr += 1i32;
}
unsafe fn pop_lit_stk(mut pop_lit: *mut i32, mut pop_type: *mut StkType) {
    if lit_stk_ptr == 0i32 {
        log!("You can\'t pop an empty literal stack");
        bst_ex_warn_print();
        *pop_type = StkType::Empty;
    /*stk_empty */
    } else {
        lit_stk_ptr -= 1i32;
        *pop_lit = *lit_stack.offset(lit_stk_ptr as isize);
        *pop_type = *lit_stk_type.offset(lit_stk_ptr as isize);
        if *pop_type as i32 == 1i32 {
            /*stk_str */
            if *pop_lit >= cmd_str_ptr {
                if *pop_lit != str_ptr - 1i32 {
                    log!("Nontop top of string stack");
                    print_confusion();
                    panic!();
                }
                str_ptr -= 1i32;
                pool_ptr = *str_start.offset(str_ptr as isize)
            }
        }
    };
}
unsafe fn print_wrong_stk_lit(mut stk_lt: i32, mut stk_tp1: StkType, mut stk_tp2: StkType) {
    if stk_tp1 != StkType::Empty {
        /*stk_empty */
        print_stk_lit(stk_lt, stk_tp1);
        match stk_tp2 {
            StkType::Int => log!(", not an integer,"),
            StkType::Str => log!(", not a string,"),
            StkType::Fn => log!(", not a function,"),
            StkType::FieldMissing | StkType::Empty => illegl_literal_confusion(),
        }
        bst_ex_warn_print();
    };
}
unsafe fn pop_top_and_print() {
    let mut stk_lt: i32 = 0;
    let mut stk_tp = StkType::Int;
    pop_lit_stk(&mut stk_lt, &mut stk_tp);
    if stk_tp == StkType::Empty {
        log!("Empty literal\n");
    } else {
        print_lit(stk_lt, stk_tp);
    };
}
unsafe fn pop_whole_stack() {
    while lit_stk_ptr > 0i32 {
        pop_top_and_print();
    }
}
unsafe fn init_command_execution() {
    lit_stk_ptr = 0i32;
    cmd_str_ptr = str_ptr;
}
unsafe fn check_command_execution() {
    if lit_stk_ptr != 0i32 {
        log!("ptr={}, stack=\n", lit_stk_ptr);
        pop_whole_stack();
        log!("---the literal stack isn\'t empty");
        bst_ex_warn_print();
    }
    if cmd_str_ptr != str_ptr {
        log!("Nonempty empty string stack");
        print_confusion();
        panic!();
    };
}
unsafe fn add_pool_buf_and_push() {
    while pool_ptr + ex_buf_length > pool_size {
        pool_overflow();
    }
    ex_buf_ptr = 0i32;
    while ex_buf_ptr < ex_buf_length {
        *str_pool.offset(pool_ptr as isize) = *ex_buf.offset(ex_buf_ptr as isize);
        pool_ptr += 1i32;
        ex_buf_ptr += 1i32
    }
    push_lit_stk(make_string(), StkType::Str);
}
unsafe fn add_buf_pool(mut p_str: str_number) {
    let s = get_string_from_pool(p_str);
    if ex_buf_length + s.len() as i32 > buf_size {
        buffer_overflow();
    }
    ex_buf_ptr = ex_buf_length;
    for c in s.iter() {
        *ex_buf.offset(ex_buf_ptr as isize) = *c;
        ex_buf_ptr += 1i32;
    }
    ex_buf_length = ex_buf_ptr;
}
unsafe fn add_out_pool(mut p_str: str_number) {
    let mut break_ptr: buf_pointer = 0;
    let mut end_ptr: buf_pointer = 0;
    let mut break_pt_found: bool = false;
    let mut unbreakable_tail: bool = false;
    let s = get_string_from_pool(p_str);
    while out_buf_length + s.len() as i32 > buf_size {
        buffer_overflow();
    }
    out_buf_ptr = out_buf_length;
    for c in s.iter() {
        *out_buf.offset(out_buf_ptr as isize) = *c;
        out_buf_ptr += 1i32
    }
    out_buf_length = out_buf_ptr;
    unbreakable_tail = false;
    while out_buf_length > 79i32 && !unbreakable_tail {
        /*324: */
        end_ptr = out_buf_length;
        out_buf_ptr = 79i32;
        break_pt_found = false;
        while lex_class[*out_buf.offset(out_buf_ptr as isize) as usize] != LexType::WhiteSpace
            && out_buf_ptr >= 3i32
        {
            out_buf_ptr -= 1i32
        }
        if out_buf_ptr == 3i32 - 1i32 {
            /*325: */
            out_buf_ptr = 79i32 + 1i32;
            while out_buf_ptr < end_ptr {
                if !(lex_class[*out_buf.offset(out_buf_ptr as isize) as usize]
                    != LexType::WhiteSpace)
                {
                    break;
                }
                /*white_space */
                out_buf_ptr += 1i32
            }
            /*loop1_exit */
            if out_buf_ptr == end_ptr {
                unbreakable_tail = true
            } else {
                break_pt_found = true;
                while out_buf_ptr + 1i32 < end_ptr {
                    if !(lex_class[*out_buf.offset((out_buf_ptr + 1i32) as isize) as usize]
                        == LexType::WhiteSpace)
                    {
                        break;
                    }
                    /*white_space */
                    out_buf_ptr += 1i32
                }
            }
        } else {
            break_pt_found = true
        } /*space */
        if break_pt_found {
            out_buf_length = out_buf_ptr; /*space */
            break_ptr = out_buf_length + 1i32;
            output_bbl_line();
            *out_buf.offset(0) = 32i32 as u8;
            *out_buf.offset(1) = 32i32 as u8;
            out_buf_ptr = 2i32;
            let mut tmp_ptr = break_ptr;
            while tmp_ptr < end_ptr {
                *out_buf.offset(out_buf_ptr as isize) = *out_buf.offset(tmp_ptr as isize);
                out_buf_ptr += 1i32;
                tmp_ptr += 1i32
            }
            out_buf_length = end_ptr - break_ptr + 2i32
        }
    }
}
unsafe fn x_equals() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != pop_typ2 as i32 {
        if pop_typ1 as i32 != 4i32 && pop_typ2 as i32 != 4i32 {
            print_stk_lit(pop_lit1, pop_typ1);
            log!(", ");
            print_stk_lit(pop_lit2, pop_typ2);
            putc_log('\n' as i32);
            log!("---they aren\'t the same literal types");
            bst_ex_warn_print();
        }
        push_lit_stk(0i32, StkType::Int);
    } else if pop_typ1 as i32 != 0i32 && pop_typ1 as i32 != 1i32 {
        if pop_typ1 as i32 != 4i32 {
            /*stk_empty */
            print_stk_lit(pop_lit1, pop_typ1);
            log!(", not an integer or a string,");
            bst_ex_warn_print();
        }
        push_lit_stk(0i32, StkType::Int);
    } else if pop_typ1 as i32 == 0i32 {
        /*stk_int */
        if pop_lit2 == pop_lit1 {
            push_lit_stk(1i32, StkType::Int);
        } else {
            push_lit_stk(0i32, StkType::Int);
        }
    } else if str_eq_str(pop_lit2, pop_lit1) {
        push_lit_stk(1i32, StkType::Int);
    } else {
        push_lit_stk(0i32, StkType::Int);
    };
}
unsafe fn x_greater_than() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Int);
        push_lit_stk(0i32, StkType::Int);
    } else if pop_typ2 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit2, pop_typ2, StkType::Int);
        push_lit_stk(0i32, StkType::Int);
    } else if pop_lit2 > pop_lit1 {
        push_lit_stk(1i32, StkType::Int);
    } else {
        push_lit_stk(0i32, StkType::Int);
    };
}
unsafe fn x_less_than() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 != StkType::Int {
        /*stk_int */
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Int);
        push_lit_stk(0i32, StkType::Int);
    } else if pop_typ2 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit2, pop_typ2, StkType::Int);
        push_lit_stk(0i32, StkType::Int);
    } else if pop_lit2 < pop_lit1 {
        push_lit_stk(1i32, StkType::Int);
    } else {
        push_lit_stk(0i32, StkType::Int);
    };
}
unsafe fn x_plus() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Int);
        push_lit_stk(0i32, StkType::Int);
    } else if pop_typ2 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit2, pop_typ2, StkType::Int);
        push_lit_stk(0i32, StkType::Int);
    } else {
        push_lit_stk(pop_lit2 + pop_lit1, StkType::Int);
    };
}
unsafe fn x_minus() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 != StkType::Int {
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Int);
        push_lit_stk(0i32, StkType::Int);
    } else if pop_typ2 != StkType::Int {
        print_wrong_stk_lit(pop_lit2, pop_typ2, StkType::Int);
        push_lit_stk(0i32, StkType::Int);
    } else {
        push_lit_stk(pop_lit2 - pop_lit1, StkType::Int);
    };
}
unsafe fn x_concatenate() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Str);
        push_lit_stk(s_null, StkType::Str);
    } else if pop_typ2 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit2, pop_typ2, StkType::Str); /*352: */
        push_lit_stk(s_null, StkType::Str); /*353: */
    } else if pop_lit2 >= cmd_str_ptr {
        if pop_lit1 >= cmd_str_ptr {
            *str_start.offset(pop_lit1 as isize) = *str_start.offset((pop_lit1 + 1i32) as isize); /*354: */
            str_ptr += 1i32;
            pool_ptr = *str_start.offset(str_ptr as isize);
            lit_stk_ptr += 1i32
        } else if *str_start.offset((pop_lit2 + 1i32) as isize)
            - *str_start.offset(pop_lit2 as isize)
            == 0i32
        {
            push_lit_stk(pop_lit1, StkType::Str);
        } else {
            pool_ptr = *str_start.offset((pop_lit2 + 1i32) as isize);
            while pool_ptr
                + (*str_start.offset((pop_lit1 + 1i32) as isize)
                    - *str_start.offset(pop_lit1 as isize))
                > pool_size
            {
                pool_overflow();
            }
            sp_ptr = *str_start.offset(pop_lit1 as isize);
            sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
            while sp_ptr < sp_end {
                *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
                pool_ptr += 1i32;
                sp_ptr += 1i32
            }
            push_lit_stk(make_string(), StkType::Str);
        }
    } else if pop_lit1 >= cmd_str_ptr {
        if *str_start.offset((pop_lit2 + 1i32) as isize) - *str_start.offset(pop_lit2 as isize)
            == 0i32
        {
            str_ptr += 1i32;
            pool_ptr = *str_start.offset(str_ptr as isize);
            *lit_stack.offset(lit_stk_ptr as isize) = pop_lit1;
            lit_stk_ptr += 1i32
        } else if *str_start.offset((pop_lit1 + 1i32) as isize)
            - *str_start.offset(pop_lit1 as isize)
            == 0i32
        {
            lit_stk_ptr += 1i32
        } else {
            sp_length = *str_start.offset((pop_lit1 + 1i32) as isize)
                - *str_start.offset(pop_lit1 as isize);
            sp2_length = *str_start.offset((pop_lit2 + 1i32) as isize)
                - *str_start.offset(pop_lit2 as isize);
            while pool_ptr + sp_length + sp2_length > pool_size {
                pool_overflow();
            }
            sp_ptr = *str_start.offset((pop_lit1 + 1i32) as isize);
            sp_end = *str_start.offset(pop_lit1 as isize);
            sp_xptr1 = sp_ptr + sp2_length;
            while sp_ptr > sp_end {
                sp_ptr -= 1i32;
                sp_xptr1 -= 1i32;
                *str_pool.offset(sp_xptr1 as isize) = *str_pool.offset(sp_ptr as isize)
            }
            sp_ptr = *str_start.offset(pop_lit2 as isize);
            sp_end = *str_start.offset((pop_lit2 + 1i32) as isize);
            while sp_ptr < sp_end {
                *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
                pool_ptr += 1i32;
                sp_ptr += 1i32
            }
            pool_ptr += sp_length;
            push_lit_stk(make_string(), StkType::Str);
        }
    } else if *str_start.offset((pop_lit1 + 1i32) as isize) - *str_start.offset(pop_lit1 as isize)
        == 0i32
    {
        lit_stk_ptr += 1i32
    } else if *str_start.offset((pop_lit2 + 1i32) as isize) - *str_start.offset(pop_lit2 as isize)
        == 0i32
    {
        push_lit_stk(pop_lit1, StkType::Str);
    } else {
        while pool_ptr
            + (*str_start.offset((pop_lit1 + 1i32) as isize) - *str_start.offset(pop_lit1 as isize))
            + (*str_start.offset((pop_lit2 + 1i32) as isize) - *str_start.offset(pop_lit2 as isize))
            > pool_size
        {
            pool_overflow();
        }
        sp_ptr = *str_start.offset(pop_lit2 as isize);
        sp_end = *str_start.offset((pop_lit2 + 1i32) as isize);
        while sp_ptr < sp_end {
            *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
            pool_ptr += 1i32;
            sp_ptr += 1i32
        }
        sp_ptr = *str_start.offset(pop_lit1 as isize);
        sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
        while sp_ptr < sp_end {
            *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
            pool_ptr += 1i32;
            sp_ptr += 1i32
        }
        push_lit_stk(make_string(), StkType::Str);
    };
}
unsafe fn x_gets() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 2i32 {
        /*stk_fn */
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Fn);
    } else if !mess_with_entries
        && (*fn_type.offset(pop_lit1 as isize) as i32 == 6i32
            || *fn_type.offset(pop_lit1 as isize) as i32 == 5i32)
    {
        bst_cant_mess_with_entries_print();
    } else {
        match *fn_type.offset(pop_lit1 as isize) as i32 {
            5 => {
                /*
                356: */
                if pop_typ2 != StkType::Int {
                    print_wrong_stk_lit(pop_lit2, pop_typ2, StkType::Int);
                } else {
                    *entry_ints.offset(
                        (cite_ptr * num_ent_ints + *ilk_info.offset(pop_lit1 as isize)) as isize,
                    ) = pop_lit2
                }
            }
            6 => {
                if pop_typ2 != StkType::Str {
                    /*stk_str */
                    print_wrong_stk_lit(pop_lit2, pop_typ2, StkType::Str);
                } else {
                    str_ent_ptr = cite_ptr * num_ent_strs + *ilk_info.offset(pop_lit1 as isize);
                    ent_chr_ptr = 0i32;
                    sp_ptr = *str_start.offset(pop_lit2 as isize);
                    sp_xptr1 = *str_start.offset((pop_lit2 + 1i32) as isize);
                    if sp_xptr1 - sp_ptr > ent_str_size {
                        bst_1print_string_size_exceeded();
                        log!("{}, the entry", ent_str_size);
                        bst_2print_string_size_exceeded();
                        sp_xptr1 = sp_ptr + ent_str_size
                    }
                    while sp_ptr < sp_xptr1 {
                        *entry_strs
                            .offset((str_ent_ptr * (ent_str_size + 1i32) + ent_chr_ptr) as isize) =
                            *str_pool.offset(sp_ptr as isize);
                        ent_chr_ptr += 1i32;
                        sp_ptr += 1i32
                    }
                    *entry_strs
                        .offset((str_ent_ptr * (ent_str_size + 1i32) + ent_chr_ptr) as isize) =
                        127i32 as u8
                    /*end_of_string */
                }
            }
            7 => {
                if pop_typ2 as i32 != 0i32 {
                    /*stk_int */
                    print_wrong_stk_lit(pop_lit2, pop_typ2, StkType::Int);
                } else {
                    *ilk_info.offset(pop_lit1 as isize) = pop_lit2
                }
            }
            8 => {
                if pop_typ2 as i32 != 1i32 {
                    /*stk_str */
                    print_wrong_stk_lit(pop_lit2, pop_typ2, StkType::Str);
                } else {
                    str_glb_ptr = *ilk_info.offset(pop_lit1 as isize);
                    if pop_lit2 < cmd_str_ptr {
                        *glb_str_ptr.offset(str_glb_ptr as isize) = pop_lit2
                    } else {
                        *glb_str_ptr.offset(str_glb_ptr as isize) = 0i32;
                        glob_chr_ptr = 0i32;
                        sp_ptr = *str_start.offset(pop_lit2 as isize);
                        sp_end = *str_start.offset((pop_lit2 + 1i32) as isize);
                        if sp_end - sp_ptr > glob_str_size {
                            bst_1print_string_size_exceeded();
                            log!("{}, the global", glob_str_size);
                            bst_2print_string_size_exceeded();
                            sp_end = sp_ptr + glob_str_size
                        }
                        while sp_ptr < sp_end {
                            *global_strs.offset(
                                (str_glb_ptr * (glob_str_size + 1i32) + glob_chr_ptr) as isize,
                            ) = *str_pool.offset(sp_ptr as isize);
                            glob_chr_ptr += 1i32;
                            sp_ptr += 1i32
                        }
                        *glb_str_end.offset(str_glb_ptr as isize) = glob_chr_ptr
                    }
                }
            }
            _ => {
                log!("You can\'t assign to type ");
                print_fn_class(pop_lit1);
                log!(", a nonvariable function class");
                bst_ex_warn_print();
            }
        }
    };
}
unsafe fn x_add_period() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 != StkType::Str {
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Str);
        push_lit_stk(s_null, StkType::Str);
    } else if get_string_from_pool(pop_lit1).is_empty() {
        push_lit_stk(s_null, StkType::Str);
    } else {
        /*362: */
        sp_ptr = *str_start.offset((pop_lit1 + 1) as isize);
        sp_end = *str_start.offset(pop_lit1 as isize);
        while sp_ptr > sp_end {
            sp_ptr -= 1;
            if *str_pool.offset(sp_ptr as isize) != b'}' {
                break;
            }
        }
        match *str_pool.offset(sp_ptr as isize) {
            b'.' | b'?' | b'!' => {
                if *lit_stack.offset(lit_stk_ptr as isize) >= cmd_str_ptr {
                    str_ptr += 1;
                    pool_ptr = *str_start.offset(str_ptr as isize)
                }
                lit_stk_ptr += 1;
            }
            _ => {
                if pop_lit1 < cmd_str_ptr {
                    while pool_ptr
                        + (*str_start.offset((pop_lit1 + 1i32) as isize)
                            - *str_start.offset(pop_lit1 as isize))
                        + 1i32
                        > pool_size
                    {
                        pool_overflow();
                    }
                    sp_ptr = *str_start.offset(pop_lit1 as isize);
                    sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
                    while sp_ptr < sp_end {
                        *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
                        pool_ptr += 1;
                        sp_ptr += 1;
                    }
                } else {
                    pool_ptr = *str_start.offset((pop_lit1 + 1i32) as isize);
                    while pool_ptr + 1i32 > pool_size {
                        pool_overflow();
                    }
                }
                *str_pool.offset(pool_ptr as isize) = b'.';
                pool_ptr += 1;
                push_lit_stk(make_string(), StkType::Str);
            }
        }
    };
}
unsafe fn x_change_case() {
    let mut current_block: u64;
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Str);
        push_lit_stk(s_null, StkType::Str);
    } else if pop_typ2 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit2, pop_typ2, StkType::Str); /*title_lowers */
        push_lit_stk(s_null, StkType::Str); /*all_lowers */
    } else {
        let mut prev_colon = false;
        let mut conversion_type =
            match *str_pool.offset(*str_start.offset(pop_lit1 as isize) as isize) {
                b't' | b'T' => ConversionType::TitleLowers,
                b'l' | b'L' => ConversionType::AllLowers,
                b'u' | b'U' => ConversionType::AllUppers,
                _ => ConversionType::BadConversion,
            };
        let s = get_string_from_pool(pop_lit1);
        if s.len() != 1 || conversion_type == ConversionType::BadConversion {
            conversion_type = ConversionType::BadConversion;
            print_a_pool_str(pop_lit1);
            log!(" is an illegal case-conversion string");
            bst_ex_warn_print();
        }
        ex_buf_length = 0i32;
        add_buf_pool(pop_lit2);
        brace_level = 0i32;
        ex_buf_ptr = 0i32;
        while ex_buf_ptr < ex_buf_length {
            if *ex_buf.offset(ex_buf_ptr as isize) == b'{' {
                /*left_brace */
                brace_level += 1i32;
                if brace_level == 1i32
                    && ex_buf_ptr + 4i32 <= ex_buf_length
                    && *ex_buf.offset((ex_buf_ptr + 1i32) as isize) == b'\\'
                {
                    if conversion_type == ConversionType::TitleLowers {
                        if ex_buf_ptr == 0i32 {
                            current_block = 17_089_879_097_653_631_793;
                        } else if prev_colon
                            && lex_class[*ex_buf.offset((ex_buf_ptr - 1i32) as isize) as usize]
                                == LexType::WhiteSpace
                        {
                            current_block = 17_089_879_097_653_631_793;
                        } else {
                            current_block = 6_417_057_564_578_538_666;
                        }
                    } else {
                        current_block = 6_417_057_564_578_538_666;
                    }
                    match current_block {
                        17_089_879_097_653_631_793 => {}
                        _ => {
                            ex_buf_ptr += 1i32;
                            while ex_buf_ptr < ex_buf_length && brace_level > 0i32 {
                                ex_buf_ptr += 1i32;
                                ex_buf_xptr = ex_buf_ptr;
                                while ex_buf_ptr < ex_buf_length
                                    && (*ex_buf.offset(ex_buf_ptr as isize)).is_ascii_alphabetic()
                                {
                                    ex_buf_ptr += 1i32
                                }
                                control_seq_loc = str_lookup(
                                    ex_buf,
                                    ex_buf_xptr,
                                    ex_buf_ptr - ex_buf_xptr,
                                    14i32 as str_ilk,
                                    false,
                                );
                                if hash_found {
                                    /*373: */
                                    match conversion_type {
                                        ConversionType::TitleLowers | ConversionType::AllLowers => {
                                            match *ilk_info.offset(control_seq_loc as isize) {
                                                11 | 9 | 3 | 5 | 7 => {
                                                    lower_case(
                                                        ex_buf,
                                                        ex_buf_xptr,
                                                        ex_buf_ptr - ex_buf_xptr,
                                                    );
                                                }
                                                _ => {}
                                            }
                                        }
                                        ConversionType::AllUppers => {
                                            match *ilk_info.offset(control_seq_loc as isize) {
                                                10 | 8 | 2 | 4 | 6 => {
                                                    upper_case(
                                                        ex_buf,
                                                        ex_buf_xptr,
                                                        ex_buf_ptr - ex_buf_xptr,
                                                    );
                                                }
                                                0 | 1 | 12 => {
                                                    upper_case(
                                                        ex_buf,
                                                        ex_buf_xptr,
                                                        ex_buf_ptr - ex_buf_xptr,
                                                    );
                                                    while ex_buf_xptr < ex_buf_ptr {
                                                        *ex_buf.offset(
                                                            (ex_buf_xptr - 1i32) as isize,
                                                        ) = *ex_buf.offset(ex_buf_xptr as isize);
                                                        ex_buf_xptr += 1i32
                                                    }
                                                    ex_buf_xptr -= 1i32;
                                                    while ex_buf_ptr < ex_buf_length
                                                        && lex_class[*ex_buf
                                                            .offset(ex_buf_ptr as isize)
                                                            as usize]
                                                            == LexType::WhiteSpace
                                                    {
                                                        ex_buf_ptr += 1i32
                                                    }
                                                    let mut tmp_ptr = ex_buf_ptr;
                                                    while tmp_ptr < ex_buf_length {
                                                        *ex_buf.offset(
                                                            (tmp_ptr - (ex_buf_ptr - ex_buf_xptr))
                                                                as isize,
                                                        ) = *ex_buf.offset(tmp_ptr as isize);
                                                        tmp_ptr += 1i32
                                                    }
                                                    ex_buf_length =
                                                        tmp_ptr - (ex_buf_ptr - ex_buf_xptr);
                                                    ex_buf_ptr = ex_buf_xptr
                                                }
                                                _ => {}
                                            }
                                        }
                                        ConversionType::BadConversion => {}
                                    }
                                }
                                ex_buf_xptr = ex_buf_ptr;
                                while ex_buf_ptr < ex_buf_length
                                    && brace_level > 0i32
                                    && *ex_buf.offset(ex_buf_ptr as isize) as i32 != 92i32
                                {
                                    if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 125i32 {
                                        /*right_brace */
                                        brace_level -= 1i32
                                    } else if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 123i32 {
                                        /*left_brace */
                                        brace_level += 1i32
                                    }
                                    ex_buf_ptr += 1i32
                                }
                                match conversion_type {
                                    ConversionType::AllLowers | ConversionType::TitleLowers => {
                                        lower_case(ex_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr);
                                    }
                                    ConversionType::AllUppers => {
                                        upper_case(ex_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr);
                                    }
                                    ConversionType::BadConversion => {}
                                }
                            }
                            ex_buf_ptr -= 1i32
                        }
                    }
                }
                /*backslash */
                /*ok_pascal_i_give_up */
                prev_colon = false
            } else if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 125i32 {
                /*right_brace */
                decr_brace_level(pop_lit2);
                prev_colon = false
            } else if brace_level == 0i32 {
                /*377: */
                match conversion_type {
                    ConversionType::TitleLowers => {
                        if ex_buf_ptr != 0i32
                            && !(prev_colon
                                && lex_class[*ex_buf.offset((ex_buf_ptr - 1i32) as isize) as usize]
                                    == LexType::WhiteSpace)
                        {
                            lower_case(ex_buf, ex_buf_ptr, 1i32);
                        }
                        if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 58i32 {
                            /*colon */
                            prev_colon = true
                        } else if lex_class[*ex_buf.offset(ex_buf_ptr as isize) as usize]
                            != LexType::WhiteSpace
                        {
                            /*white_space */
                            prev_colon = false
                        }
                    }
                    ConversionType::AllLowers => lower_case(ex_buf, ex_buf_ptr, 1i32),
                    ConversionType::AllUppers => upper_case(ex_buf, ex_buf_ptr, 1i32),
                    ConversionType::BadConversion => {}
                }
            }
            ex_buf_ptr += 1i32
        }
        check_brace_level(pop_lit2);
        add_pool_buf_and_push();
    };
}
unsafe fn x_chr_to_int() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 != StkType::Str {
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Str);
        push_lit_stk(0, StkType::Int);
    } else if get_string_from_pool(pop_lit1).len() != 1 {
        putc_log('\"' as i32);
        print_a_pool_str(pop_lit1);
        log!("\" isn\'t a single character");
        bst_ex_warn_print();
        push_lit_stk(0, StkType::Int);
    } else {
        push_lit_stk(
            *str_pool.offset(*str_start.offset(pop_lit1 as isize) as isize) as i32,
            StkType::Int,
        );
    };
}
unsafe fn x_cite() {
    if !mess_with_entries {
        bst_cant_mess_with_entries_print();
    } else {
        push_lit_stk(*cite_list.offset(cite_ptr as isize), StkType::Str);
    };
}
unsafe fn x_duplicate() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 != StkType::Str {
        push_lit_stk(pop_lit1, pop_typ1);
        push_lit_stk(pop_lit1, pop_typ1);
    } else {
        if *lit_stack.offset(lit_stk_ptr as isize) >= cmd_str_ptr {
            str_ptr += 1;
            pool_ptr = *str_start.offset(str_ptr as isize)
        }
        lit_stk_ptr += 1;
        if pop_lit1 < cmd_str_ptr {
            push_lit_stk(pop_lit1, pop_typ1);
        } else {
            let s = get_string_from_pool(pop_lit1);
            while pool_ptr + s.len() as i32 > pool_size {
                pool_overflow();
            }
            sp_ptr = *str_start.offset(pop_lit1 as isize);
            sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
            while sp_ptr < sp_end {
                *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
                pool_ptr += 1;
                sp_ptr += 1;
            }
            push_lit_stk(make_string(), StkType::Str);
        }
    };
}
unsafe fn x_empty() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    match pop_typ1 {
        StkType::Str => {
            sp_ptr = *str_start.offset(pop_lit1 as isize);
            sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
            while sp_ptr < sp_end {
                if lex_class[*str_pool.offset(sp_ptr as isize) as usize] != LexType::WhiteSpace {
                    /*white_space */
                    push_lit_stk(0i32, StkType::Int);
                    return;
                }
                sp_ptr += 1i32
            }
            push_lit_stk(1i32, StkType::Int);
        }
        StkType::FieldMissing => push_lit_stk(1i32, StkType::Int),
        StkType::Empty => push_lit_stk(0i32, StkType::Int),
        _ => {
            print_stk_lit(pop_lit1, pop_typ1);
            log!(", not a string or missing field,");
            bst_ex_warn_print();
            push_lit_stk(0i32, StkType::Int);
        }
    };
}
unsafe fn x_format_name() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
    if pop_typ1 != StkType::Str {
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Str);
        push_lit_stk(s_null, StkType::Str);
    } else if pop_typ2 != StkType::Int {
        print_wrong_stk_lit(pop_lit2, pop_typ2, StkType::Int);
        push_lit_stk(s_null, StkType::Str);
    } else if pop_typ3 != StkType::Str {
        print_wrong_stk_lit(pop_lit3, pop_typ3, StkType::Str);
        push_lit_stk(s_null, StkType::Str);
    } else {
        ex_buf_length = 0i32;
        add_buf_pool(pop_lit3);
        ex_buf_ptr = 0i32;
        num_names = 0i32;
        while num_names < pop_lit2 && ex_buf_ptr < ex_buf_length {
            num_names += 1i32;
            ex_buf_xptr = ex_buf_ptr;
            name_scan_for_and(pop_lit3);
        }
        if ex_buf_ptr < ex_buf_length {
            ex_buf_ptr -= 4i32
        }
        if num_names < pop_lit2 {
            if pop_lit2 == 1i32 {
                log!("There is no name in \"");
            } else {
                log!("There aren't {} names in \"", pop_lit2);
            }
            print_a_pool_str(pop_lit3);
            putc_log('\"' as i32);
            bst_ex_warn_print();
        }
        while ex_buf_ptr > ex_buf_xptr {
            match lex_class[*ex_buf.offset((ex_buf_ptr - 1i32) as isize) as usize] {
                LexType::WhiteSpace | LexType::SepChar => ex_buf_ptr -= 1i32,
                _ => {
                    if *ex_buf.offset((ex_buf_ptr - 1i32) as isize) as i32 != 44i32 {
                        break;
                    }
                    /*comma */
                    log!("Name {} in \"", pop_lit2);
                    print_a_pool_str(pop_lit3);
                    log!("\" has a comma at the end");
                    bst_ex_warn_print();
                    ex_buf_ptr -= 1i32
                }
            }
        }
        name_bf_ptr = 0i32;
        let mut num_commas = 0i32;
        let mut num_tokens = 0i32;
        let mut token_starting = true;
        let mut comma1 = 0;
        let mut comma2 = 0;
        while ex_buf_xptr < ex_buf_ptr {
            match *ex_buf.offset(ex_buf_xptr as isize) as i32 {
                44 => {
                    if num_commas == 2i32 {
                        log!("Too many commas in name {} of \"", pop_lit2,);
                        print_a_pool_str(pop_lit3);
                        putc_log('\"' as i32);
                        bst_ex_warn_print();
                    } else {
                        num_commas += 1i32;
                        if num_commas == 1i32 {
                            comma1 = num_tokens
                        } else {
                            comma2 = num_tokens
                        }
                        *name_sep_char.offset(num_tokens as isize) = 44i32 as u8
                        /*comma */
                    }
                    ex_buf_xptr += 1i32;
                    token_starting = true
                }
                123 => {
                    brace_level += 1i32;
                    if token_starting {
                        *name_tok.offset(num_tokens as isize) = name_bf_ptr;
                        num_tokens += 1i32
                    }
                    *sv_buffer.offset(name_bf_ptr as isize) = *ex_buf.offset(ex_buf_xptr as isize);
                    name_bf_ptr += 1i32;
                    ex_buf_xptr += 1i32;
                    while brace_level > 0i32 && ex_buf_xptr < ex_buf_ptr {
                        if *ex_buf.offset(ex_buf_xptr as isize) as i32 == 125i32 {
                            /*right_brace */
                            brace_level -= 1i32
                        } else if *ex_buf.offset(ex_buf_xptr as isize) as i32 == 123i32 {
                            /*left_brace */
                            brace_level += 1i32
                        } /*space */
                        *sv_buffer.offset(name_bf_ptr as isize) =
                            *ex_buf.offset(ex_buf_xptr as isize);
                        name_bf_ptr += 1i32;
                        ex_buf_xptr += 1i32
                    }
                    token_starting = false
                }
                125 => {
                    if token_starting {
                        *name_tok.offset(num_tokens as isize) = name_bf_ptr;
                        num_tokens += 1i32
                    }
                    log!("Name {} of \"", pop_lit2);
                    print_a_pool_str(pop_lit3);
                    log!("\" isn\'t brace balanced");
                    bst_ex_warn_print();
                    ex_buf_xptr += 1i32;
                    token_starting = false
                }
                _ => match lex_class[*ex_buf.offset(ex_buf_xptr as isize) as usize] {
                    LexType::WhiteSpace => {
                        if !token_starting {
                            *name_sep_char.offset(num_tokens as isize) = 32i32 as u8
                        }
                        ex_buf_xptr += 1i32;
                        token_starting = true
                    }
                    LexType::SepChar => {
                        if !token_starting {
                            *name_sep_char.offset(num_tokens as isize) =
                                *ex_buf.offset(ex_buf_xptr as isize)
                        }
                        ex_buf_xptr += 1i32;
                        token_starting = true
                    }
                    _ => {
                        if token_starting {
                            *name_tok.offset(num_tokens as isize) = name_bf_ptr;
                            num_tokens += 1i32
                        }
                        *sv_buffer.offset(name_bf_ptr as isize) =
                            *ex_buf.offset(ex_buf_xptr as isize);
                        name_bf_ptr += 1i32;
                        ex_buf_xptr += 1i32;
                        token_starting = false
                    }
                },
            }
        }
        *name_tok.offset(num_tokens as isize) = name_bf_ptr;
        if num_commas == 0i32 {
            first_start = 0i32;
            last_end = num_tokens;
            jr_end = last_end;
            let mut current_block_127: u64;
            von_start = 0i32;
            loop {
                if von_start >= last_end - 1i32 {
                    current_block_127 = 248_631_179_418_912_492;
                    break;
                }
                name_bf_ptr = *name_tok.offset(von_start as isize);
                name_bf_xptr = *name_tok.offset((von_start + 1i32) as isize);
                if von_token_found() {
                    von_name_ends_and_last_name_starts_stuff();
                    current_block_127 = 7_590_078_969_446_600_227;
                    break;
                } else {
                    von_start += 1i32
                }
            }
            loop {
                match current_block_127 {
                    7_590_078_969_446_600_227 => {
                        /*von_found */
                        first_end = von_start;
                        break;
                    }
                    _ => {
                        if von_start > 0i32
                            && !(lex_class[*name_sep_char.offset(von_start as isize) as usize]
                                != LexType::SepChar
                                || *name_sep_char.offset(von_start as isize) as i32 == 126i32)
                        {
                            von_start -= 1i32;
                            current_block_127 = 248_631_179_418_912_492;
                            continue;
                        }
                        /*loop2_exit */
                        von_end = von_start;
                        current_block_127 = 7_590_078_969_446_600_227;
                    }
                }
            }
        } else if num_commas == 1i32 {
            von_start = 0i32;
            last_end = comma1;
            jr_end = last_end;
            first_start = jr_end;
            first_end = num_tokens;
            von_name_ends_and_last_name_starts_stuff();
        } else if num_commas == 2i32 {
            von_start = 0i32;
            last_end = comma1;
            jr_end = comma2;
            first_start = jr_end;
            first_end = num_tokens;
            von_name_ends_and_last_name_starts_stuff();
        } else {
            log!("Illegal number of comma,s");
            print_confusion();
            panic!();
        }
        ex_buf_length = 0i32;
        add_buf_pool(pop_lit1);
        figure_out_the_formatted_name();
        add_pool_buf_and_push();
    };
}
unsafe fn x_int_to_chr() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Int);
        push_lit_stk(s_null, StkType::Str);
    } else if pop_lit1 < 0i32 || pop_lit1 > 127i32 {
        log!("{} isn't valid ASCII", pop_lit1);
        bst_ex_warn_print();
        push_lit_stk(s_null, StkType::Str);
    } else {
        while pool_ptr + 1i32 > pool_size {
            pool_overflow();
        }
        *str_pool.offset(pool_ptr as isize) = pop_lit1 as u8;
        pool_ptr += 1i32;
        push_lit_stk(make_string(), StkType::Str);
    };
}

unsafe fn x_int_to_str() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 != StkType::Int {
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Int);
        push_lit_stk(s_null, StkType::Str);
    } else {
        int_to_ASCII(pop_lit1, ex_buf, 0i32, &mut ex_buf_length);
        add_pool_buf_and_push();
    };
}
unsafe fn x_missing() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if !mess_with_entries {
        bst_cant_mess_with_entries_print();
    } else if pop_typ1 != StkType::Str && pop_typ1 != StkType::FieldMissing {
        if pop_typ1 != StkType::Empty {
            print_stk_lit(pop_lit1, pop_typ1);
            log!(", not a string or missing field,");
            bst_ex_warn_print();
        }
        push_lit_stk(0i32, StkType::Int);
    } else if pop_typ1 == StkType::FieldMissing {
        push_lit_stk(1i32, StkType::Int);
    } else {
        push_lit_stk(0i32, StkType::Int);
    };
}
unsafe fn x_num_names() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Str);
        push_lit_stk(0i32, StkType::Int);
    } else {
        ex_buf_length = 0i32;
        add_buf_pool(pop_lit1);
        ex_buf_ptr = 0i32;
        num_names = 0i32;
        while ex_buf_ptr < ex_buf_length {
            name_scan_for_and(pop_lit1);
            num_names += 1i32
        }
        push_lit_stk(num_names, StkType::Int);
    };
}
unsafe fn x_preamble() {
    ex_buf_length = 0i32;
    preamble_ptr = 0;
    while preamble_ptr < num_preamble_strings {
        add_buf_pool(*s_preamble.add(preamble_ptr));
        preamble_ptr += 1;
    }
    add_pool_buf_and_push();
}
unsafe fn x_purify() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 != StkType::Str {
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Str); /*space */
        push_lit_stk(s_null, StkType::Str);
    } else {
        ex_buf_length = 0i32;
        add_buf_pool(pop_lit1);
        brace_level = 0i32;
        ex_buf_xptr = 0i32;
        ex_buf_ptr = 0i32;
        while ex_buf_ptr < ex_buf_length {
            match lex_class[*ex_buf.offset(ex_buf_ptr as isize) as usize] {
                LexType::WhiteSpace | LexType::SepChar => {
                    *ex_buf.offset(ex_buf_xptr as isize) = 32i32 as u8;
                    ex_buf_xptr += 1i32
                }
                LexType::Alpha | LexType::Numeric => {
                    *ex_buf.offset(ex_buf_xptr as isize) = *ex_buf.offset(ex_buf_ptr as isize);
                    ex_buf_xptr += 1i32
                }
                _ => {
                    if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 123i32 {
                        /*left_brace */
                        brace_level += 1i32;
                        if brace_level == 1i32
                            && ex_buf_ptr + 1i32 < ex_buf_length
                            && *ex_buf.offset((ex_buf_ptr + 1i32) as isize) as i32 == 92i32
                        {
                            /*backslash */
                            /*433: */
                            ex_buf_ptr += 1i32;
                            while ex_buf_ptr < ex_buf_length && brace_level > 0i32 {
                                ex_buf_ptr += 1i32;
                                ex_buf_yptr = ex_buf_ptr;
                                while ex_buf_ptr < ex_buf_length
                                    && lex_class[*ex_buf.offset(ex_buf_ptr as isize) as usize]
                                        == LexType::Alpha
                                {
                                    ex_buf_ptr += 1i32
                                }
                                control_seq_loc = str_lookup(
                                    ex_buf,
                                    ex_buf_yptr,
                                    ex_buf_ptr - ex_buf_yptr,
                                    14i32 as str_ilk,
                                    false,
                                );
                                if hash_found {
                                    /*434: */
                                    *ex_buf.offset(ex_buf_xptr as isize) =
                                        *ex_buf.offset(ex_buf_yptr as isize);
                                    ex_buf_xptr += 1i32;
                                    match *ilk_info.offset(control_seq_loc as isize) {
                                        2 | 3 | 4 | 5 | 12 => {
                                            *ex_buf.offset(ex_buf_xptr as isize) =
                                                *ex_buf.offset((ex_buf_yptr + 1i32) as isize);
                                            ex_buf_xptr += 1i32
                                        }
                                        _ => {}
                                    }
                                }
                                while ex_buf_ptr < ex_buf_length
                                    && brace_level > 0i32
                                    && *ex_buf.offset(ex_buf_ptr as isize) as i32 != 92i32
                                {
                                    match lex_class[*ex_buf.offset(ex_buf_ptr as isize) as usize] {
                                        LexType::Alpha | LexType::Numeric => {
                                            *ex_buf.offset(ex_buf_xptr as isize) =
                                                *ex_buf.offset(ex_buf_ptr as isize);
                                            ex_buf_xptr += 1i32
                                        }
                                        _ => {
                                            if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 125i32
                                            {
                                                /*right_brace */
                                                brace_level -= 1i32
                                            } else if *ex_buf.offset(ex_buf_ptr as isize) as i32
                                                == 123i32
                                            {
                                                /*left_brace */
                                                brace_level += 1i32
                                            }
                                        }
                                    }
                                    ex_buf_ptr += 1i32
                                }
                            }
                            ex_buf_ptr -= 1i32
                        }
                    } else if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 125i32 {
                        /*right_brace */
                        if brace_level > 0i32 {
                            brace_level -= 1i32
                        }
                    }
                }
            } /*double_quote */
            ex_buf_ptr += 1i32
        }
        ex_buf_length = ex_buf_xptr;
        add_pool_buf_and_push();
    };
}
unsafe fn x_quote() {
    while pool_ptr + 1i32 > pool_size {
        pool_overflow();
    }
    *str_pool.offset(pool_ptr as isize) = 34i32 as u8;
    pool_ptr += 1i32;
    push_lit_stk(make_string(), StkType::Str);
}
unsafe fn x_substring() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
    if pop_typ1 != StkType::Int {
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Int);
        push_lit_stk(s_null, StkType::Str);
    } else if pop_typ2 != StkType::Int {
        print_wrong_stk_lit(pop_lit2, pop_typ2, StkType::Int);
        push_lit_stk(s_null, StkType::Str);
    } else if pop_typ3 != StkType::Str {
        print_wrong_stk_lit(pop_lit3, pop_typ3, StkType::Str); /*439: */
        push_lit_stk(s_null, StkType::Str); /*441: */
    } else {
        sp_length =
            *str_start.offset((pop_lit3 + 1i32) as isize) - *str_start.offset(pop_lit3 as isize);
        if pop_lit1 >= sp_length && (pop_lit2 == 1i32 || pop_lit2 == -1i32) {
            if *lit_stack.offset(lit_stk_ptr as isize) >= cmd_str_ptr {
                str_ptr += 1i32;
                pool_ptr = *str_start.offset(str_ptr as isize)
            }
            lit_stk_ptr += 1i32;
            return;
        }
        if pop_lit1 <= 0i32 || pop_lit2 == 0i32 || pop_lit2 > sp_length || pop_lit2 < -sp_length {
            push_lit_stk(s_null, StkType::Str);
        } else {
            if pop_lit2 > 0i32 {
                if pop_lit1 > sp_length - (pop_lit2 - 1i32) {
                    pop_lit1 = sp_length - (pop_lit2 - 1i32)
                }
                sp_ptr = *str_start.offset(pop_lit3 as isize) + (pop_lit2 - 1i32);
                sp_end = sp_ptr + pop_lit1;
                if pop_lit2 == 1i32 && pop_lit3 >= cmd_str_ptr {
                    *str_start.offset((pop_lit3 + 1i32) as isize) = sp_end;
                    str_ptr += 1i32;
                    pool_ptr = *str_start.offset(str_ptr as isize);
                    lit_stk_ptr += 1i32;
                    return;
                }
            } else {
                pop_lit2 = -pop_lit2;
                if pop_lit1 > sp_length - (pop_lit2 - 1i32) {
                    pop_lit1 = sp_length - (pop_lit2 - 1i32)
                }
                sp_end = *str_start.offset((pop_lit3 + 1i32) as isize) - (pop_lit2 - 1i32);
                sp_ptr = sp_end - pop_lit1
            }
            while pool_ptr + sp_end - sp_ptr > pool_size {
                pool_overflow();
            }
            while sp_ptr < sp_end {
                *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
                pool_ptr += 1i32;
                sp_ptr += 1i32
            }
            push_lit_stk(make_string(), StkType::Str);
        }
    };
}
unsafe fn x_swap() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 1i32 || pop_lit1 < cmd_str_ptr {
        push_lit_stk(pop_lit1, pop_typ1);
        if pop_typ2 as i32 == 1i32 && pop_lit2 >= cmd_str_ptr {
            str_ptr += 1i32;
            pool_ptr = *str_start.offset(str_ptr as isize)
        }
        push_lit_stk(pop_lit2, pop_typ2);
    } else if pop_typ2 as i32 != 1i32 || pop_lit2 < cmd_str_ptr {
        str_ptr += 1i32;
        pool_ptr = *str_start.offset(str_ptr as isize);
        push_lit_stk(pop_lit1, StkType::Str);
        push_lit_stk(pop_lit2, pop_typ2);
    } else {
        ex_buf_length = 0i32;
        add_buf_pool(pop_lit2);
        sp_ptr = *str_start.offset(pop_lit1 as isize);
        sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
        while sp_ptr < sp_end {
            *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
            pool_ptr += 1i32;
            sp_ptr += 1i32
        }
        push_lit_stk(make_string(), StkType::Str);
        add_pool_buf_and_push();
    };
}
unsafe fn x_text_length() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 != StkType::Str {
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Str);
        push_lit_stk(s_null, StkType::Str);
    } else {
        let mut num_text_chars = 0i32;
        sp_ptr = *str_start.offset(pop_lit1 as isize);
        sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
        sp_brace_level = 0i32;
        while sp_ptr < sp_end {
            sp_ptr += 1i32;
            if *str_pool.offset((sp_ptr - 1i32) as isize) as i32 == 123i32 {
                /*left_brace */
                sp_brace_level += 1i32;
                if sp_brace_level == 1i32
                    && sp_ptr < sp_end
                    && *str_pool.offset(sp_ptr as isize) as i32 == 92i32
                {
                    /*backslash */
                    sp_ptr += 1i32;
                    while sp_ptr < sp_end && sp_brace_level > 0i32 {
                        if *str_pool.offset(sp_ptr as isize) as i32 == 125i32 {
                            /*right_brace */
                            sp_brace_level -= 1i32
                        } else if *str_pool.offset(sp_ptr as isize) as i32 == 123i32 {
                            /*left_brace */
                            sp_brace_level += 1i32
                        }
                        sp_ptr += 1i32
                    }
                    num_text_chars += 1i32
                }
            } else if *str_pool.offset((sp_ptr - 1i32) as isize) as i32 == 125i32 {
                /*right_brace */
                if sp_brace_level > 0i32 {
                    sp_brace_level -= 1i32
                }
            } else {
                num_text_chars += 1i32
            }
        }
        push_lit_stk(num_text_chars, StkType::Int);
    };
}
unsafe fn x_text_prefix() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 != StkType::Int {
        /*stk_int */
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Int);
        push_lit_stk(s_null, StkType::Str);
    } else if pop_typ2 != StkType::Str {
        /*stk_str */
        print_wrong_stk_lit(pop_lit2, pop_typ2, StkType::Str); /*445: */
        push_lit_stk(s_null, StkType::Str);
    } else if pop_lit1 <= 0i32 {
        push_lit_stk(s_null, StkType::Str);
    } else {
        sp_ptr = *str_start.offset(pop_lit2 as isize);
        sp_end = *str_start.offset((pop_lit2 + 1i32) as isize);
        let mut num_text_chars = 0i32;
        sp_brace_level = 0i32;
        sp_xptr1 = sp_ptr;
        while sp_xptr1 < sp_end && num_text_chars < pop_lit1 {
            sp_xptr1 += 1i32;
            if *str_pool.offset((sp_xptr1 - 1i32) as isize) as i32 == 123i32 {
                /*left_brace */
                sp_brace_level += 1i32;
                if sp_brace_level == 1i32
                    && sp_xptr1 < sp_end
                    && *str_pool.offset(sp_xptr1 as isize) as i32 == 92i32
                {
                    /*backslash */
                    sp_xptr1 += 1i32;
                    while sp_xptr1 < sp_end && sp_brace_level > 0i32 {
                        if *str_pool.offset(sp_xptr1 as isize) as i32 == 125i32 {
                            /*right_brace */
                            sp_brace_level -= 1i32
                        } else if *str_pool.offset(sp_xptr1 as isize) as i32 == 123i32 {
                            /*left_brace */
                            sp_brace_level += 1i32
                        }
                        sp_xptr1 += 1i32
                    }
                    num_text_chars += 1i32
                }
            } else if *str_pool.offset((sp_xptr1 - 1i32) as isize) as i32 == 125i32 {
                /*right_brace */
                if sp_brace_level > 0i32 {
                    sp_brace_level -= 1i32
                }
            } else {
                num_text_chars += 1i32
            }
        } /*right_brace */
        sp_end = sp_xptr1;
        while pool_ptr + sp_brace_level + sp_end - sp_ptr > pool_size {
            pool_overflow();
        }
        if pop_lit2 >= cmd_str_ptr {
            pool_ptr = sp_end
        } else {
            while sp_ptr < sp_end {
                *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
                pool_ptr += 1i32;
                sp_ptr += 1i32
            }
        }
        while sp_brace_level > 0i32 {
            *str_pool.offset(pool_ptr as isize) = 125i32 as u8;
            pool_ptr += 1i32;
            sp_brace_level -= 1i32
        }
        push_lit_stk(make_string(), StkType::Str);
    };
}
unsafe fn x_type() {
    if !mess_with_entries {
        bst_cant_mess_with_entries_print();
    } else if *type_list.offset(cite_ptr as isize) == undefined
        || *type_list.offset(cite_ptr as isize) == 0i32
    {
        push_lit_stk(s_null, StkType::Str);
    } else {
        push_lit_stk(
            *hash_text.offset(*type_list.offset(cite_ptr as isize) as isize),
            StkType::Str,
        );
    };
}
unsafe fn x_warning() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Str);
    } else {
        log!("Warning--");
        print_lit(pop_lit1, pop_typ1);
        mark_warning();
    };
}
unsafe fn x_width() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 != StkType::Str {
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Str);
        push_lit_stk(0i32, StkType::Int);
    } else {
        ex_buf_length = 0i32;
        add_buf_pool(pop_lit1);
        string_width = 0i32;
        brace_level = 0i32;
        ex_buf_ptr = 0i32;
        while ex_buf_ptr < ex_buf_length {
            if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 123i32 {
                /*left_brace */
                brace_level += 1i32;
                if brace_level == 1i32 && ex_buf_ptr + 1i32 < ex_buf_length {
                    if *ex_buf.offset((ex_buf_ptr + 1i32) as isize) as i32 == 92i32 {
                        /*backslash */
                        /*453: */
                        ex_buf_ptr += 1i32;
                        while ex_buf_ptr < ex_buf_length && brace_level > 0i32 {
                            ex_buf_ptr += 1i32;
                            ex_buf_xptr = ex_buf_ptr;
                            while ex_buf_ptr < ex_buf_length
                                && lex_class[*ex_buf.offset(ex_buf_ptr as isize) as usize]
                                    == LexType::Alpha
                            {
                                ex_buf_ptr += 1i32
                            }
                            if ex_buf_ptr < ex_buf_length && ex_buf_ptr == ex_buf_xptr {
                                ex_buf_ptr += 1i32
                            } else {
                                control_seq_loc = str_lookup(
                                    ex_buf,
                                    ex_buf_xptr,
                                    ex_buf_ptr - ex_buf_xptr,
                                    14i32 as str_ilk,
                                    false,
                                );
                                if hash_found {
                                    /*454: */
                                    match *ilk_info.offset(control_seq_loc as isize) {
                                        12 => string_width += 500i32,
                                        4 => string_width += 722i32,
                                        2 => string_width += 778i32,
                                        5 => string_width += 903i32,
                                        3 => string_width += 1014i32,
                                        _ => {
                                            string_width += char_width
                                                [*ex_buf.offset(ex_buf_xptr as isize) as usize]
                                        }
                                    }
                                }
                            }
                            while ex_buf_ptr < ex_buf_length
                                && lex_class[*ex_buf.offset(ex_buf_ptr as isize) as usize]
                                    == LexType::WhiteSpace
                            {
                                ex_buf_ptr += 1i32
                            }
                            while ex_buf_ptr < ex_buf_length
                                && brace_level > 0i32
                                && *ex_buf.offset(ex_buf_ptr as isize) as i32 != 92i32
                            {
                                if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 125i32 {
                                    /*right_brace */
                                    brace_level -= 1i32
                                } else if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 123i32 {
                                    /*left_brace */
                                    brace_level += 1i32
                                } else {
                                    string_width +=
                                        char_width[*ex_buf.offset(ex_buf_ptr as isize) as usize]
                                }
                                ex_buf_ptr += 1i32
                            }
                        }
                        ex_buf_ptr -= 1i32
                    } else {
                        string_width += char_width[123]
                    }
                } else {
                    string_width += char_width[123]
                }
            } else if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 125i32 {
                /*right_brace */
                decr_brace_level(pop_lit1);
                string_width += char_width[125]
            } else {
                string_width += char_width[*ex_buf.offset(ex_buf_ptr as isize) as usize]
            }
            ex_buf_ptr += 1i32
        }
        check_brace_level(pop_lit1);
        push_lit_stk(string_width, StkType::Int);
    };
}
unsafe fn x_write() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 != StkType::Str {
        print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Str);
    } else {
        add_out_pool(pop_lit1);
    };
}
unsafe fn execute_fn(mut ex_fn_loc: hash_loc) {
    let mut r_pop_lt1: i32 = 0;
    let mut r_pop_lt2: i32 = 0;
    let mut r_pop_tp1: StkType = StkType::Int;
    let mut r_pop_tp2: StkType = StkType::Int;
    let mut wiz_ptr: wiz_fn_loc = 0;
    match *fn_type.offset(ex_fn_loc as isize) as i32 {
        0 => match *ilk_info.offset(ex_fn_loc as isize) {
            0 => x_equals(),
            1 => x_greater_than(),
            2 => x_less_than(),
            3 => x_plus(),
            4 => x_minus(),
            5 => x_concatenate(),
            6 => x_gets(),
            7 => x_add_period(),
            8 => {
                if !mess_with_entries {
                    bst_cant_mess_with_entries_print();
                } else if *type_list.offset(cite_ptr as isize) == undefined {
                    execute_fn(b_default);
                } else if *type_list.offset(cite_ptr as isize) != 0i32 {
                    execute_fn(*type_list.offset(cite_ptr as isize));
                }
            }
            9 => x_change_case(),
            10 => x_chr_to_int(),
            11 => x_cite(),
            12 => x_duplicate(),
            13 => x_empty(),
            14 => x_format_name(),
            15 => {
                pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                if pop_typ1 as i32 != 2i32 {
                    print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Fn);
                } else if pop_typ2 as i32 != 2i32 {
                    print_wrong_stk_lit(pop_lit2, pop_typ2, StkType::Fn);
                } else if pop_typ3 as i32 != 0i32 {
                    print_wrong_stk_lit(pop_lit3, pop_typ3, StkType::Int);
                } else if pop_lit3 > 0i32 {
                    execute_fn(pop_lit2);
                } else {
                    execute_fn(pop_lit1);
                }
            }
            16 => x_int_to_chr(),
            17 => x_int_to_str(),
            18 => x_missing(),
            19 => output_bbl_line(),
            20 => x_num_names(),
            21 => pop_lit_stk(&mut pop_lit1, &mut pop_typ1),
            22 => x_preamble(),
            23 => x_purify(),
            24 => x_quote(),
            25 => {}
            26 => pop_whole_stack(),
            27 => x_substring(),
            28 => x_swap(),
            29 => x_text_length(),
            30 => x_text_prefix(),
            31 => pop_top_and_print(),
            32 => x_type(),
            33 => x_warning(),
            34 => {
                pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                if r_pop_tp1 as i32 != 2i32 {
                    print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, StkType::Fn);
                } else if r_pop_tp2 as i32 != 2i32 {
                    print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, StkType::Fn);
                } else {
                    loop {
                        execute_fn(r_pop_lt2);
                        pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        if pop_typ1 as i32 != 0i32 {
                            print_wrong_stk_lit(pop_lit1, pop_typ1, StkType::Int);
                            break;
                        } else {
                            if pop_lit1 <= 0i32 {
                                break;
                            }
                            execute_fn(r_pop_lt1);
                        }
                    }
                }
            }
            35 => x_width(),
            36 => x_write(),
            _ => {
                log!("Unknown built-in function");
                print_confusion();
                panic!();
            }
        },
        1 => {
            wiz_ptr = *ilk_info.offset(ex_fn_loc as isize);
            while *wiz_functions.offset(wiz_ptr as isize) != end_of_def {
                if *wiz_functions.offset(wiz_ptr as isize) != 1i32 - 1i32 {
                    execute_fn(*wiz_functions.offset(wiz_ptr as isize));
                } else {
                    wiz_ptr += 1i32;
                    push_lit_stk(*wiz_functions.offset(wiz_ptr as isize), StkType::Fn);
                }
                wiz_ptr += 1i32
            }
        }
        2 => push_lit_stk(*ilk_info.offset(ex_fn_loc as isize), StkType::Int),
        3 => push_lit_stk(*hash_text.offset(ex_fn_loc as isize), StkType::Str),
        4 => {
            if !mess_with_entries {
                bst_cant_mess_with_entries_print();
            } else {
                field_ptr = cite_ptr * num_fields + *ilk_info.offset(ex_fn_loc as isize);
                if field_ptr >= max_fields {
                    log!("field_info index is out of range");
                    print_confusion();
                    panic!();
                }
                if *field_info.offset(field_ptr as isize) == 0i32 {
                    /*missing */
                    push_lit_stk(*hash_text.offset(ex_fn_loc as isize), StkType::FieldMissing);
                } else {
                    push_lit_stk(*field_info.offset(field_ptr as isize), StkType::Str);
                }
            }
        }
        5 => {
            if !mess_with_entries {
                bst_cant_mess_with_entries_print();
            } else {
                push_lit_stk(
                    *entry_ints.offset(
                        (cite_ptr * num_ent_ints + *ilk_info.offset(ex_fn_loc as isize)) as isize,
                    ),
                    StkType::Int,
                );
            }
        }
        6 => {
            if !mess_with_entries {
                bst_cant_mess_with_entries_print();
            } else {
                str_ent_ptr = cite_ptr * num_ent_strs + *ilk_info.offset(ex_fn_loc as isize);
                ex_buf_ptr = 0i32;
                while *entry_strs
                    .offset((str_ent_ptr * (ent_str_size + 1i32) + ex_buf_ptr) as isize)
                    as i32
                    != 127i32
                {
                    /*end_of_string */
                    *ex_buf.offset(ex_buf_ptr as isize) = *entry_strs
                        .offset((str_ent_ptr * (ent_str_size + 1i32) + ex_buf_ptr) as isize); /* strip off the (assumed) ".aux" for subsequent futzing */
                    ex_buf_ptr += 1i32
                }
                ex_buf_length = ex_buf_ptr;
                add_pool_buf_and_push();
            }
        }
        7 => push_lit_stk(*ilk_info.offset(ex_fn_loc as isize), StkType::Int),
        8 => {
            str_glb_ptr = *ilk_info.offset(ex_fn_loc as isize);
            if *glb_str_ptr.offset(str_glb_ptr as isize) > 0i32 {
                push_lit_stk(*glb_str_ptr.offset(str_glb_ptr as isize), StkType::Str);
            } else {
                while pool_ptr + *glb_str_end.offset(str_glb_ptr as isize) > pool_size {
                    pool_overflow();
                }
                glob_chr_ptr = 0i32;
                while glob_chr_ptr < *glb_str_end.offset(str_glb_ptr as isize) {
                    *str_pool.offset(pool_ptr as isize) = *global_strs
                        .offset((str_glb_ptr * (glob_str_size + 1i32) + glob_chr_ptr) as isize);
                    pool_ptr += 1i32;
                    glob_chr_ptr += 1i32
                }
                push_lit_stk(make_string(), StkType::Str);
            }
        }
        _ => unknwn_function_class_confusion(),
    };
}
unsafe fn get_the_top_level_aux_file_name(mut aux_file_name: *const i8) -> i32 {
    name_of_file = xmalloc_array(strlen(aux_file_name) + 1);
    strcpy(name_of_file as *mut i8, aux_file_name);
    aux_name_length = strlen(name_of_file as *mut i8) as i32;
    aux_name_length -= 4i32;
    name_length = aux_name_length;
    /* this code used to auto-add the .aux extension if needed; we don't */
    aux_ptr = 0i32; // preserve pascal-style string semantics
    aux_file[aux_ptr as usize] = peekable_open(name_of_file as *mut i8, TTInputFormat::TEX);
    if aux_file[aux_ptr as usize].is_none() {
        sam_wrong_file_name_print();
        return 1i32;
    }
    add_extension(s_log_extension);
    log_file = ttstub_output_open(name_of_file as *mut i8, 0i32);
    if log_file.is_none() {
        sam_wrong_file_name_print();
        return 1i32;
    }
    name_length = aux_name_length;
    add_extension(s_bbl_extension);
    bbl_file = ttstub_output_open(name_of_file as *mut i8, 0i32);
    if bbl_file.is_none() {
        sam_wrong_file_name_print();
        return 1i32;
    }
    name_length = aux_name_length;
    add_extension(s_aux_extension);
    name_ptr = 0i32;
    while name_ptr < name_length {
        *buffer.offset((name_ptr + 1i32) as isize) = *name_of_file.offset(name_ptr as isize);
        name_ptr += 1i32
    }
    top_lev_str = *hash_text
        .offset(str_lookup(buffer, 1i32, aux_name_length, 0i32 as str_ilk, true) as isize);
    aux_list[aux_ptr as usize] =
        *hash_text.offset(str_lookup(buffer, 1i32, name_length, 3i32 as str_ilk, true) as isize);
    if hash_found {
        log!("Already encountered auxiliary file");
        print_confusion();
        panic!();
    }
    aux_ln_stack[aux_ptr as usize] = 0i32;
    0i32
}
unsafe fn aux_bib_data_command() {
    if bib_seen {
        aux_err_illegal_another_print(0i32);
        aux_err_print();
        return;
    }
    bib_seen = true;
    while *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        buf_ptr2 += 1i32;
        if !scan2_white(125i32 as u8, 44i32 as u8) {
            aux_err_no_right_brace_print();
            aux_err_print();
            return;
        }
        if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] == LexType::WhiteSpace {
            /*white_space */
            aux_err_white_space_in_argument_print();
            aux_err_print();
            return;
        }
        if last > buf_ptr2 + 1i32 && *buffer.offset(buf_ptr2 as isize) as i32 == 125i32 {
            aux_err_stuff_after_right_brace_print();
            aux_err_print();
            return;
        }
        if bib_ptr == MAX_BIB_FILES {
            bib_list = xrealloc(
                bib_list as *mut libc::c_void,
                ((MAX_BIB_FILES + 20 + 1) as u64)
                    .wrapping_mul(::std::mem::size_of::<str_number>() as u64) as _,
            ) as *mut str_number;
            /*bib_file = xrealloc(
                bib_file as *mut libc::c_void,
                ((MAX_BIB_FILES + 20 + 1) as u64)
                    .wrapping_mul(::std::mem::size_of::<*mut peekable_input_t>() as u64) as _
            ) as *mut *mut peekable_input_t;*/
            s_preamble = xrealloc(
                s_preamble as *mut libc::c_void,
                ((MAX_BIB_FILES + 20 + 1) as u64)
                    .wrapping_mul(::std::mem::size_of::<str_number>() as u64) as _,
            ) as *mut str_number;
            MAX_BIB_FILES += 20;
        }
        *bib_list.add(bib_ptr) = *hash_text.offset(str_lookup(
            buffer,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            6i32 as str_ilk,
            true,
        ) as isize);
        if hash_found {
            log!("This database file appears more than once: ");
            print_bib_name();
            aux_err_print();
            return;
        }
        start_name(*bib_list.add(bib_ptr));
        bib_file.push(peekable_open(name_of_file as *mut i8, TTInputFormat::BIB));
        if bib_file[bib_ptr].is_none() {
            log!("I couldn\'t open database file ");
            print_bib_name();
            aux_err_print();
            return;
        }
        bib_ptr += 1;
        assert!(bib_file.len() == bib_ptr);
    }
}
unsafe fn aux_bib_style_command() {
    if bst_seen {
        aux_err_illegal_another_print(1i32);
        aux_err_print();
        return;
    }
    bst_seen = true;
    buf_ptr2 += 1i32;
    if !scan1_white(125i32 as u8) {
        aux_err_no_right_brace_print();
        aux_err_print();
        return;
    }
    if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] == LexType::WhiteSpace {
        /*white_space */
        aux_err_white_space_in_argument_print();
        aux_err_print();
        return;
    }
    if last > buf_ptr2 + 1i32 {
        aux_err_stuff_after_right_brace_print();
        aux_err_print();
        return;
    }
    bst_str =
        *hash_text.offset(
            str_lookup(buffer, buf_ptr1, buf_ptr2 - buf_ptr1, 5i32 as str_ilk, true) as isize,
        );
    if hash_found {
        log!("Already encountered style file");
        print_confusion();
        panic!();
    }
    start_name(bst_str);
    bst_file = peekable_open(name_of_file as *mut i8, TTInputFormat::BST);
    if bst_file.is_none() {
        log!("I couldn\'t open style file ");
        print_bst_name();
        bst_str = 0i32;
        aux_err_print();
        return;
    }
    if verbose {
        log!("The style file: ");
        print_bst_name();
    } else {
        write!(log_file.as_mut().unwrap(), "The style file: ").unwrap();
        log_pr_bst_name();
    };
}
unsafe fn aux_citation_command() {
    citation_seen = true;
    'lab23: while *buffer.offset(buf_ptr2 as isize) as i32 != 125
    /*right_brace */
    {
        buf_ptr2 += 1;
        if !scan2_white(125 /*right_brace */, 44 /*comma */) {
            aux_err_no_right_brace_print();
            aux_err_print();
            return;
        }
        if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] == LexType::WhiteSpace {
            /*white_space */
            aux_err_white_space_in_argument_print();
            aux_err_print();
            return;
        }
        if last > buf_ptr2 + 1i32 && *buffer.offset(buf_ptr2 as isize) as i32 == 125i32 {
            aux_err_stuff_after_right_brace_print();
            aux_err_print();
            return;
        }
        if buf_ptr2 - buf_ptr1 == 1i32 && *buffer.offset(buf_ptr1 as isize) as i32 == 42i32 {
            /*star */
            if all_entries {
                log!("Multiple inclusions of entire database\n"); /*137: */
                aux_err_print();
                return;
            } else {
                all_entries = true;
                all_marker = cite_ptr
            }
            continue 'lab23;
        }
        let mut tmp_ptr = buf_ptr1;
        while tmp_ptr < buf_ptr2 {
            *ex_buf.offset(tmp_ptr as isize) = *buffer.offset(tmp_ptr as isize);
            tmp_ptr += 1i32
        }
        lower_case(ex_buf, buf_ptr1, buf_ptr2 - buf_ptr1);
        lc_cite_loc = str_lookup(
            ex_buf,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            10i32 as str_ilk,
            true,
        );
        if hash_found {
            /*136: */
            dummy_loc = str_lookup(
                buffer,
                buf_ptr1,
                buf_ptr2 - buf_ptr1,
                9i32 as str_ilk,
                false,
            );
            if !hash_found {
                log!("Case mismatch error between cite keys ");
                print_a_token();
                log!(" and ");
                print_a_pool_str(*cite_list.offset(
                    *ilk_info.offset(*ilk_info.offset(lc_cite_loc as isize) as isize) as isize,
                ));
                putc_log('\n' as i32);
                aux_err_print();
                return;
            }
        } else {
            /*137: */
            cite_loc = str_lookup(buffer, buf_ptr1, buf_ptr2 - buf_ptr1, 9i32 as str_ilk, true);
            if hash_found {
                hash_cite_confusion();
            }
            check_cite_overflow(cite_ptr);
            *cite_list.offset(cite_ptr as isize) = *hash_text.offset(cite_loc as isize);
            *ilk_info.offset(cite_loc as isize) = cite_ptr;
            *ilk_info.offset(lc_cite_loc as isize) = cite_loc;
            cite_ptr += 1i32
        }
        continue 'lab23; /*next_cite */
    }
}
unsafe fn aux_input_command() {
    let mut aux_extension_ok: bool = false;
    buf_ptr2 += 1i32;
    if !scan1_white(125i32 as u8) {
        aux_err_no_right_brace_print();
        aux_err_print();
        return;
    }
    if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] == LexType::WhiteSpace {
        /*white_space */
        aux_err_white_space_in_argument_print();
        aux_err_print();
        return;
    }
    if last > buf_ptr2 + 1i32 {
        aux_err_stuff_after_right_brace_print();
        aux_err_print();
        return;
    }
    aux_ptr += 1i32;
    if aux_ptr == 20i32 {
        print_a_token();
        log!(": ");
        print_overflow();
        log!("auxiliary file depth {}\n", 20,);
        panic!();
    }
    aux_extension_ok = true;

    let buffer_offset = buf_ptr2
        - (*str_start.offset((s_aux_extension + 1i32) as isize)
            - *str_start.offset(s_aux_extension as isize));
    let len = *str_start.offset((s_aux_extension + 1i32) as isize)
        - *str_start.offset(s_aux_extension as isize);
    let s = slice::from_raw_parts(buffer.offset(buffer_offset as isize), len as usize);

    if buf_ptr2 - buf_ptr1 < len {
        aux_extension_ok = false
    } else if !str_eq_buf(s_aux_extension, s) {
        aux_extension_ok = false
    }
    if !aux_extension_ok {
        print_a_token();
        log!(" has a wrong extension");
        aux_ptr -= 1i32;
        aux_err_print();
        return;
    }
    aux_list[aux_ptr as usize] =
        *hash_text.offset(
            str_lookup(buffer, buf_ptr1, buf_ptr2 - buf_ptr1, 3i32 as str_ilk, true) as isize,
        );
    if hash_found {
        log!("Already encountered file ");
        print_aux_name();
        aux_ptr -= 1i32;
        aux_err_print();
        return;
    }
    start_name(aux_list[aux_ptr as usize]);
    name_ptr = name_length;
    *name_of_file.offset(name_ptr as isize) = 0i32 as u8;
    aux_file[aux_ptr as usize] = peekable_open(name_of_file as *mut i8, TTInputFormat::TEX);
    if aux_file[aux_ptr as usize].is_none() {
        log!("I couldn\'t open auxiliary file ");
        print_aux_name();
        aux_ptr -= 1;
        aux_err_print();
        return;
    }
    log!("A level-{} auxiliary file: ", aux_ptr,);
    print_aux_name();
    aux_ln_stack[aux_ptr as usize] = 0i32;
}
unsafe fn pop_the_aux_stack() -> i32 {
    let _ = aux_file[aux_ptr as usize].take();
    if aux_ptr == 0i32 {
        return 1i32;
    }
    aux_ptr -= 1;
    0i32
}
unsafe fn get_aux_command_and_process() {
    buf_ptr2 = 0i32;
    if !scan1(123i32 as u8) {
        return;
    }
    command_num = *ilk_info.offset(str_lookup(
        buffer,
        buf_ptr1,
        buf_ptr2 - buf_ptr1,
        2i32 as str_ilk,
        false,
    ) as isize);
    if hash_found {
        match command_num {
            0 => aux_bib_data_command(),
            1 => aux_bib_style_command(),
            2 => aux_citation_command(),
            3 => aux_input_command(),
            _ => {
                log!("Unknown auxiliary-file command");
                print_confusion();
                panic!();
            }
        }
    };
}
unsafe fn last_check_for_aux_errors() {
    num_cites = cite_ptr;
    num_bib_files = bib_ptr;
    if !citation_seen {
        aux_end1_err_print();
        log!("\\citation commands");
        aux_end2_err_print();
    } else if num_cites == 0i32 && !all_entries {
        aux_end1_err_print();
        log!("cite keys");
        aux_end2_err_print();
    }
    if !bib_seen {
        aux_end1_err_print();
        log!("\\bibdata command");
        aux_end2_err_print();
    } else if num_bib_files == 0 {
        aux_end1_err_print();
        log!("database files");
        aux_end2_err_print();
    }
    if !bst_seen {
        aux_end1_err_print();
        log!("\\bibstyle command");
        aux_end2_err_print();
    } else if bst_str == 0 {
        aux_end1_err_print();
        log!("style file");
        aux_end2_err_print();
    };
}
unsafe fn bst_entry_command() {
    if entry_seen {
        log!("Illegal, another entry command");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    entry_seen = true;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("entry");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        log!("entry");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("entry");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    while *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        let scan_result = scan_identifier(125i32 as u8, 37i32 as u8, 37i32 as u8); /*field */
        if scan_result == ScanResult::WhiteAdjacent
            || scan_result == ScanResult::SpecifiedCharAdjacent
        {
        } else {
            bst_id_print(scan_result);
            log!("entry");
            bst_err_print_and_look_for_blank_line();
            return;
        }
        lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
        fn_loc = str_lookup(
            buffer,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            11i32 as str_ilk,
            true,
        );
        if hash_found {
            already_seen_function_print(fn_loc);
            return;
        }
        *fn_type.offset(fn_loc as isize) = FnClass::Field;
        *ilk_info.offset(fn_loc as isize) = num_fields;
        num_fields += 1i32;
        if !eat_bst_white_space() {
            eat_bst_print();
            log!("entry");
            bst_err_print_and_look_for_blank_line();
            return;
        }
    }
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("entry");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if num_fields == num_pre_defined_fields {
        log!("Warning--I didn\'t find any fields");
        bst_warn_print();
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        log!("entry");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("entry");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    while *buffer.offset(buf_ptr2 as isize) != b'}' {
        /*right_brace */
        let scan_result = scan_identifier(125i32 as u8, 37i32 as u8, 37i32 as u8); /*int_entry_var */
        if scan_result == ScanResult::WhiteAdjacent
            || scan_result == ScanResult::SpecifiedCharAdjacent
        {
        } else {
            bst_id_print(scan_result);
            log!("entry");
            bst_err_print_and_look_for_blank_line();
            return;
        }
        lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
        fn_loc = str_lookup(
            buffer,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            11i32 as str_ilk,
            true,
        );
        if hash_found {
            already_seen_function_print(fn_loc);
            return;
        }
        *fn_type.offset(fn_loc as isize) = FnClass::IntEntryVar;
        *ilk_info.offset(fn_loc as isize) = num_ent_ints;
        num_ent_ints += 1i32;
        if !eat_bst_white_space() {
            eat_bst_print();
            log!("entry");
            bst_err_print_and_look_for_blank_line();
            return;
        }
    }
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("entry");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        log!("entry");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("entry");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    while *buffer.offset(buf_ptr2 as isize) != b'}' {
        let scan_result = scan_identifier(125i32 as u8, 37i32 as u8, 37i32 as u8); /*str_entry_var */
        if scan_result == ScanResult::WhiteAdjacent
            || scan_result == ScanResult::SpecifiedCharAdjacent
        {
        } else {
            bst_id_print(scan_result);
            log!("entry");
            bst_err_print_and_look_for_blank_line();
            return;
        }
        lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
        fn_loc = str_lookup(
            buffer,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            11i32 as str_ilk,
            true,
        );
        if hash_found {
            already_seen_function_print(fn_loc);
            return;
        }
        *fn_type.offset(fn_loc as isize) = FnClass::StrEntryVar;
        *ilk_info.offset(fn_loc as isize) = num_ent_strs;
        num_ent_strs += 1i32;
        if !eat_bst_white_space() {
            eat_bst_print();
            log!("entry");
            bst_err_print_and_look_for_blank_line();
            return;
        }
    }
    buf_ptr2 += 1i32;
}
unsafe fn bad_argument_token() -> bool {
    lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
    fn_loc = str_lookup(
        buffer,
        buf_ptr1,
        buf_ptr2 - buf_ptr1,
        11i32 as str_ilk,
        false,
    );
    if !hash_found {
        print_a_token();
        log!(" is an unknown function");
        bst_err_print_and_look_for_blank_line();
        return true;
    } else if *fn_type.offset(fn_loc as isize) as i32 != 0i32
        && *fn_type.offset(fn_loc as isize) as i32 != 1i32
    {
        print_a_token();
        log!(" has bad function type ");
        print_fn_class(fn_loc);
        bst_err_print_and_look_for_blank_line();
        return true;
    }
    false
}
unsafe fn bst_execute_command() {
    if !read_seen {
        log!("Illegal, execute command before read command");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("execute");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        log!("execute");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("execute");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    let scan_result = scan_identifier(125i32 as u8, 37i32 as u8, 37i32 as u8);
    if scan_result == ScanResult::WhiteAdjacent || scan_result == ScanResult::SpecifiedCharAdjacent
    {
    } else {
        bst_id_print(scan_result);
        log!("execute");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if bad_argument_token() {
        return;
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("execute");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        bst_right_brace_print();
        log!("execute");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    init_command_execution();
    mess_with_entries = false;
    execute_fn(fn_loc);
    check_command_execution();
}
unsafe fn bst_function_command() {
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("function");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print(); /*wiz_defined */
        log!("function");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("function");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    let scan_result = scan_identifier(125i32 as u8, 37i32 as u8, 37i32 as u8);
    if scan_result == ScanResult::WhiteAdjacent || scan_result == ScanResult::SpecifiedCharAdjacent
    {
    } else {
        bst_id_print(scan_result);
        log!("function");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
    wiz_loc = str_lookup(
        buffer,
        buf_ptr1,
        buf_ptr2 - buf_ptr1,
        11i32 as str_ilk,
        true,
    );
    if hash_found {
        already_seen_function_print(wiz_loc);
        return;
    }
    *fn_type.offset(wiz_loc as isize) = FnClass::WizDefined;
    if *hash_text.offset(wiz_loc as isize) == s_default {
        b_default = wiz_loc
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("function");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        bst_right_brace_print();
        log!("function");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("function");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        log!("function");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    scan_fn_def(wiz_loc);
}
unsafe fn bst_integers_command() {
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("integers");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        log!("integers");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("integers");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    while *buffer.offset(buf_ptr2 as isize) != b'}' {
        let scan_result = scan_identifier(125i32 as u8, 37i32 as u8, 37i32 as u8); /*int_global_var */
        if scan_result == ScanResult::WhiteAdjacent
            || scan_result == ScanResult::SpecifiedCharAdjacent
        {
        } else {
            bst_id_print(scan_result);
            log!("integers");
            bst_err_print_and_look_for_blank_line();
            return;
        }
        lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
        fn_loc = str_lookup(
            buffer,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            11i32 as str_ilk,
            true,
        );
        if hash_found {
            already_seen_function_print(fn_loc);
            return;
        }
        *fn_type.offset(fn_loc as isize) = FnClass::IntGlobalVar;
        *ilk_info.offset(fn_loc as isize) = 0i32;
        if !eat_bst_white_space() {
            eat_bst_print();
            log!("integers");
            bst_err_print_and_look_for_blank_line();
            return;
        }
    }
    buf_ptr2 += 1i32;
}
unsafe fn bst_iterate_command() {
    if !read_seen {
        log!("Illegal, iterate command before read command");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("iterate");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        log!("iterate");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("iterate");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    let scan_result = scan_identifier(125i32 as u8, 37i32 as u8, 37i32 as u8);
    if scan_result == ScanResult::WhiteAdjacent || scan_result == ScanResult::SpecifiedCharAdjacent
    {
    } else {
        bst_id_print(scan_result);
        log!("iterate");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if bad_argument_token() {
        return;
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("iterate");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        bst_right_brace_print();
        log!("iterate");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    init_command_execution();
    mess_with_entries = true;
    sort_cite_ptr = 0i32;
    while sort_cite_ptr < num_cites {
        cite_ptr = *cite_info.offset(sort_cite_ptr as isize);
        execute_fn(fn_loc);
        check_command_execution();
        sort_cite_ptr += 1i32
    }
}
unsafe fn bst_macro_command() {
    if read_seen {
        log!("Illegal, macro command after read command");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("macro");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        log!("macro");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("macro");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    let scan_result = scan_identifier(125i32 as u8, 37i32 as u8, 37i32 as u8);
    if scan_result == ScanResult::WhiteAdjacent || scan_result == ScanResult::SpecifiedCharAdjacent
    {
    } else {
        bst_id_print(scan_result);
        log!("macro");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
    macro_name_loc = str_lookup(
        buffer,
        buf_ptr1,
        buf_ptr2 - buf_ptr1,
        13i32 as str_ilk,
        true,
    );
    if hash_found {
        print_a_token();
        log!(" is already defined as a macro");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    *ilk_info.offset(macro_name_loc as isize) = *hash_text.offset(macro_name_loc as isize);
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("macro");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        bst_right_brace_print();
        log!("macro");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("macro");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        log!("macro");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("macro");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 34i32 {
        /*double_quote */
        log!("A macro definition must be \"-delimited"); /*str_literal */
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    if !scan1(34i32 as u8) {
        log!("There\'s no `\"\' to end macro definition");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    macro_def_loc = str_lookup(buffer, buf_ptr1, buf_ptr2 - buf_ptr1, 0i32 as str_ilk, true);
    *fn_type.offset(macro_def_loc as isize) = FnClass::StrLiteral;
    *ilk_info.offset(macro_name_loc as isize) = *hash_text.offset(macro_def_loc as isize);
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("macro");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        bst_right_brace_print();
        log!("macro");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
}
unsafe fn get_bib_command_or_entry_and_process() {
    let mut current_block: u64;
    at_bib_command = false;
    while !scan1(64i32 as u8) {
        if !input_ln(&mut bib_file[bib_ptr]) {
            return;
        }
        bib_line_num += 1;
        buf_ptr2 = 0i32
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 64i32 {
        /*at_sign */
        log!("An \"@\" disappeared");
        print_confusion();
        panic!();
    }
    buf_ptr2 += 1;
    if !eat_bib_white_space() {
        eat_bib_print();
        return;
    }
    // TODO: Replace this pattern by a function returning Result somehow
    let scan_result = scan_identifier(123i32 as u8, 40i32 as u8, 40i32 as u8);
    if scan_result == ScanResult::WhiteAdjacent || scan_result == ScanResult::SpecifiedCharAdjacent
    {
    } else {
        bib_id_print(scan_result);
        log!("an entry type");
        bib_err_print();
        return;
    }
    lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
    command_num = *ilk_info.offset(str_lookup(
        buffer,
        buf_ptr1,
        buf_ptr2 - buf_ptr1,
        12i32 as str_ilk,
        false,
    ) as isize);
    if hash_found {
        /*240: */
        at_bib_command = true;
        match command_num {
            0 => return,
            1 => {
                if preamble_ptr == MAX_BIB_FILES {
                    bib_list = xrealloc(
                        bib_list as *mut libc::c_void,
                        ((MAX_BIB_FILES + 20 + 1) as u64)
                            .wrapping_mul(::std::mem::size_of::<str_number>() as u64)
                            as _,
                    ) as *mut str_number;
                    /*bib_file = xrealloc(
                        bib_file as *mut libc::c_void,
                        ((MAX_BIB_FILES + 20 + 1) as u64)
                            .wrapping_mul(::std::mem::size_of::<*mut peekable_input_t>() as u64),
                    ) as *mut *mut peekable_input_t;*/
                    s_preamble = xrealloc(
                        s_preamble as *mut libc::c_void,
                        ((MAX_BIB_FILES + 20 + 1) as u64)
                            .wrapping_mul(::std::mem::size_of::<str_number>() as u64)
                            as _,
                    ) as *mut str_number;
                    MAX_BIB_FILES += 20;
                }
                if !eat_bib_white_space() {
                    eat_bib_print();
                    return;
                }
                if *buffer.offset(buf_ptr2 as isize) as i32 == 123i32 {
                    /*left_brace */
                    right_outer_delim = 125i32 as u8
                } else if *buffer.offset(buf_ptr2 as isize) as i32 == 40i32 {
                    /*right_brace */
                    /*left_paren */
                    right_outer_delim = 41i32 as u8
                } else {
                    bib_one_of_two_print(123i32 as u8, 40i32 as u8); /*right_paren */
                    return;
                }
                buf_ptr2 += 1i32;
                if !eat_bib_white_space() {
                    eat_bib_print();
                    return;
                }
                store_field = true;
                if !scan_and_store_the_field_value_and_eat_white() {
                    return;
                }
                if *buffer.offset(buf_ptr2 as isize) as i32 != right_outer_delim as i32 {
                    log!(
                        "Missing \"{}\" in preamble command",
                        right_outer_delim as char
                    );
                    bib_err_print();
                    return;
                }
                buf_ptr2 += 1i32;
                return;
            }
            2 => {
                if !eat_bib_white_space() {
                    eat_bib_print();
                    return;
                }
                if *buffer.offset(buf_ptr2 as isize) as i32 == 123i32 {
                    /*left_brace */
                    right_outer_delim = 125i32 as u8
                } else if *buffer.offset(buf_ptr2 as isize) as i32 == 40i32 {
                    /*right_brace */
                    /*left_paren */
                    right_outer_delim = 41i32 as u8
                } else {
                    bib_one_of_two_print(123i32 as u8, 40i32 as u8); /*right_paren */
                    return;
                }
                buf_ptr2 += 1i32;
                if !eat_bib_white_space() {
                    eat_bib_print();
                    return;
                }
                let scan_result = scan_identifier(61i32 as u8, 61i32 as u8, 61i32 as u8);
                if scan_result == ScanResult::WhiteAdjacent
                    || scan_result == ScanResult::SpecifiedCharAdjacent
                {
                } else {
                    bib_id_print(scan_result);
                    log!("a string name");
                    bib_err_print();
                    return;
                }
                lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
                cur_macro_loc = str_lookup(
                    buffer,
                    buf_ptr1,
                    buf_ptr2 - buf_ptr1,
                    13i32 as str_ilk,
                    true,
                );
                *ilk_info.offset(cur_macro_loc as isize) =
                    *hash_text.offset(cur_macro_loc as isize);
                if !eat_bib_white_space() {
                    eat_bib_print();
                    return;
                }
                if *buffer.offset(buf_ptr2 as isize) as i32 != 61i32 {
                    /*equals_sign */
                    bib_equals_sign_print();
                    return;
                }
                buf_ptr2 += 1i32;
                if !eat_bib_white_space() {
                    eat_bib_print();
                    return;
                }
                store_field = true;
                if !scan_and_store_the_field_value_and_eat_white() {
                    return;
                }
                if *buffer.offset(buf_ptr2 as isize) as i32 != right_outer_delim as i32 {
                    log!(
                        "Missing \"{}\" in string command",
                        right_outer_delim as char
                    );
                    bib_err_print();
                    return;
                }
                buf_ptr2 += 1i32;
                return;
            }
            _ => bib_cmd_confusion(),
        }
    } else {
        entry_type_loc = str_lookup(
            buffer,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            11i32 as str_ilk,
            false,
        );
        if !hash_found || *fn_type.offset(entry_type_loc as isize) as i32 != 1i32 {
            type_exists = false
        } else {
            type_exists = true
        }
    }
    if !eat_bib_white_space() {
        eat_bib_print();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 == 123i32 {
        /*left_brace */
        right_outer_delim = 125i32 as u8
    } else if *buffer.offset(buf_ptr2 as isize) as i32 == 40i32 {
        /*right_brace */
        /*left_paren */
        right_outer_delim = 41i32 as u8
    } else {
        bib_one_of_two_print(123i32 as u8, 40i32 as u8); /*right_paren */
        return;
    }
    buf_ptr2 += 1i32;
    if !eat_bib_white_space() {
        eat_bib_print();
        return;
    }
    if right_outer_delim as i32 == 41i32 {
        /*right_paren */
        scan1_white(44i32 as u8);
    } else {
        scan2_white(44i32 as u8, 125i32 as u8);
    }
    let mut tmp_ptr = buf_ptr1;
    while tmp_ptr < buf_ptr2 {
        *ex_buf.offset(tmp_ptr as isize) = *buffer.offset(tmp_ptr as isize);
        tmp_ptr += 1i32
    }
    lower_case(ex_buf, buf_ptr1, buf_ptr2 - buf_ptr1);
    if all_entries {
        lc_cite_loc = str_lookup(
            ex_buf,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            10i32 as str_ilk,
            true,
        )
    } else {
        lc_cite_loc = str_lookup(
            ex_buf,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            10i32 as str_ilk,
            false,
        )
    }
    if hash_found {
        entry_cite_ptr = *ilk_info.offset(*ilk_info.offset(lc_cite_loc as isize) as isize);
        if !all_entries || entry_cite_ptr < all_marker || entry_cite_ptr >= old_num_cites {
            if *type_list.offset(entry_cite_ptr as isize) == 0i32 {
                /*empty */
                if !all_entries && entry_cite_ptr >= old_num_cites {
                    cite_loc =
                        str_lookup(buffer, buf_ptr1, buf_ptr2 - buf_ptr1, 9i32 as str_ilk, true);
                    if !hash_found {
                        *ilk_info.offset(lc_cite_loc as isize) = cite_loc;
                        *ilk_info.offset(cite_loc as isize) = entry_cite_ptr;
                        *cite_list.offset(entry_cite_ptr as isize) =
                            *hash_text.offset(cite_loc as isize);
                        hash_found = true
                    }
                }
                current_block = 12_387_625_063_048_049_585;
            } else {
                current_block = 3_813_860_224_257_983_916;
            }
        } else if !*entry_exists.offset(entry_cite_ptr as isize) {
            ex_buf_ptr = 0i32;
            let mut tmp_ptr =
                *str_start.offset(*cite_info.offset(entry_cite_ptr as isize) as isize);
            let mut tmp_end_ptr =
                *str_start.offset((*cite_info.offset(entry_cite_ptr as isize) + 1i32) as isize);
            while tmp_ptr < tmp_end_ptr {
                *ex_buf.offset(ex_buf_ptr as isize) = *str_pool.offset(tmp_ptr as isize);
                ex_buf_ptr += 1i32;
                tmp_ptr += 1i32
            }
            lower_case(
                ex_buf,
                0i32,
                *str_start.offset((*cite_info.offset(entry_cite_ptr as isize) + 1i32) as isize)
                    - *str_start.offset(*cite_info.offset(entry_cite_ptr as isize) as isize),
            );
            lc_xcite_loc = str_lookup(
                ex_buf,
                0i32,
                *str_start.offset((*cite_info.offset(entry_cite_ptr as isize) + 1i32) as isize)
                    - *str_start.offset(*cite_info.offset(entry_cite_ptr as isize) as isize),
                10i32 as str_ilk,
                false,
            );
            if !hash_found {
                cite_key_disappeared_confusion();
            }
            if lc_xcite_loc == lc_cite_loc {
                current_block = 12_387_625_063_048_049_585;
            } else {
                current_block = 3_813_860_224_257_983_916;
            }
        } else {
            current_block = 3_813_860_224_257_983_916;
        }
        match current_block {
            12_387_625_063_048_049_585 => {}
            _ => {
                if *type_list.offset(entry_cite_ptr as isize) == 0i32 {
                    /*empty */
                    log!("The cite list is messed up");
                    print_confusion();
                    panic!();
                }
                log!("Repeated entry");
                bib_err_print();
                return;
            }
        }
    }
    /*first_time_entry */
    store_entry = true;
    if all_entries {
        let mut current_block_216: u64;
        /*273: */
        if hash_found {
            if entry_cite_ptr < all_marker {
                current_block_216 = 17_170_253_997_621_722_914;
            } else {
                *entry_exists.offset(entry_cite_ptr as isize) = true;
                cite_loc = *ilk_info.offset(lc_cite_loc as isize);
                current_block_216 = 763_224_442_071_743_734;
            }
        } else {
            cite_loc = str_lookup(buffer, buf_ptr1, buf_ptr2 - buf_ptr1, 9i32 as str_ilk, true);
            if hash_found {
                hash_cite_confusion();
            }
            current_block_216 = 763_224_442_071_743_734;
        }
        match current_block_216 {
            763_224_442_071_743_734 => {
                entry_cite_ptr = cite_ptr;
                add_database_cite(&mut cite_ptr);
            }
            _ => {}
        }
    } else if !hash_found {
        store_entry = false
    }
    if store_entry {
        /*274: */
        if type_exists {
            *type_list.offset(entry_cite_ptr as isize) = entry_type_loc
        } else {
            *type_list.offset(entry_cite_ptr as isize) = undefined;
            log!("Warning--entry type for \"");
            print_a_token();
            log!("\" isn\'t style-file defined\n");
            bib_warn_print();
        }
    }
    if !eat_bib_white_space() {
        eat_bib_print();
        return;
    }
    while *buffer.offset(buf_ptr2 as isize) as i32 != right_outer_delim as i32 {
        if *buffer.offset(buf_ptr2 as isize) as i32 != 44i32 {
            /*comma */
            bib_one_of_two_print(44i32 as u8, right_outer_delim);
            return;
        }
        buf_ptr2 += 1i32;
        if !eat_bib_white_space() {
            eat_bib_print();
            return;
        }
        if *buffer.offset(buf_ptr2 as isize) as i32 == right_outer_delim as i32 {
            break;
        }
        let scan_result = scan_identifier(61i32 as u8, 61i32 as u8, 61i32 as u8);
        if scan_result == ScanResult::WhiteAdjacent
            || scan_result == ScanResult::SpecifiedCharAdjacent
        {
        } else {
            bib_id_print(scan_result);
            log!("a field name");
            bib_err_print();
            return;
        }
        store_field = false;
        if store_entry {
            lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
            field_name_loc = str_lookup(
                buffer,
                buf_ptr1,
                buf_ptr2 - buf_ptr1,
                11i32 as str_ilk,
                false,
            );
            if hash_found && *fn_type.offset(field_name_loc as isize) as i32 == 4i32 {
                /*field */
                store_field = true
            }
        }
        if !eat_bib_white_space() {
            eat_bib_print();
            return;
        }
        if *buffer.offset(buf_ptr2 as isize) as i32 != 61i32 {
            /*equals_sign */
            bib_equals_sign_print(); /*missing */
            return;
        } /*empty */
        buf_ptr2 += 1i32; /*any_value */
        if !eat_bib_white_space() {
            eat_bib_print();
            return;
        }
        if !scan_and_store_the_field_value_and_eat_white() {
            return;
        }
    }
    buf_ptr2 += 1i32;
}
unsafe fn bst_read_command(bibtex_config: &BibtexConfig) {
    if read_seen {
        log!("Illegal, another read command");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    read_seen = true;
    if !entry_seen {
        log!("Illegal, read command before entry command");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    let sv_ptr1 = buf_ptr2;
    let sv_ptr2 = last;
    let mut tmp_ptr = sv_ptr1;
    while tmp_ptr < sv_ptr2 {
        *sv_buffer.offset(tmp_ptr as isize) = *buffer.offset(tmp_ptr as isize);
        tmp_ptr += 1i32
    }
    check_field_overflow(num_fields * num_cites);
    field_ptr = 0i32;
    while field_ptr < max_fields {
        *field_info.offset(field_ptr as isize) = 0i32;
        field_ptr += 1i32
    }
    cite_ptr = 0i32;
    while cite_ptr < max_cites {
        *type_list.offset(cite_ptr as isize) = 0i32;
        *cite_info.offset(cite_ptr as isize) = 0i32;
        cite_ptr += 1i32
    }
    old_num_cites = num_cites;
    if all_entries {
        cite_ptr = all_marker;
        while cite_ptr < old_num_cites {
            *cite_info.offset(cite_ptr as isize) = *cite_list.offset(cite_ptr as isize);
            *entry_exists.offset(cite_ptr as isize) = false;
            cite_ptr += 1i32
        }
        cite_ptr = all_marker
    } else {
        cite_ptr = num_cites;
        all_marker = 0i32
        /*any_value */
    }
    read_performed = true;
    bib_ptr = 0;
    while bib_ptr < num_bib_files {
        if verbose {
            log!("Database file #{}: ", bib_ptr + 1);
            print_bib_name();
        } else {
            write!(
                log_file.as_mut().unwrap(),
                "Database file #{}: ",
                bib_ptr as i64 + 1
            )
            .unwrap();
            log_pr_bib_name();
        }
        bib_line_num = 0;
        buf_ptr2 = last;
        while !tectonic_eof(bib_file[bib_ptr].as_mut()) {
            get_bib_command_or_entry_and_process();
        }
        let _ = bib_file[bib_ptr].take();
        bib_ptr += 1;
    }
    reading_completed = true;
    num_cites = cite_ptr;
    num_preamble_strings = preamble_ptr;
    if (num_cites - 1i32) * num_fields + crossref_num >= max_fields {
        log!("field_info index is out of range");
        print_confusion();
        panic!();
    }
    cite_ptr = 0i32;
    while cite_ptr < num_cites {
        field_ptr = cite_ptr * num_fields + crossref_num;
        if *field_info.offset(field_ptr as isize) != 0i32 {
            /*missing */
            if find_cite_locs_for_this_cite_key(*field_info.offset(field_ptr as isize)) {
                cite_loc = *ilk_info.offset(lc_cite_loc as isize);
                *field_info.offset(field_ptr as isize) = *hash_text.offset(cite_loc as isize);
                cite_parent_ptr = *ilk_info.offset(cite_loc as isize);
                field_ptr = cite_ptr * num_fields + num_pre_defined_fields;
                field_end_ptr = field_ptr - num_pre_defined_fields + num_fields;
                field_parent_ptr = cite_parent_ptr * num_fields + num_pre_defined_fields;
                while field_ptr < field_end_ptr {
                    if *field_info.offset(field_ptr as isize) == 0i32 {
                        /*missing */
                        *field_info.offset(field_ptr as isize) =
                            *field_info.offset(field_parent_ptr as isize)
                    }
                    field_ptr += 1i32;
                    field_parent_ptr += 1i32
                }
            }
        }
        cite_ptr += 1i32
    }
    if (num_cites - 1i32) * num_fields + crossref_num >= max_fields {
        log!("field_info index is out of range");
        print_confusion();
        panic!();
    }
    cite_ptr = 0i32;
    while cite_ptr < num_cites {
        field_ptr = cite_ptr * num_fields + crossref_num;
        if *field_info.offset(field_ptr as isize) != 0i32 {
            /*missing */
            if !find_cite_locs_for_this_cite_key(*field_info.offset(field_ptr as isize)) {
                if cite_hash_found {
                    hash_cite_confusion();
                }
                nonexistent_cross_reference_error();
                *field_info.offset(field_ptr as isize) = 0i32
            /*missing */
            } else {
                if cite_loc != *ilk_info.offset(lc_cite_loc as isize) {
                    hash_cite_confusion();
                }
                cite_parent_ptr = *ilk_info.offset(cite_loc as isize);
                if *type_list.offset(cite_parent_ptr as isize) == 0i32 {
                    /*empty */
                    nonexistent_cross_reference_error();
                    *field_info.offset(field_ptr as isize) = 0i32
                /*missing */
                } else {
                    field_parent_ptr = cite_parent_ptr * num_fields + crossref_num;
                    if *field_info.offset(field_parent_ptr as isize) != 0i32 {
                        /*missing */
                        /*missing */
                        /*283: */
                        log!("Warning--you\'ve nested cross references");
                        bad_cross_reference_print(*cite_list.offset(cite_parent_ptr as isize));
                        log!("\", which also refers to something\n");
                        mark_warning();
                    }
                    if !all_entries
                        && cite_parent_ptr >= old_num_cites
                        && *cite_info.offset(cite_parent_ptr as isize) < bibtex_config.min_crossrefs
                    {
                        *field_info.offset(field_ptr as isize) = 0i32
                    }
                }
            }
        }
        cite_ptr += 1i32
    }
    cite_ptr = 0i32;
    while cite_ptr < num_cites {
        if *type_list.offset(cite_ptr as isize) == 0i32 {
            /*empty */
            print_missing_entry(*cite_list.offset(cite_ptr as isize));
        } else if all_entries as i32 != 0
            || cite_ptr < old_num_cites
            || *cite_info.offset(cite_ptr as isize) >= bibtex_config.min_crossrefs
        {
            if cite_ptr > cite_xptr {
                /*286: */
                if (cite_xptr + 1i32) * num_fields > max_fields {
                    log!("field_info index is out of range");
                    print_confusion();
                    panic!();
                }
                *cite_list.offset(cite_xptr as isize) = *cite_list.offset(cite_ptr as isize);
                *type_list.offset(cite_xptr as isize) = *type_list.offset(cite_ptr as isize);
                if !find_cite_locs_for_this_cite_key(*cite_list.offset(cite_ptr as isize)) {
                    cite_key_disappeared_confusion();
                }
                if !cite_hash_found || cite_loc != *ilk_info.offset(lc_cite_loc as isize) {
                    hash_cite_confusion();
                }
                *ilk_info.offset(cite_loc as isize) = cite_xptr;
                field_ptr = cite_xptr * num_fields;
                field_end_ptr = field_ptr + num_fields;
                let mut tmp_ptr = cite_ptr * num_fields;
                while field_ptr < field_end_ptr {
                    *field_info.offset(field_ptr as isize) = *field_info.offset(tmp_ptr as isize);
                    field_ptr += 1i32;
                    tmp_ptr += 1i32
                }
            }
            cite_xptr += 1i32
        }
        cite_ptr += 1i32
    }
    num_cites = cite_xptr;
    if all_entries {
        /*287: */
        cite_ptr = all_marker; /*end_of_string */
        while cite_ptr < old_num_cites {
            if !*entry_exists.offset(cite_ptr as isize) {
                print_missing_entry(*cite_info.offset(cite_ptr as isize));
            }
            cite_ptr += 1i32
        }
    }
    entry_ints = xmalloc(
        (((num_ent_ints + 1i32) * (num_cites + 1i32)) as u64)
            .wrapping_mul(::std::mem::size_of::<i32>() as u64) as _,
    ) as *mut i32;
    int_ent_ptr = 0i32;
    while int_ent_ptr < num_ent_ints * num_cites {
        *entry_ints.offset(int_ent_ptr as isize) = 0i32;
        int_ent_ptr += 1i32
    }
    entry_strs = xmalloc(
        (((num_ent_strs + 1i32) * (num_cites + 1i32) * (ent_str_size + 1i32)) as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64) as _,
    ) as *mut u8;
    str_ent_ptr = 0i32;
    while str_ent_ptr < num_ent_strs * num_cites {
        *entry_strs.offset((str_ent_ptr * (ent_str_size + 1i32) + 0i32) as isize) = 127i32 as u8;
        str_ent_ptr += 1i32
    }
    cite_ptr = 0i32;
    while cite_ptr < num_cites {
        *cite_info.offset(cite_ptr as isize) = cite_ptr;
        cite_ptr += 1i32
    }
    read_completed = true;
    buf_ptr2 = sv_ptr1;
    last = sv_ptr2;
    let mut tmp_ptr = buf_ptr2;
    while tmp_ptr < last {
        *buffer.offset(tmp_ptr as isize) = *sv_buffer.offset(tmp_ptr as isize);
        tmp_ptr += 1i32
    }
}
unsafe fn bst_reverse_command() {
    if !read_seen {
        log!("Illegal, reverse command before read command");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("reverse");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        log!("reverse");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("reverse");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    let scan_result = scan_identifier(125i32 as u8, 37i32 as u8, 37i32 as u8);
    if scan_result == ScanResult::WhiteAdjacent || scan_result == ScanResult::SpecifiedCharAdjacent
    {
    } else {
        bst_id_print(scan_result);
        log!("reverse");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if bad_argument_token() {
        return;
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("reverse");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        bst_right_brace_print();
        log!("reverse");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1i32;
    init_command_execution();
    mess_with_entries = true;
    if num_cites > 0i32 {
        sort_cite_ptr = num_cites;
        loop {
            sort_cite_ptr -= 1i32;
            cite_ptr = *cite_info.offset(sort_cite_ptr as isize);
            execute_fn(fn_loc);
            check_command_execution();
            if sort_cite_ptr == 0i32 {
                break;
            }
        }
    };
}
unsafe fn bst_sort_command() {
    if !read_seen {
        log!("Illegal, sort command before read command");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if num_cites > 1i32 {
        quick_sort(0i32, num_cites - 1i32);
    };
}
unsafe fn bst_strings_command() {
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("strings");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        log!("strings");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1;
    if !eat_bst_white_space() {
        eat_bst_print();
        log!("strings");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    while *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        let scan_result = scan_identifier(125i32 as u8, 37i32 as u8, 37i32 as u8);
        if scan_result != ScanResult::WhiteAdjacent
            && scan_result != ScanResult::SpecifiedCharAdjacent
        {
            /*specified_char_adjacent */
            bst_id_print(scan_result); /*str_global_var */
            log!("strings"); /*HASH_SIZE */
            bst_err_print_and_look_for_blank_line();
            return;
        }
        lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
        fn_loc = str_lookup(
            buffer,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            11i32 as str_ilk,
            true,
        );
        if hash_found {
            already_seen_function_print(fn_loc);
            return;
        }
        *fn_type.offset(fn_loc as isize) = FnClass::StrGlobalVar;
        *ilk_info.offset(fn_loc as isize) = num_glb_strs;
        if num_glb_strs == max_glob_strs {
            glb_str_ptr = xrealloc(
                glb_str_ptr as *mut libc::c_void,
                ((max_glob_strs + 10i32 + 1i32) as u64)
                    .wrapping_mul(::std::mem::size_of::<str_number>() as u64) as _,
            ) as *mut str_number;
            global_strs = xrealloc(
                global_strs as *mut libc::c_void,
                (((max_glob_strs + 10i32) * (glob_str_size + 1i32)) as u64)
                    .wrapping_mul(::std::mem::size_of::<u8>() as u64) as _,
            ) as *mut u8;
            glb_str_end = xrealloc(
                glb_str_end as *mut libc::c_void,
                ((max_glob_strs + 10i32 + 1i32) as u64)
                    .wrapping_mul(::std::mem::size_of::<i32>() as u64) as _,
            ) as *mut i32;
            max_glob_strs += 10i32;
            str_glb_ptr = num_glb_strs;
            while str_glb_ptr < max_glob_strs {
                *glb_str_ptr.offset(str_glb_ptr as isize) = 0i32;
                *glb_str_end.offset(str_glb_ptr as isize) = 0i32;
                str_glb_ptr += 1i32
            }
        }
        num_glb_strs += 1;
        if !eat_bst_white_space() {
            eat_bst_print();
            log!("strings");
            bst_err_print_and_look_for_blank_line();
            return;
        }
    }
    buf_ptr2 += 1;
}
unsafe fn get_bst_command_and_process(bibtex_config: &BibtexConfig) {
    if !scan_alpha() {
        log!(
            "\"{}\" can't start a style-file command",
            *buffer.offset(buf_ptr2 as isize) as char
        );
        bst_err_print_and_look_for_blank_line();
        return;
    }
    lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
    command_num = *ilk_info.offset(str_lookup(
        buffer,
        buf_ptr1,
        buf_ptr2 - buf_ptr1,
        4i32 as str_ilk,
        false,
    ) as isize);
    if !hash_found {
        print_a_token();
        log!(" is an illegal style-file command");
        bst_err_print_and_look_for_blank_line();
        return;
    }
    match command_num {
        0 => bst_entry_command(),
        1 => bst_execute_command(),
        2 => bst_function_command(),
        3 => bst_integers_command(),
        4 => bst_iterate_command(),
        5 => bst_macro_command(),
        6 => bst_read_command(bibtex_config),
        7 => bst_reverse_command(),
        8 => bst_sort_command(),
        9 => bst_strings_command(),
        _ => {
            log!("Unknown style-file command");
            print_confusion();
            panic!();
        }
    };
}
unsafe fn setup_params() {
    ent_str_size = 250i32;
    glob_str_size = 20000i32;
    max_strings = 35307i32;
    hash_size = max_strings;
    if hash_size < 5000i32 {
        /*HASH_SIZE */
        hash_size = 5000i32
    } /*other_lex */
    hash_max = hash_size + 1i32 - 1i32; /*alpha */
    end_of_def = hash_max + 1i32; /*illegal */
    undefined = hash_max + 1i32; /*illegal */
}
unsafe fn compute_hash_prime() {
    let mut hash_want: i32 = 0; /*white_space */
    let mut k: i32 = 0; /*white_space */
    let mut j: i32 = 0; /*white_space */
    let mut o: i32 = 0; /*sep_char */
    let mut n: i32 = 0; /*sep_char */
    let mut square: i32 = 0; /*numeric */
    let mut j_prime: bool = false; /*alpha */
    hash_want = hash_size / 20i32 * 17i32; /*alpha */
    j = 1i32; /*legal_id_char */
    k = 1i32; /*illegal_id_char */
    hash_prime = 2i32; /*illegal_id_char */
    *hash_next.offset(k as isize) = hash_prime; /*illegal_id_char */
    o = 2i32; /*illegal_id_char */
    square = 9i32; /*illegal_id_char */
    while hash_prime < hash_want {
        loop {
            j += 2i32; /*illegal_id_char */
            if j == square {
                *hash_text.offset(o as isize) = j; /*illegal_id_char */
                j += 2i32; /*illegal_id_char */
                o += 1i32; /*illegal_id_char */
                square = *hash_next.offset(o as isize) * *hash_next.offset(o as isize)
            } /*illegal_id_char */
            n = 2i32; /*illegal_id_char */
            j_prime = true; /*illegal_id_char */
            while n < o && j_prime as i32 != 0 {
                while *hash_text.offset(n as isize) < j {
                    let fresh11 = &mut (*hash_text.offset(n as isize)); /*illegal_id_char */
                    *fresh11 += 2i32 * *hash_next.offset(n as isize)
                } /*empty */
                if *hash_text.offset(n as isize) == j {
                    j_prime = false
                }
                n += 1i32
            }
            if j_prime {
                break;
            }
        }
        k += 1;
        hash_prime = j;
        *hash_next.offset(k as isize) = hash_prime
    }
}

unsafe fn badness() -> i32 {
    let mut bad = 0;
    if min_print_line < 3 {
        bad = 1;
    }
    if max_print_line <= min_print_line {
        bad = 10 * bad + 2;
    }
    if max_print_line >= buf_size {
        bad = 10 * bad + 3;
    }
    if hash_prime < 128 {
        bad = 10 * bad + 4;
    }
    if hash_prime > hash_size {
        bad = 10 * bad + 5;
    }
    /*if hash_prime >= (16384 - 64) {
        bad = 10 * bad + 6;
    }*/
    if max_strings > hash_size {
        bad = 10 * bad + 7;
    }
    if max_cites > max_strings {
        bad = 10 * bad + 8;
    }
    if 10i32 < 2i32 * 4i32 + 2i32 {
        bad = 100i32 * bad + 22i32
    }
    bad
}

unsafe fn initialize(mut aux_file_name: *const i8) -> i32 {
    let mut i: i32 = 0;
    let mut k: hash_loc = 0;
    if badness() != 0 {
        return 1i32;
    }
    history = TTHistory::SPOTLESS;
    i = 0i32;
    while i <= 127i32 {
        char_width[i as usize] = 0i32;
        i += 1
    }
    char_width[32] = 278i32;
    char_width[33] = 278i32;
    char_width[34] = 500i32;
    char_width[35] = 833i32;
    char_width[36] = 500i32;
    char_width[37] = 833i32;
    char_width[38] = 778i32;
    char_width[39] = 278i32;
    char_width[40] = 389i32;
    char_width[41] = 389i32;
    char_width[42] = 500i32;
    char_width[43] = 778i32;
    char_width[44] = 278i32;
    char_width[45] = 333i32;
    char_width[46] = 278i32;
    char_width[47] = 500i32;
    char_width[48] = 500i32;
    char_width[49] = 500i32;
    char_width[50] = 500i32;
    char_width[51] = 500i32;
    char_width[52] = 500i32;
    char_width[53] = 500i32;
    char_width[54] = 500i32;
    char_width[55] = 500i32;
    char_width[56] = 500i32;
    char_width[57] = 500i32;
    char_width[58] = 278i32;
    char_width[59] = 278i32;
    char_width[60] = 278i32;
    char_width[61] = 778i32;
    char_width[62] = 472i32;
    char_width[63] = 472i32;
    char_width[64] = 778i32;
    char_width[65] = 750i32;
    char_width[66] = 708i32;
    char_width[67] = 722i32;
    char_width[68] = 764i32;
    char_width[69] = 681i32;
    char_width[70] = 653i32;
    char_width[71] = 785i32;
    char_width[72] = 750i32;
    char_width[73] = 361i32;
    char_width[74] = 514i32;
    char_width[75] = 778i32;
    char_width[76] = 625i32;
    char_width[77] = 917i32;
    char_width[78] = 750i32;
    char_width[79] = 778i32;
    char_width[80] = 681i32;
    char_width[81] = 778i32;
    char_width[82] = 736i32;
    char_width[83] = 556i32;
    char_width[84] = 722i32;
    char_width[85] = 750i32;
    char_width[86] = 750i32;
    char_width[87] = 1028i32;
    char_width[88] = 750i32;
    char_width[89] = 750i32;
    char_width[90] = 611i32;
    char_width[91] = 278i32;
    char_width[92] = 500i32;
    char_width[93] = 278i32;
    char_width[94] = 500i32;
    char_width[95] = 278i32;
    char_width[96] = 278i32;
    char_width[97] = 500i32;
    char_width[98] = 556i32;
    char_width[99] = 444i32;
    char_width[100] = 556i32;
    char_width[101] = 444i32;
    char_width[102] = 306i32;
    char_width[103] = 500i32;
    char_width[104] = 556i32;
    char_width[105] = 278i32;
    char_width[106] = 306i32;
    char_width[107] = 528i32;
    char_width[108] = 278i32;
    char_width[109] = 833i32;
    char_width[110] = 556i32;
    char_width[111] = 500i32;
    char_width[112] = 556i32;
    char_width[113] = 528i32;
    char_width[114] = 392i32;
    char_width[115] = 394i32;
    char_width[116] = 389i32;
    char_width[117] = 556i32;
    char_width[118] = 528i32;
    char_width[119] = 722i32;
    char_width[120] = 528i32;
    char_width[121] = 528i32;
    char_width[122] = 444i32;
    char_width[123] = 500i32;
    char_width[124] = 1000i32;
    char_width[125] = 500i32;
    char_width[126] = 500i32;
    k = 1i32;
    while k <= hash_max {
        *hash_next.offset(k as isize) = 0i32;
        *hash_text.offset(k as isize) = 0i32;
        k += 1
    }
    hash_used = hash_max + 1i32;
    pool_ptr = 0i32;
    str_ptr = 1i32;
    *str_start.offset(str_ptr as isize) = pool_ptr;
    bib_ptr = 0;
    bib_seen = false;
    bst_str = 0i32;
    bst_seen = false;
    cite_ptr = 0i32;
    citation_seen = false;
    all_entries = false;
    wiz_def_ptr = 0i32;
    num_ent_ints = 0i32;
    num_ent_strs = 0i32;
    num_fields = 0i32;
    str_glb_ptr = 0i32;
    while str_glb_ptr < max_glob_strs {
        *glb_str_ptr.offset(str_glb_ptr as isize) = 0i32;
        *glb_str_end.offset(str_glb_ptr as isize) = 0i32;
        str_glb_ptr += 1i32
    }
    num_glb_strs = 0i32;
    entry_seen = false;
    read_seen = false;
    read_performed = false;
    reading_completed = false;
    read_completed = false;
    impl_fn_num = 0i32;
    out_buf_length = 0i32;
    pre_def_certain_strings();
    get_the_top_level_aux_file_name(aux_file_name)
}
/* tectonic/bibtex.h
   Copyright 2017 the Tectonic Project
   Licensed under the MIT License.
*/
pub unsafe fn bibtex_main(bibtex_config: &BibtexConfig, mut aux_file_name: *const i8) -> TTHistory {
    pool_size = POOL_SIZE;
    buf_size = BUF_SIZE;
    MAX_BIB_FILES = MAX_BIBFILES;
    max_glob_strs = MAX_GLOB_STRS;
    max_fields = MAX_FIELDS;
    max_cites = MAX_CITES;
    wiz_fn_space = WIZ_FN_SPACE;
    lit_stk_size = LIT_STK_SIZE;
    standard_output = ttstub_output_open_stdout();
    if standard_output.is_none() {
        return TTHistory::FATAL_ERROR;
    }
    setup_params();
    entry_ints = ptr::null_mut();
    entry_strs = ptr::null_mut();
    bib_file = Vec::with_capacity(MAX_BIB_FILES + 1);
    bib_list = xmalloc(
        ((MAX_BIB_FILES + 1) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64) as _,
    ) as *mut str_number;
    wiz_functions = xmalloc(
        ((wiz_fn_space + 1i32) as u64).wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64) as _,
    ) as *mut hash_ptr2;
    field_info = xmalloc(
        ((max_fields + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64) as _,
    ) as *mut str_number;
    s_preamble = xmalloc(
        ((MAX_BIB_FILES + 1) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64) as _,
    ) as *mut str_number;
    str_pool =
        xmalloc(((pool_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as _)
            as *mut u8;
    buffer =
        xmalloc(((buf_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as _)
            as buf_type;
    sv_buffer =
        xmalloc(((buf_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as _)
            as buf_type;
    ex_buf =
        xmalloc(((buf_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as _)
            as buf_type;
    out_buf =
        xmalloc(((buf_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as _)
            as buf_type;
    name_tok = xmalloc(
        ((buf_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<buf_pointer>() as u64) as _,
    ) as *mut buf_pointer;
    name_sep_char =
        xmalloc(((buf_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as _)
            as *mut u8;
    glb_str_ptr = xmalloc(
        (max_glob_strs as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64) as _,
    ) as *mut str_number;
    global_strs = xmalloc(
        ((max_glob_strs * (glob_str_size + 1i32)) as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64) as _,
    ) as *mut u8;
    glb_str_end =
        xmalloc((max_glob_strs as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64) as _)
            as *mut i32;
    cite_list = xmalloc(
        ((max_cites + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64) as _,
    ) as *mut str_number;
    type_list = xmalloc(
        ((max_cites + 1i32) as u64).wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64) as _,
    ) as *mut hash_ptr2;
    entry_exists = xmalloc(
        ((max_cites + 1i32) as u64).wrapping_mul(::std::mem::size_of::<bool>() as u64) as _,
    ) as *mut bool;
    cite_info = xmalloc(
        ((max_cites + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64) as _,
    ) as *mut str_number;
    str_start = xmalloc(
        ((max_strings + 1i32) as u64).wrapping_mul(::std::mem::size_of::<pool_pointer>() as u64)
            as _,
    ) as *mut pool_pointer;
    hash_next = xmalloc(
        ((hash_max + 1i32) as u64).wrapping_mul(::std::mem::size_of::<hash_pointer>() as u64) as _,
    ) as *mut hash_pointer;
    hash_text = xmalloc(
        ((hash_max + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64) as _,
    ) as *mut str_number;
    hash_ilk = xmalloc(
        ((hash_max + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_ilk>() as u64) as _,
    ) as *mut str_ilk;
    ilk_info =
        xmalloc(((hash_max + 1i32) as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64) as _)
            as *mut i32;
    fn_type = xmalloc(
        ((hash_max + 1i32) as u64).wrapping_mul(::std::mem::size_of::<FnClass>() as u64) as _,
    ) as *mut FnClass;
    lit_stack = xmalloc(
        ((lit_stk_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64) as _,
    ) as *mut i32;
    lit_stk_type = xmalloc(
        ((lit_stk_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<StkType>() as u64) as _,
    ) as *mut StkType;
    compute_hash_prime();
    if initialize(aux_file_name) != 0 {
        /* TODO: log initialization or get_the_..() error */
        return TTHistory::FATAL_ERROR;
    }
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let _ = panic::catch_unwind(|| {
        if verbose {
            log!("This is BibTeX, Version 0.99d\n");
        } else {
            writeln!(log_file.as_mut().unwrap(), "This is BibTeX, Version 0.99d").unwrap();
        }
        writeln!(
            log_file.as_mut().unwrap(),
            "Capacity: max_strings={}, hash_size={}, hash_prime={}",
            max_strings as i64,
            hash_size as i64,
            hash_prime as i64
        )
        .unwrap();
        if verbose {
            log!("The top-level auxiliary file: ");
            print_aux_name();
        } else {
            write!(log_file.as_mut().unwrap(), "The top-level auxiliary file: ").unwrap();
            log_pr_aux_name();
        }
        loop {
            aux_ln_stack[aux_ptr as usize] += 1;
            if !input_ln(&mut aux_file[aux_ptr as usize]) {
                if pop_the_aux_stack() != 0 {
                    break;
                }
            } else {
                get_aux_command_and_process();
            }
        }
        last_check_for_aux_errors();
        if bst_str != 0i32 {
            bst_line_num = 0i32;
            bbl_line_num = 1i32;
            buf_ptr2 = last;
            let prev_hook = panic::take_hook();
            panic::set_hook(Box::new(|_| {}));
            let _ = panic::catch_unwind(|| {
                while eat_bst_white_space() {
                    get_bst_command_and_process(bibtex_config);
                }
            });
            panic::set_hook(prev_hook);
            let _ = bst_file.take();
        }
        ttstub_output_close(bbl_file.take().unwrap());
    });
    panic::set_hook(prev_hook);

    /*456:*/
    if read_performed && !reading_completed {
        log!("Aborted at line {} of file ", bib_line_num,);
        print_bib_name();
    }
    match history {
        TTHistory::SPOTLESS => {}
        TTHistory::WARNING_ISSUED => {
            if err_count == 1 {
                log!("(There was 1 warning)\n");
            } else {
                log!("(There were {} warnings)\n", err_count,);
            }
        }
        TTHistory::ERROR_ISSUED => {
            if err_count == 1 {
                log!("(There was 1 error message)\n");
            } else {
                log!("(There were {} error messages)\n", err_count,);
            }
        }
        TTHistory::FATAL_ERROR => log!("(That was a fatal error)\n"),
    }
    ttstub_output_close(log_file.take().unwrap());
    history
}
