/*
 * @Descripttion: 
 * @version: 
 * @Author: Muyi
 * @Date: 2023-10-18 10:09:04
 * @LastEditors: Muyi
 * @LastEditTime: 2023-10-18 10:09:04
 * @E-mail: Muyi_Mi@aliyun.com
 */
// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
