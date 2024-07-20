// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// 普通结构体
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 元组结构体
struct Color(i32, i32, i32);

// 单元结构体
struct Unit;

// 普通结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// 为 Rectangle 结构体实现方法
impl Rectangle {
    // 计算矩形面积的方法
    fn area(&self) -> f32 {
        let Rectangle { top_left: Point { x: x1, y: y1 }, bottom_right: Point { x: x2, y: y2 } } = self;
        let width = (x2 - x1).abs();
        let height = (y2 - y1).abs();
        width * height
    }

    // 创建一个正方形的方法
    fn square(top_left: Point, size: f32) -> Rectangle {
        let bottom_right = Point {
            x: top_left.x + size,
            y: top_left.y - size,
        };
        Rectangle { top_left, bottom_right }
    }
}

fn main() {
    // 创建普通结构体实例
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // 打印调试结构体
    println!("{:?}", peter);

    // 实例化一个 Point 结构体
    let point: Point = Point { x: 10.3, y: 0.4 };
    let another_point: Point = Point { x: 5.2, y: 0.2 };

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用结构体更新语法创建一个新的 Point
    let bottom_right = Point { x: 5.2, ..another_point };

    // 通过 let 绑定解构 point
    let Point { x: left_edge, y: top_edge } = point;

    // 创建一个 Rectangle 实例
    let rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // 计算并打印矩形的面积
    println!("The area of the rectangle is {}", rectangle.area());

    // 实例化一个单元结构体
    let _unit = Unit;

    // 实例化一个元组结构体
    let pair = Color(1, 0.1, 0.2);

    // 访问元组结构体的字段
    println!("pair contains {:?}, {:?}, and {:?}", pair.0, pair.1, pair.2);

    // 解构一个元组结构体
    let Color(r, g, b) = pair;

    println!("Color contains {}, {}, and {}", r, g, b);

    // 创建一个正方形并打印其面积
    let square_top_left = Point { x: 0.0, y: 0.0 };
    let square_size = 3.0;
    let square = Rectangle::square(square_top_left, square_size);
    println!("The area of the square is {}", square.area());
}
