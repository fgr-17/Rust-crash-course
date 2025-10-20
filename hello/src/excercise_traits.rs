pub trait Colorful {
    fn color(&self) -> String;
}

pub struct Hat {
    pub size: i32,
}

impl Colorful for Hat {
    fn color(&self) -> String {
        if self.size > 0 && self.size <= 5 {
            String::from("red")
        } else if self.size >= 6 && self.size <= 7 {
            String::from("green")
        } else {
            String::from("blue")
        }
    }
}

impl Colorful for i32 {
    fn color(&self) -> String {
        if self.is_even() {
            String::from("orange")
        } else {
            String::from("purple")
        }
    }
}

pub fn describe_three_hats(hat1: &Hat, hat2: &Hat, hat3: &Hat) {
    for hat in [hat1, hat2, hat3] {
        let largeness = if hat.size < 3 {
            "small"
        } else if hat.size < 9 {
            "medium"
        } else {
            "large"
        };
        println!("The {} hat is {}", largeness, hat.color());
    }
}

trait EvenOdd {
    fn is_even(&self) -> bool;
}

impl EvenOdd for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

// fn is_even(number: i32) -> bool {
//     number % 2 == 0
// }

pub fn fortune<T: Colorful>(item: T) {
    println!("The color I see in your future is: {}", item.color());
}
