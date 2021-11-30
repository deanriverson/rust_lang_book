#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let scale = 2;

    let rect1 = Rect {
        width: dbg!(15 * scale),
        height: 50,
    };

    let rect2 = Rect {
        width: 10,
        height: 40,
    };

    let rect3 = Rect {
        width: 60,
        height: 45,
    };

    dbg!(&rect1);
    dbg!(&rect2);
    dbg!(&rect3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rect::square(32);
    dbg!(&square1);

    println!(
        "The area of the square is {} square pixels.",
        square1.area()
    );
}

// fn area(width: u32, height: u32) -> u32 {
// fn area(rect: (u32, u32)) -> u32 {
// fn area(rect: Rect) -> u32 {
//     rect.width * rect.height
// }
