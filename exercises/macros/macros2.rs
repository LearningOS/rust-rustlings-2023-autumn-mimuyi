/*
 * @Descripttion: 
 * @version: 
 * @Author: Muyi
 * @Date: 2023-10-19 15:03:29
 * @LastEditors: Muyi
 * @LastEditTime: 2023-10-19 15:03:30
 * @E-mail: Muyi_Mi@aliyun.com
 */
// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}


