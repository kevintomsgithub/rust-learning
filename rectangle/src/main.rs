#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

impl Rectangle {
    // associative functions - similar to static method in python class
    // where the first argument is not self
    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size,
        }
    }

    fn can_hold(rect1: Rectangle, rect2: Rectangle) -> bool {
        rect1.length > rect2.length && rect1.width > rect2.width
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 10,
        width: 20,
    };

    let rect2 = Rectangle {
        length: 5,
        width: 10,
    };

    let square = Rectangle::square(10);

    println!("Rectangle: {:#?}", rect1);
    println!("Square: {:#?}", square);

    println!("Rectangle area: {:?}", rect1.area());
    println!("Square area: {:?}", square.area());

    let result = Rectangle::can_hold(rect1, rect2);
    println!("Can hold: {result}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 10,
            width: 20,
        };

        let smaller = Rectangle {
            length: 5,
            width: 10,
        };

        assert!(Rectangle::can_hold(larger, smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            length: 10,
            width: 20,
        };

        let smaller = Rectangle {
            length: 5,
            width: 10,
        };

        assert!(!Rectangle::can_hold(smaller, larger));
    }
}
