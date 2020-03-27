use std::os::unix::fs::PermissionsExt;
use std::{env, fs, io};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let path = match args.len() {
        1 => get_current_dir().unwrap_or_else(|error| {
            panic!("{}", error);
        }),
        _ => args.remove(1),
    };

    let abs_path = determine_abs_path(&path[..], &get_current_dir().unwrap()[..]);

    match fs::read_dir(abs_path) {
        Err(e) => panic!("{}", e),
        Ok(dir) => {
            for f in dir {
                match f {
                    Ok(entry) => println!("{:o}", entry.metadata().unwrap().permissions().mode()),
                    Err(e) => panic!("{}", e),
                }
            }
        }
    }
}

fn get_current_dir() -> Result<String, io::Error> {
    let dir = env::current_dir()?;
    let dir_str = dir
        .to_str()
        .ok_or(io::Error::from(io::ErrorKind::InvalidData))?;
    Ok(String::from(dir_str))
}

fn determine_abs_path(path: &str, current_dir: &str) -> String {
    let mut abs_path: String;
    if path.as_bytes()[0] == b'/' {
        abs_path = String::from(path);
        return abs_path;
    }

    abs_path = String::from(current_dir);
    abs_path.push_str("/");
    abs_path.push_str(path);

    abs_path
}
