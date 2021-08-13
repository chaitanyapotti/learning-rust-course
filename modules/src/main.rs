// use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as Archive;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
mod archive;
mod player;

// Multiple modules are grouped into a crate
// two types - binary or library
fn main() {
    player::play_movie("snatch.mp4");
    player::play_audio("rhcp.mp3");
    clean::perform_clean();
    clean::files::clean_files();

    Archive("some.txt");

    let mut rng = thread_rng();
    let a: i32 = rng.gen();
    println!("{}", a);
    println!("bounded int: {}", rng.gen_range(1..10));
    println!("bounded float: {}", rng.gen_range(1.0..100.0));

    let rand_string: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", rand_string);
}

mod clean {
    pub fn perform_clean() {
        println!("Performing clean...");
    }
    // Can also nest modules
    pub mod files {
        pub fn clean_files() {
            println!("Cleaning files...");
        }
    }
}
