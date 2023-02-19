use std::env;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;

struct Argumnents {
    barebones: bool,
    files: Vec<String>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let iter_args = &args;
    let mut do_not_skip_it = false;
    let mut parameters =  Argumnents { barebones: false, files: Vec::new() };

    fn get_extension_from(filename: &str) -> Option<&str> {    
        Path::new(filename)        
            .extension()        
            .and_then(OsStr::to_str)
            .or(Some(""))
    }

    for item in iter_args {

        // Ignore the first item
        if do_not_skip_it
        {
            if item == "-b"
            {
                parameters.barebones = true;
            }
            else {
                parameters.files.push(item.to_string());
            }
        }

        // Ensures processing flags, not the executable.
        do_not_skip_it = true;
    }

    // What is in the Struct Vec<String>?
    for record in parameters.files.iter() {
        println!("File: {}", record);

        // Create file in write-only mode, returns `io::Result<File>`
        match File::create(record) {
            Err(why) => panic!("couldn't create {}: {}", record, why),
            Ok(file) => file,
        };

        println!("{:?}",get_extension_from(record).unwrap());
    }
}
