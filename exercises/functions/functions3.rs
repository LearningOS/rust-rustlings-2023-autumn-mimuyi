/*
 * @Descripttion: 
 * @version: 
 * @Author: Muyi
 * @Date: 2023-10-12 15:19:24
 * @LastEditors: Muyi
 * @LastEditTime: 2023-10-12 15:19:25
 * @E-mail: Muyi_Mi@aliyun.com
 */
// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(6);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
