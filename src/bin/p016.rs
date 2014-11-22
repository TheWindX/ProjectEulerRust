#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(slicing_syntax)]

extern crate num;
extern crate common;

use std::iter::AdditiveIterator;
use num::bigint::BigInt;
use common::Solver;

fn compute(base: uint, exp: uint) -> uint {
    let base: BigInt = FromPrimitive::from_uint(base).unwrap();
    num::pow(base, exp)
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
}

fn solve() -> String { compute(2, 1000).to_string() }

fn main() { Solver::new("1366", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn two_fifteen() {
        assert_eq!(26, super::compute(2, 15));
    }
}
