// Copyright 2018 Đỗ Hoàng Anh Duy.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait FromStrRadix
where
    Self: std::marker::Sized,
{
    fn from_str(s: &str, base: u32) -> Result<Self, String>;
}

macro_rules! auto_impl {
    ($t:ty) => {
        impl FromStrRadix for $t {
            fn from_str(s: &str, base: u32) -> Result<Self, String> {
                Self::from_str_radix(s, base).map_err(|e| e.to_string())
            }
        }
    };
}

auto_impl!(i8);
auto_impl!(i16);
auto_impl!(i32);
auto_impl!(i64);

trait ToStrRadix {
    fn to_str(&self, out_base: u32) -> String;
}

use std::fmt;
impl<T> ToStrRadix for T
where
    T: fmt::UpperHex + fmt::Binary + fmt::Octal + fmt::Display,
{
    fn to_str(&self, out_base: u32) -> String {
        match out_base {
            2 => format!("{:b}", self),
            8 => format!("{:o}", self),
            10 => format!("{}", self),
            16 => format!("{:X}", self),
            _ => panic!(),
        }
    }
}

fn do_process<T: FromStrRadix + ToStrRadix>(inp: &str, in_base: u32, out_base: u32) -> String {
    match T::from_str(inp, in_base) {
        Err(s) => s,
        Ok(x) => x.to_str(out_base),
    }
}

pub fn process(inp: &str, in_base: u32, out_base: u32, size: u32) -> String {
    let inp = inp.trim();
    if inp.is_empty() {
        return String::new();
    }
    match size {
        8 => do_process::<i8>(inp, in_base, out_base),
        16 => do_process::<i16>(inp, in_base, out_base),
        32 => do_process::<i32>(inp, in_base, out_base),
        64 => do_process::<i64>(inp, in_base, out_base),
        _ => panic!(),
    }
}
