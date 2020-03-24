use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let dir = if args.len() == 1 {
        let cur_dir = match env::current_dir() {
            Ok(v) => match v.to_str() {
                Some(v) => String::from(v),
                None => String::from(""),
            },
            Err(_e) => String::from(""),
        };
        cur_dir
    } else {
        args.remove(1)
    };

    println!("Dir: {}", dir)
}
