/*
 * @Descripttion: 
 * @version: 
 * @Author: Muyi
 * @Date: 2023-10-19 10:45:29
 * @LastEditors: Muyi
 * @LastEditTime: 2023-10-19 10:45:30
 * @E-mail: Muyi_Mi@aliyun.com
 */
// tests3.rs
//
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the
// result we expect to get when we call `is_even(5)`.
//
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a
// hint.

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(is_even(2));
    }
}
