/*
 * @Descripttion: 
 * @version: 
 * @Author: Muyi
 * @Date: 2023-10-13 17:41:38
 * @LastEditors: Muyi
 * @LastEditTime: 2023-10-13 17:41:39
 * @E-mail: Muyi_Mi@aliyun.com
 */
// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age)/* your pattern here */ = cat;

    println!("{} is {} years old.", name, age);
}
