// fn diverges() -> ! {
//     panic!("This function never returns!");
// }

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    return x + 1
}

fn types() {
    // bool
    let x = true;
    let y: bool = false;

    // unicode scalar value;
    let x = 'x';
    let two_hearts = '💕';

    // char is not a single byte, but four.

    /* NUMERIC */
    let x = 42; // `x` has type `i32`.
    let y = 1.0; // `y` has type `f64`.

    // types: i8, i16, i32, i64
    // unsigned: u8, u16, u32, u64
    // float: f32, f64
    // isize, usize
    let z: isize = 10;
}

fn arrays() {
    let a = [1, 2, 3]; // a[i32; 3]
    let mut m = [1, 2, 3]; // m: [i32; 3]

    // initialize a to 0
    let a = [0; 20]; // a: [i32; 20]

    println!("a has {} elements", a.len());

    let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]

    println!("The second name is: {}", names[1]);
}

fn slices() {
    let a = [0, 1, 2, 3, 4];
    let complete = &a[..]; // A slice containing all of the elements in `a`.
    let middle = &a[1..4]; // A slice of `a`: only the elements `1`, `2`, and `3`.
}

fn tuples() {
    // a tuple
    let x = (1, "hello");
    
    let y: (i32, &str) = (1, "hello");

    let (x, y, z) = (1, 2, 3);

    println!("x is {}", x);
    (0,); // A single-element tuple.
    (0); // A zero in parentheses.

    // indexing
    let tuple = (1, 2, 3);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("x is {}", x);
}

fn fn_pointers() {
    fn foo(x: i32) -> i32 { x }

    let x: fn(i32) -> i32 = foo;
}

fn misc() {
    let x: i32 = 8;
    {
        println!("{}", x); // Prints "8".
        let x = 12;
        println!("{}", x); // Prints "12".
    }
    println!("{}", x); // Prints "8".
    let x = 42;
    println!("{}", x); // Prints "42".

    println!("{}", add_one(3));
    // let x: String = diverges();

    // function pointers
    let f: fn(i32) -> i32 = add_one;
    println!("{}", f(2));

    // function pointers with type inference
    let g = add_one;
    println!("{}", g(2));

}

fn loops() {

    loop {
        println!("Forever loop");
        break;
    }

    let mut x = 5; // mut x: i32
    let mut done = false; // mut done: bool
    
    while !done {
        x += x - 3;
        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }
    
    for x in 0..10 {
        println!("{}", x); // x: i32
    }
    
    // enumerate
    for (index, value) in (5..10).enumerate() {
        println!("index = {} and value = {}", index, value);
    }

    // iterators
    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

    // loop labels
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
            if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
            println!("x: {}, y: {}", x, y);
        }
    }

}

fn if_statements() {
    let x = 5;

    let y = if x == 5 {
        10
    } else {
        15
    }; // y: i32

    println!("x = {}", x);
    
    //also
    let y = if x == 5 { 10 } else { 15 }; // y: i32
}

fn vectors() {
    let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
    // let v = vec![0; 10]; // A vector of ten zeroes.
     
    println!("The third element of v is {}", v[2]);

    // this will panic
    // let v = vec![1, 2, 3];
    // println!("Item 7 is {}", v[7]);

    // without panicking
    let v = vec![1, 2, 3];
    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }
}

fn ownership() {
    let mut v = vec![1, 2, 3];
    v.push(9);
    println!("{}", v[3]);

    let v = 2;

    let v2 = v;

    println!("v is: {}", v);

    struct Foo<'a> {
        x: &'a i32,
    }

    let y = &5; // This is the same as `let _y = 5; let y = &_y;`.
    let f = Foo { x: y };

    println!("{}", f.x);
}

fn greet_user(user: &str) {
    println!("Hello {}!", user);
}

struct Point {
    x: i32,
    y: i32,
}

fn structsx() {
    let mut origin = Point { x: 0, y: 0 }; // origin: Point

    origin.x = 11;

    // origin is now immutable
    let origin = origin;

    // origin.x = 6; // error

    println!("The origin is at ({}, {})", origin.x, origin.y);

    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let mut point = Point3D { x: 0, y: 0, z: 0 };
    point = Point3D {y: 1, .. point};

    let point2 = Point3D { x: 10, ..point };
}

fn tuplestructs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let black_r = black.0;
    let Point(_, origin_y, origin_z) = origin;

    struct Inches(i32);

    let length = Inches(10);
    
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);
    // the same as above
    let integer_length = length.0;
}

fn matches() {
    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        6 => println!("six"),
        _ => println!("else"),
    }
    let number = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "something else",
    };
    
    let x = 1;

    match x {
        y => println!("x: {} y: {}", x, y),
    }

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, y } => println!("({},{})", x, y),
    }
}

fn patterns() {
    let tuple: (u32, String) = (5, String::from("five"));

    // Here, tuple is moved, because the String moved:
    let (x, _s) = tuple;

    // The next line would give "error: use of partially moved value: `tuple`".
    // println!("Tuple is: {:?}", tuple);

    // However,

    let tuple = (5, String::from("five"));

    // Here, tuple is _not_ moved, as the String was never moved, and u32 is Copy:
    let (x, _) = tuple;

    // That means this works:
    println!("Tuple is: {:?}", tuple);

    enum OptionalTuple {
        Value(i32, i32, i32),
        Missing,
    }

    let x = OptionalTuple::Value(5, -2, 3);

    match x {
        OptionalTuple::Value(..) => println!("Got a tuple!"),
        OptionalTuple::Missing => println!("No such luck."),
    }

    let mut x = 5;

    match x {
        ref mut mr => { println!("Got a mutable reference to {}", mr); *mr = 6; }
    }
    println!("x == {}", x);


    let x = 1;

    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("anything"),
    }
    let x = '💅';

    match x {
        'a' ... 'j' => println!("early letter"),
        'k' ... 'z' => println!("late letter"),
        _ => println!("something else"),
    }
    let x = 1;

    match x {
        e @ 1 ... 5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(5);

    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }
}

fn associated() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle {
                x: x,
                y: y,
                radius: radius,
            }
        }
    }

    let c = Circle::new(0.0, 0.0, 2.0);
}

fn generics() {
    let x: Option<i32> = Some(5);
    let y: Option<f64> = Some(5.0f64);
    println!("{:?}", x);
    //
    fn takes_anything<T>(x: T) {
    }
    fn takes_two_same_things<T>(x: T, y: T) {
        //
    }

    fn takes_two_things<T, U>(x: T, y: U) {
    }

    // generic structs
    struct Point<T> {
        x: T,
        y: T,
    }
    let int_origin = Point { x: 0, y: 0 };
    let float_origin = Point { x: 0.0, y: 0.0 };

    impl<T> Point<T> {
        fn swap(&mut self) {
            std::mem::swap(&mut self.x, &mut self.y);
        }
    }
}

fn traits() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    trait HasArea {
        fn area(&self) -> f64;

        fn is_larger(&self, &Self) -> bool;
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }

        fn is_larger(&self, other: &Self) -> bool {
            self.area() > other.area()
        }
    }
}

pub fn main() {
    // misc();
    // types();
    // arrays();
    // tuples();
    // if_statements();
    //
    // loops();
    // vectors();
    //ownership();

    //structsx();
    // matches();
    // patterns();
    // associated();
    generics();
    // greet_user("Phil");
}
