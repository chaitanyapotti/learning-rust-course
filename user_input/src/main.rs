use std::io;

/// Crate comment. What is this module trying to achieve


fn main() {

    //! # Main function
    //! 
    //! ```
    //! fn main()
    //! ```
    //! 
    //! Reads user input and prints it to the console
    let mut input:String = String::new();
    // Print a message to the user
    println!("Say Something!");
    /*
        Read input from user
        and respond accordingly
    */
    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("You Said {}", input),
        Err(e) => println!("Something went wrong {}", e)
    }
}