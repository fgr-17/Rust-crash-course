pub fn greet() {
    println!("Hello, world!");
}

// no struct inheritance
struct RedFox {
    // capital camel case
    enemy: bool,
    life: u8,
}

// implementation block (can use RedFox instead of Self)
impl RedFox {
    fn new() -> Self {
        Self {
            enemy: false,
            life: 100,
        }
    }
}

pub fn my_fun_struct() {
    let fox = RedFox {
        // need to specify a value for every single field
        enemy: true,
        life: 60,
    };

    print_fox(&fox);
    let other_fox = RedFox::new();

    print_fox(&other_fox);
    print_noise(&other_fox);

    let robot = Robot {};
    robot.run();
}

fn print_fox(fox: &RedFox) {
    println!("fox: {}-{}", fox.enemy, fox.life);
}

// an struct has a trait if has the required members
trait Noisy {
    fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {
        "Meow!"
    }
}

fn print_noise<T: Noisy>(item: &T) {
    println!("{}", item.get_noise());
}

// special trait called Copy
// if a struct implements the copy trait, will be copied instead of move in move situations
// makes sense for small values that fits on the stack
// int, floats and bools implement copy
// if the type uses the heap, it cannot implement copy
// a given type can implement copy if the inner types only uses copy types

// Traits have inheritance
// if a struct implements a trait, it has to implement the parent too
// traits can also have def behaviors:

trait Run {
    fn run(&self) {
        println!("I'm running!");
    }
}

struct Robot {}
impl Run for Robot {}

pub enum DispenserItem {
    Empty,
    Ammo(u8),
    Things(String, i32),
    Place { x: i32, y: i32 },
}

// impl DispenserItem {
//     fn display(&self) {}
// }

pub enum Option<T> {
    Some(T),
    None,
}
