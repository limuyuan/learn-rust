fn main() {
    println!("I put this on leetcode: https://leetcode-cn.com/problems/fibonacci-number/solution/rustfei-bo-na-qi-shu-lie-jie-fa-by-limuy-4y2b/");
}

struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut f_n_2: i32 = 0;
        let mut f_n_1: i32 = 1;
        let mut f_n: i32 = 0;

        match n {
            0 => f_n = 0,
            1 => f_n = 1,
            _ => (),
        }

        for i in 2..n + 1 {
            f_n = f_n_2 + f_n_1;
            f_n_2 = f_n_1;
            f_n_1 = f_n;
        }

        f_n
    }
}
