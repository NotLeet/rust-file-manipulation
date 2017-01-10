use std::fs;
use std::env;
use std::io::Read;
use std::path::PathBuf;


fn main() {
    let args: Vec<_> = env::args().collect();

    let dir =  
        if args.len() > 1 {
            let ref arg_ref = args[1];
            PathBuf::from(arg_ref)
        } else {
            env::current_dir().unwrap()
        };


    match validate_dir(&dir) {
        Ok(expr) => fetch_files(&dir),
        Err(err) => {
            println!("Please ensure your directory is set up correctly and retry the program.");
        }
    }
}


fn validate_dir(dir: &PathBuf) -> Result<(), String> {
    match fs::read_dir(dir) {
        Err(err) => {
            println!("Failure: please ensure you've supplied the correct directory.");
            return Err(err.to_string())
        },
        Ok(paths) => {
            for res in paths {
                let path = res.unwrap().path();
                match path.extension() {
                    Some(extension_str) => {
                        if extension_str == "txt" {
                            continue
                        }
                        else if path.file_name().unwrap() == "read_file.exe" {
                            continue
                        }
                        else {
                            println!("Failure: Non-text items in directory.");
                            return Err("Non-text items in directory".to_string())
                        }
                    },
                    None => { 
                        println!("Failure: Non-text items in directory.");
                        return Err("Non-text items in directory".to_string())
                    },
                }
            }
            return Ok(())
        },
    }
}


fn fetch_files(dir: &PathBuf) -> Vec<PathBuf> {
    let paths = fs::read_dir(dir).unwrap();
    let mut paths_vec: Vec<PathBuf> = Vec::new();
    for res in paths {
        let path = res.unwrap();
        if path.path().extension().unwrap() == "txt" {
            paths_vec.push(path.path())
        } 
    }
    // TODO: Sort vector
    paths_vec
}


fn get_file_number(file_path: &PathBuf) -> i32 {
    4 // chosen by fair die roll
}


fn read_file(pth: PathBuf) {
    let mut data = String::new();
    let mut f = fs::File::open(pth).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    println!("{}", data);
}