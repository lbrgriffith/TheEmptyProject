use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let iter_args = &args;

            for item in iter_args {
                println!("{:?}", item);
     // Create file in write-only mode, returns `io::Result<File>`
                match File::create(item) {
                    Err(why) => panic!("couldn't create {}: {}", item, why),
                    Ok(file) => file,
                };
            }

    }