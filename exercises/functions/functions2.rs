/*
 * @Descripttion: 
 * @version: 
 * @Author: Muyi
 * @Date: 2023-10-12 15:17:56
 * @LastEditors: Muyi
 * @LastEditTime: 2023-10-12 15:17:57
 * @E-mail: Muyi_Mi@aliyun.com
 */
// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
