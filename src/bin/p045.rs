#![warn(unused, bad_style,
        unused_qualifications, unused_typecasts, unused_results)]

extern crate common;

use common::Solver;

fn triangle(i: uint) -> uint {
    let n = i + 1;
    n * (n + 1) / 2
}

fn pentagonal(i: uint) -> uint {
    let n = i + 1;
    n * (3 * n - 1) / 2
}

fn hexagonal(i: uint) -> uint {
    let n = i + 1;
    n * (2 * n - 1)
}

fn compute(start: uint) -> uint {
    let mut n = start;

    let mut t_i = 0;
    let mut p_i = 0;
    let mut h_i = 0;

    loop {
        let mut t = triangle(t_i);
        while t < n {
            t_i += 1;
            t = triangle(t_i);
        }
        if t > n { n = t; }

        let mut p = pentagonal(p_i);
        while p < n {
            p_i += 1;
            p = pentagonal(p_i);
        }
        if p > n { n = p; continue }

        let mut h = hexagonal(h_i);
        while h < n {
            h_i += 1;
            h = hexagonal(h_i);
        }
        if h > n { n = h; continue }

        break
    }

    triangle(t_i)
}

fn solve() -> String {
    compute(40755 + 1).to_string()
}

fn main() { Solver::new("1533776805", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn first() {
        assert_eq!(1, super::compute(0));
        assert_eq!(40755, super::compute(2));
    }
}