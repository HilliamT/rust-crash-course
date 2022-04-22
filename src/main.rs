
// Main
fn main() {

    // Prints "Hello, world!" to the console
    struct_implement_setter();
}

// Print
fn print() {

    // Prints "Hello, world!" to the console
    println!("Hello, world!");
}

fn mutable_variable() {
    let mut x = 5;
    x = x + 1;
    println!("The value of x is: {}", x);
}

fn destructure_tuple() {
    let (x, y, z) = (500, 6.4, 1);
    println!("The value of y is: {}", y);
}

fn explicit_types() {
    let x: i32 = 5;
    let y: f64 = 6.4;
    let z: bool = true;
    println!("The value of x is: {}", x);
}

fn string_manipulation() {
    let mut hello = String::from("Hello,");

    hello.push(' ');
    hello.push_str("world!");
    println!("{}", hello);
}

fn string_methods() {
    let mut hello = String::from("Hello,");
    hello.push(' ');
    hello.push_str("world!");
    println!("{}", hello);
    println!("{}", hello.len());
    println!("{}", hello.capacity());
    println!("{}", hello.is_empty());
    println!("{}", hello.contains("world"));
    println!("{}", hello.replace("world", "Rust"));
}

fn print_array() {
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);
}

fn for_loop() {
    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("{}", element);
    }
}

fn mutate_for_loop() {
    let mut a = [1, 2, 3, 4, 5];
    for element in a.iter_mut() {
        *element += 1;
    }
    println!("{:?}", a);
}

fn if_else() {
    let x = 5;
    let y = 10;

    if x == 5 {
        println!("x is five");
    } else if y == 10 {
        println!("y is ten");
    } else {
        println!("x is not five or y is not ten");
    }
}

fn break_infinite_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn add_function(x: i32, y: i32) -> i32 {
    x + y
}

fn closures() {
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("{}", add_one(1));
}

fn primitive_arrays() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = arr1;
    println!("{:?}", arr1);
    println!("{:?}", arr2);
}

fn vector_arrays() {
    let v = vec![1, 2, 3, 4, 5];
    let v2 = &v;
    println!("{:?}", &v);
    println!("{:?}", v2);
}

fn structs() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        email: String::from("email@example.com"),
        username: String::from("username"),
        active: true,
        sign_in_count: 1,
    };

    user1.sign_in_count = 2;

    println!("{}", user1.email);
}

fn tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);
}

fn struct_implementations() {
    struct User {
        username: String,
        email: String,
    }

    impl User {
        fn new(username: String, email: String) -> User {
            User {
                username,
                email,
            }
        }
    }

    let user1 = User::new(String::from("username"), String::from("email"));
    println!("{}", user1.username);
}

fn struct_implement_setter() {
    struct User {
        username: String,
        email: String,
    }

    impl User {
        fn new(username: String, email: String) -> User {
            User {
                username,
                email,
            }
        }

        fn set_email(&mut self, email: String) {
            self.email = email;
        }
    }

    let mut user1 = User::new(String::from("username"), String::from("email"));
    user1.set_email(String::from("new_email"));
    println!("{}", user1.email);
}

fn enums() {
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let dir: Direction = Direction::Up;
    match dir {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }
}