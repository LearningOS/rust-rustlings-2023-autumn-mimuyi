/*
 * @Descripttion: 
 * @version: 
 * @Author: Muyi
 * @Date: 2023-10-19 15:03:03
 * @LastEditors: Muyi
 * @LastEditTime: 2023-10-19 15:03:03
 * @E-mail: Muyi_Mi@aliyun.com
 */
// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
