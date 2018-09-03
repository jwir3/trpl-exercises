// The next line is an annotation telling the struct to use the Debug derived trait.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // Note that because the first parameter is not a version of self, this is an associated
    // (i.e. static) method.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let rectangle = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // Note that {:?} specifies debug output, and {:#?} specifies pretty debug output
    // println!("****DEBUG: {:#?}", rectangle);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rectangle.area()
    // );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::square(10);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold square? {}", rect1.can_hold(&square));
}

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }
