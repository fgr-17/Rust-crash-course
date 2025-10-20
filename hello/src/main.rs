const AGE: i32 = 39; // inline at compile time, can live outside functions, SCREAMING_SNAKE_CASE is used for constants, global variables

mod excercise_traits;
use excercise_traits::{Colorful, Hat, describe_three_hats, fortune};
use hello::DispenserItem::*;
use hello::Option::*;

fn main() {
    let _bunnies = 2; // not explicitly typed, _ is used to indicate that the variable is not used
    let _bunnies: i32 = 2; // explicitly typed

    let (bunnies, carrots) = (2, 3); // tuple pattern matching to initialize variables
    println!("Bunnies: {bunnies}, Carrots: {carrots}");

    // Rust doesn't let you change variables by default: safety, concurrency, speed
    let mut name = "Fede";
    println!("Hello, {name}!");

    name = "Juan";
    println!("Hello, {name}!");
    println!("You are {AGE} years old!");

    println!("scope is the region of code where a variable is valid");
    println!(
        "shadowing is when a variable is redeclared in the same scope, the inner variable shadows the outer variable"
    );
    let x = 5;
    {
        let x = 99;
        println!("inner x is {x}");
    }
    println!("outer x is {x}");

    {
        println!("shadowing in the same scope, changing mutability");
        let y = 10;
        println!("first y is {y}");
        let mut y = 12;
        println!("second y is {y}");
        y = 114;
        println!("third y is {y}");
    }

    {
        println!("shadowing in the same scope, changing type");
        let meme = "More cowbell";
        println!("meme is {meme}");
        let meme = 14;
        println!("meme is {meme}");
    }

    // using println! macro, fn does not support variable numbers of arguments or types
    // a macro name always ends with !
    println!("do_stuff result is {}", do_stuff(1.0, 2.0));

    // --- Modules ---
    println!("--- Modules ---");
    hello::greet(); // absolute path
    use hello::greet; // relative path
    greet();

    // -- Primitive Types ---
    println!("--- Primitive Types ---");
    let _x: i32 = 5; // i32 is normally used by default
    let _y: i32 = 6;

    // Some types are not available on some processors
    let _my_hex: u32 = 0xdeadbeef;
    let _my_octal: u32 = 0o1234_1234; // _ is used to separate the digits, underscores are ignored
    let _my_binary: u32 = 0b10101010;
    let _my_byte = b'A';

    // using types as suffixes
    let _x_suffix = 5_u16;
    let _y_suffix = 3.114_f32;

    // -- Compound Types ---
    // Tuples
    let info = (1, 23.2, 123);
    println!("info is {info:?}");

    let info: (u8, f64, u8) = (1, 23.2, 123);
    println!("info is {info:?}");
    let first = info.0;
    let second = info.1;
    let third = info.2;
    println!("first is {first}, second is {second}, third is {third}");

    // destructuring tuples (arity of 4, max 12)
    let (jets, fuel, ammo) = info;
    println!("jets is {jets}, fuel is {fuel}, ammo is {ammo}");

    // Arrays: multiple values of the same type, max size is 32
    // arrays live on the stack and are fixed size
    let _buf = [1, 2, 3];
    // define the type explictly
    let _buf: [i32; 3] = [1, 2, 3];
    // repeated values
    let _buf = [0; 3];

    println!("----- Conditions: -----");
    // rust doesn't use parenthesis, conditions needs to be boolean
    // rust doesn't do type coercion to bool

    let num = 24;
    let msg = if num == 5 {
        "five"
    } else if num == 4 {
        "four"
    } else {
        "other"
    }; // needed bc I'm using the output of the if expression

    println!("msg {msg}");

    // Loops
    println!("----- Loops: -----");
    // unconditional loops can be named to refer using break
    // let mut counter = 0;
    // 'bob: loop {
    //     println!("Inside outer loop");
    //     counter += 1;
    //     loop {
    //         println!("Inside inner loop");
    //         if counter == 1 {
    //             break 'bob;
    //         }
    //     }
    // }

    let mut counter = 0;
    while counter < 4 {
        println!("counter: {counter}");
        counter += 1;
    }
    counter = 0;
    loop {
        if counter >= 4 {
            break;
        }
        println!("counter: {counter}");
        counter += 1;
    }

    // for loops:
    for num in [1, 3, 4].iter() {
        println!("num {num}");
    }

    let array = [(1, 2), (3, 4)];
    for (x, y) in array.iter() {
        println!("x: {x}, y: {y}");
    }

    println!("range for");
    for num in 0..5 {
        println!("num: {num}");
    }

    println!("----- Strings: -----");
    // 6 types of strings in std lib
    // string slice == borrowed string slice -> data cannot be modified
    // String with capital S -> data can be modified, under the hood: string slice (ptr + len) + capacity
    // stack: ptr, len, capacity - ptr points to heap

    // create String from string slice
    let _msg = "holaa".to_string();
    let _msg = String::from("holaa");

    // indexing on std containers are always constant time operations!
    // Strings cannot be indexed, because they could store unicode
    // word.bytes() can index by bytes
    // word.chars() can index by character

    println!("----- Ownership: -----");
    println!("rules:");
    println!("1. each value has an owner");
    println!("2. only one owner for values");
    println!("3. values gets dropped if owner is out of scope");

    let s1 = String::from("abc");
    let s2 = s1; // NOT a shallow copy, moved (borrewd) to the 2nd

    // println!("{}", s1); // !! this fails to compile

    let s1 = s2.clone(); // deep copy: makes a copy of the heap memory
    // rust reserves the term "copy" only for when stack memory is involved

    println!("s1: {}, s2: {}", s1, s2);
    // dropping the value
    // 1. destructor:
    // 2. free heap
    // 3. pop stack

    // not desirable pattern:
    let mut ss1 = String::from("abc");
    println!("ss1 {}", ss1);
    ss1 = my_fun(ss1);
    println!("ss1 {}", ss1); // should fail if function doesn't return value

    let mut ss2 = String::from("fede");
    // passing a stack ptr: reference to ss2 data in stack
    // lifetime: reference must always be valid
    // refs are unmutable by def, but rust allows &mut
    my_fun_ref(&mut ss2);
    println!("ss2 {}", ss2); // should fail if not ref

    // References rule:
    // either:
    // 1. exactly one mut ref
    // 2. any number of inmutable refs

    println!("----- Structs: -----");
    hello::my_fun_struct();

    println!("----- Excercise traits: -----");
    let small_hat = Hat { size: 2 };
    let medium_hat = Hat { size: 7 };
    let large_hat = Hat { size: 100 };
    describe_three_hats(&small_hat, &medium_hat, &large_hat);

    println!("4 is {}", 4.color());
    println!("5 is {}", 5.color());

    fortune(small_hat);
    fortune(2);

    println!("----- Collections: -----");
    // let mut v: Vec<i32> = Vec::new();
    // v.push(2);
    // v.push(4);
    // v.push(5);
    let mut v = vec![2, 4, 6];
    let _x = v.pop();
    println!("{}", v[1]);

    // let mut h: HashMap<u8, bool> = HashMap::new();
    // h.insert(5, true);
    // h.insert(6, false);
    // println!("HashMap {}", h);

    let _item = Empty;

    // let _var: hello::Option<i32> = None;
    let _var: hello::Option<i32> = Some(32);

    match _var {
        Some(x) => println!("Some {}", x),
        None => println!("None"),
    }
    // if let Some(x) = my_variable {
    //     println!("value is {}", x);
    // }

    use std::fs::File;

    let res = File::open("foo");
    // let f = res.unwrap();
    match res {
        Ok(_f) => {
            println!("All good ");
        }
        Err(e) => {
            println!("All bad {}", e);
        }
    }

    // Closures
    let s = "somethin".to_string();
    // let f = || {println!("{}", s)};
    // use move to yield ownership of s
    let f = move || println!("{}", s);

    f();

    // rust
}

fn do_stuff(qty: f64, oz: f64) -> f64 {
    // return qty * oz;
    qty * oz // tail expression, last expression in a block does not end with a semicolon, returned implicitly
}

fn my_fun(s: String) -> String {
    s.to_uppercase()
}

fn my_fun_ref(s: &mut String) {
    // dot operator auto dereferences down the actual value
    s.insert_str(0, "Hi, ");
    (*s).insert_str(0, "-- "); // manually dereference
}
