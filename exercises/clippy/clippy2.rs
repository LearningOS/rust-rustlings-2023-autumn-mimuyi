/*
 * @Descripttion: 
 * @version: 
 * @Author: Muyi
 * @Date: 2023-10-19 15:06:13
 * @LastEditors: Muyi
 * @LastEditTime: 2023-10-19 15:06:13
 * @E-mail: Muyi_Mi@aliyun.com
 */
// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x
    }
    println!("{}", res);
}
