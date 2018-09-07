#![allow(dead_code)]

use std::mem;
use std::slice;

/// Calculate the 8-bit checksum of a `<T>`
///
/// In order to interpret a piece of data as a slice of bytes you need to perform three steps:
/// 1) obtain a raw pointer to the data
/// 2) convert it to a pointer to u8
/// 3) convert that to a slice of u8
fn checksum<T>(data: &T) -> u8 {
    let p = data as *const T as *const u8;
    let s = unsafe { slice::from_raw_parts(p, mem::size_of::<T>()) };

    sum_slice(s)
}

/// Wrapping add (sum % 256) the bytes of a slice
fn sum_slice(data: &[u8]) -> u8 {
    data.iter().fold(0, |acc, &byte| acc.wrapping_add(byte))
}

#[cfg(test)]
mod tests {
    use super::{checksum, sum_slice};

    #[test]
    fn empty_slice() {
        let empty: &[u8] = &[];

        assert_eq!(sum_slice(&empty), 0);
    }

    #[test]
    fn non_empty_slice() {
        let non_empty: &[u8] = &[255, 254, 127, 126];

        assert_eq!(sum_slice(&non_empty), 250);
    }

    #[test]
    fn zero_field_struct() {
        struct Zero;

        assert_eq!(checksum(&Zero), 0);
    }

    #[test]
    fn single_field_struct() {
        struct Single(u32);

        let single = Single(0xFFFFFFFF);

        assert_eq!(checksum(&single), 252)
    }

    #[test]
    fn multi_field_struct() {
        struct Multi(u32, u32);

        let multi = Multi(0xFFFFFFFF, 0x5);

        assert_eq!(checksum(&multi), 1);
    }
}
