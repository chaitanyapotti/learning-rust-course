#[allow(unused_variables)]
#[allow(unused_assignments)]

// By default all variables are immutable. Add mut keyword to allow mutating
fn main() {
    // string slices
    let name = "Michael";
    let mut age = 32;
    println!("{}", age);
    let amount: i64 = 128934839848937;
    age = 64;
    println!("{}", age);

    // Shadowing is allowed. I hate it tho.
    let color = "blue";
    println!("{}", color);
    let color = 43;
    println!("{}", color);

    let (a, b, c) = (43, 85, "red");

    let pi: f32 = 4.0;
    let pi2 = 4.0;

    let million = 1_000_000;
    println!("{}", million);

    let smiley_face = '\u{1F601}'; // 😁
    println!("{}", smiley_face);

    // Static string slices - As a reference lifetime 'static
    // indicates that the data pointed to by the reference lives for the entire lifetime of the running program.
    // It can still be coerced to a shorter lifetime.
    let cat: &'static str = "Fluffy";
    println!("{}", cat);

    // dog is a string object
    let dog = String::new();
    let mut dog = String::from("Max");
    println!("{}", dog);

    let owner = format!("Hi I'm {}, the owner of {}", "Mark", dog);
    println!("{}", owner);
    dog.push(' ');
    dog.push_str("the dog");
    println!("{}", dog);

    // Have to delcare types for constants. Immutable. usually uppercase
    const URL: &str = "google.com";
    println!("{}", URL);

    println!("{}", 1 > 2);
    // Bitwise opperators are also supported

    println!("{}", 10 & 3);
    // << => multiply by 2 in decimal - bit shift left
    // >> => divide by 2 in decimal - bit shift right

    say_hi();

    let mut name: &str = "John";
    say_hello(name);
    name = "WIck";
    println!("{}", name);
    say_hello2(&mut name);
    println!("{}", name);

    println!("{}", say_hello3(name));
}

fn say_hi() {
    println!("Hello there!");
}

// Pass by value: borrowing the string slice
fn say_hello(name: &str) {
    println!("Hello {}", name);
}

// Pass by reference
fn say_hello2(name: &mut &str) {
    // point to the memory location of alex
    *name = "Alex";
    println!("Hello {}", name);
}

fn say_hello3(name: &str) -> String {
    let greeting = format!("Hello {}", name);
    return greeting;
    // can also just write greeting
}
