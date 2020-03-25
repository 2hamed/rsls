use std::{env, error, io};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let dir = if args.len() == 1 {
        getCurrentDir().unwrap_or_else(|error| {
            panic!("{}", error);
        })
    } else {
        args.remove(1)
    };

    println!("Dir: {}", dir)
}

fn getCurrentDir() -> Result<String, io::Error> {
    let dir = env::current_dir()?;
    let dirStr = dir
        .to_str()
        .ok_or(io::Error::from(io::ErrorKind::InvalidData))?;
    Ok(String::from(dirStr))
}
