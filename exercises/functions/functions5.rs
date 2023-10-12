/*
 * @Descripttion: 
 * @version: 
 * @Author: Muyi
 * @Date: 2023-10-12 15:22:18
 * @LastEditors: Muyi
 * @LastEditTime: 2023-10-12 15:22:19
 * @E-mail: Muyi_Mi@aliyun.com
 */
// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
