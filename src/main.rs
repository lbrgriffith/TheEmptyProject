use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(x) => {
            println!("{:?}", x);

            // Create file in write-only mode, returns `io::Result<File>`
            match File::create(x) {
                Err(why) => panic!("couldn't create {}: {}", x, why),
                Ok(file) => file,
            };
        },
        None => println!("Sorry, you must specify a filename.")
    }
}