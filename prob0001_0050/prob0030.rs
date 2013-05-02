use std::sort::{ quick_sort };

use common::calc::{ combinate_overlap, num_to_digits, pow };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 30,
    answer: "443839",
    solver: solve
};

// 9^5     = 59049
// 9999    => 9^5 * 4 = 236196
// 99999   => 9^5 * 5 = 295245
// 999999  => 9^5 * 6 = 354294
// 9999999 => 9^5 * 7 = 413343

// 1-6 digits numbers meet conditions
fn solve() -> ~str {
    let len = 7;
    let pows = vec::from_fn(10, |i| pow(i, 5));

    let mut sum = 0;
    for combinate_overlap(~[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], len) |comb| {
        let num = comb.foldl(0u, |a, &e| a + pows[e]);

        let mut nums = num_to_digits(num, 10);
        quick_sort(nums, |a, b| a < b);

        let zero_len = len - nums.len();
        if comb.tailn(zero_len) == nums &&
            comb.slice(0, zero_len).all(|&x| x == 0) {
            sum += num;
        }
    }

    return (sum - 1).to_str();  // remove 1
}
