use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let iter_args = &args;
    let mut do_not_skip_it = false;

    for item in iter_args {
        //println!("{:?}", item);

        // Ignore the first item
        if do_not_skip_it
        {
            // Create file in write-only mode, returns `io::Result<File>`
            match File::create(item) {
                Err(why) => panic!("couldn't create {}: {}", item, why),
                Ok(file) => file,
            };
        }

        // Ensures processing flags, not the executable.
        do_not_skip_it = true;
    }
}
