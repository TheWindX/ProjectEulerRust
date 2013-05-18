#[link(name = "prob0035", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use core::iterator::{ IteratorUtil };
use common::prime;
use common::calc::{ num_to_digits };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 35,
    answer: "55",
    solver: solve
};

#[inline(always)]
fn is_circular_prime(n: uint) -> bool {
    let buf = num_to_digits(n, 10);

    for uint::range(1, buf.len()) |i| {
        let mut num = 0;
        for uint::range(0, buf.len()) |j| {
            num = num * 10 + (buf[(i + j) % buf.len()] as uint);
        }
        if !prime::contains(num) { return false; }
    }

    return true;
}

pub fn solve() -> ~str {
    return prime::iter()
        .take_while(|&p| p < 1000000)
        .filter(|&p| is_circular_prime(p))
        .count()
        .to_str();
}
