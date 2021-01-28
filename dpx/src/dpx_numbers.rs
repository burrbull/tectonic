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
#![allow(non_camel_case_types)]

use std::io::Read;
pub(crate) type fixword = i32;

pub(crate) trait GetFromFile {
    fn get<R>(file: &mut R) -> Self
    where
        R: Read;
}

pub(crate) fn skip_bytes<R>(n: u32, file: &mut R)
where
    R: Read,
{
    for _ in 0..n {
        u8::get(file);
    }
}

impl GetFromFile for u8 {
    fn get<R>(file: &mut R) -> Self
    where
        R: Read,
    {
        let mut buf = [0u8; 1];
        file.read(&mut buf).expect("File ended prematurely");
        buf[0]
    }
}

impl GetFromFile for i8 {
    fn get<R>(file: &mut R) -> Self
    where
        R: Read,
    {
        u8::get(file) as i8
    }
}

impl GetFromFile for u16 {
    fn get<R>(file: &mut R) -> Self
    where
        R: Read,
    {
        let mut buf = [0u8; 2];
        file.read(&mut buf).expect("File ended prematurely");
        u16::from_be_bytes(buf)
    }
}

impl GetFromFile for i16 {
    fn get<R>(file: &mut R) -> Self
    where
        R: Read,
    {
        let mut buf = [0u8; 2];
        file.read(&mut buf).expect("File ended prematurely");
        i16::from_be_bytes(buf)
    }
}

impl GetFromFile for i32 {
    fn get<R>(file: &mut R) -> Self
    where
        R: Read,
    {
        let mut buf = [0u8; 4];
        file.read(&mut buf).expect("File ended prematurely");
        i32::from_be_bytes(buf)
    }
}

impl GetFromFile for u32 {
    fn get<R>(file: &mut R) -> Self
    where
        R: Read,
    {
        let mut buf = [0u8; 4];
        file.read(&mut buf).expect("File ended prematurely");
        u32::from_be_bytes(buf)
    }
}

pub(crate) fn get_unsigned_triple<R: Read>(file: &mut R) -> u32 {
    let mut buf = [0u8; 4];
    file.read(&mut buf[1..]).expect("File ended prematurely");
    u32::from_be_bytes(buf)
}

pub(crate) fn get_unsigned_num<R: Read>(file: &mut R, num: u8) -> u32 {
    let mut val = u8::get(file) as u32;
    match num {
        3 => {
            if val > 0x7f {
                val = val.wrapping_sub(0x100);
            }
            val = (val << 8) | u8::get(file) as u32;
            val = (val << 8) | u8::get(file) as u32;
            val = (val << 8) | u8::get(file) as u32;
        }
        2 => {
            val = (val << 8) | u8::get(file) as u32;
            val = (val << 8) | u8::get(file) as u32;
        }
        1 => {
            val = (val << 8) | u8::get(file) as u32;
        }
        _ => {}
    }
    val
}

/// Compute a signed quad that must be positive
pub(crate) fn get_positive_quad<R: Read>(file: &mut R, type_0: &str, name: &str) -> u32 {
    let val = i32::get(file);
    assert!(val >= 0, "Bad {}: negative {}: {}", type_0, name, val);
    val as u32
}

pub(crate) fn sqxfw(mut sq: i32, mut fw: fixword) -> i32 {
    let mut sign = 1;
    /* Make positive. */
    if sq < 0 {
        sign = -sign; /* 1<<3 is for rounding */
        sq = -sq
    }
    if fw < 0 {
        sign = -sign;
        fw = -fw
    }
    let a = sq as u32 >> 16;
    let b = sq as u32 & 0xffff;
    let c = fw as u32 >> 16;
    let d = fw as u32 & 0xffff;
    let ad = a.wrapping_mul(d);
    let bd = b.wrapping_mul(d);
    let bc = b.wrapping_mul(c);
    let ac = a.wrapping_mul(c);
    let e = bd >> 16;
    let f = ad >> 16;
    let g = ad & 0xffff;
    let h = bc >> 16;
    let i = bc & 0xffff;
    let j = ac >> 16;
    let k = ac & 0xffff;
    let mut result = (e
        .wrapping_add(g)
        .wrapping_add(i)
        .wrapping_add((1 << 3) as u32)
        >> 4) as i32;
    result = (result as u32).wrapping_add(f.wrapping_add(h).wrapping_add(k) << 12) as i32 as i32;
    result = (result as u32).wrapping_add(j << 28) as i32 as i32;
    if sign > 0 {
        result
    } else {
        -result
    }
}
/* When reading numbers from binary files 1, 2, or 3 bytes are
   interpreted as either signed or unsigned.

   Four bytes from DVI, PK, TFM, or VF files always yield a signed
   32-bit integer (i32), but some of them must not be negative.

   Four byte numbers from JPEG2000, OpenType, or TrueType files are
   mostly unsigned (u32) and occasionally signed (i32).
*/
