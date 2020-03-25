use std::{env, io};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let dir = match args.len() {
        1 => get_current_dir().unwrap_or_else(|error| {
            panic!("{}", error);
        }),
        _ => args.remove(1),
    };

    println!("Dir: {}", dir)
}

fn get_current_dir() -> Result<String, io::Error> {
    let dir = env::current_dir()?;
    let dir_str = dir
        .to_str()
        .ok_or(io::Error::from(io::ErrorKind::InvalidData))?;
    Ok(String::from(dir_str))
}
