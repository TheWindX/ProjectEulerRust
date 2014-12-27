#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate num;

use std::iter;
use num::Integer;

fn a(n: u64) -> u64 {
    if n == 1 { return 1 }

    iter::Unfold::new((1, 1), |st| {
            let (x, k) = *st;
            *st = ((x * 10 + 1) % n, k + 1);
            Some((x, k))
        }).find(|&(x, _)| x == 0)
        .unwrap()
        .1
}

fn solve() -> String {
    let limit = 1000001;

    iter::count(limit, 2)
        .filter(|&n| !n.is_multiple_of(&5))
        .find(|&n| a(n) >= limit)
        .unwrap()
        .to_string()
}

problem!("1000023", solve);

#[cfg(test)]
mod tests {
    use std::iter;
    use num::Integer;

    mod naive {
        use std::iter;
        use num::{One, Zero, Integer, BigUint};

        pub fn r(k: u64) -> BigUint {
            let mut r: BigUint = Zero::zero();
            let ten: BigUint = FromPrimitive::from_u64(10).unwrap();
            let one: BigUint = One::one();
            for _ in range(0, k) {
                r = &r * &ten + &one;
            }
            r
        }

        pub fn a(n: u64) -> u64 {
            let n = FromPrimitive::from_u64(n).unwrap();
            iter::count(1, 1)
                .find(|&k| r(k).is_multiple_of(&n))
                .unwrap()
        }
    }

    #[test]
    fn naive_r() {
        assert_eq!("1".to_string(), naive::r(1).to_string());
        assert_eq!("11".to_string(), naive::r(2).to_string());
        assert_eq!("111".to_string(), naive::r(3).to_string());
    }

    #[test]
    fn naive_a() {
        assert_eq!(6, naive::a(7));
        assert_eq!(5, naive::a(41));
    }

    #[test]
    fn cmp_with_naive() {
        for n in iter::range_step(1, 100, 2) {
            if n.is_multiple_of(&5) { continue; }
            assert_eq!(naive::a(n), super::a(n));
        }
    }

    #[test]
    fn a() {
        assert_eq!(6, super::a(7));
        assert_eq!(5, super::a(41));
    }
}