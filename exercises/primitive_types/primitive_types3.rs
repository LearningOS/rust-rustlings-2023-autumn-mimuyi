/*
 * @Descripttion: 
 * @version: 
 * @Author: Muyi
 * @Date: 2023-10-13 17:37:20
 * @LastEditors: Muyi
 * @LastEditTime: 2023-10-13 17:37:20
 * @E-mail: Muyi_Mi@aliyun.com
 */
// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let a: Vec<i32> = vec![1];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
