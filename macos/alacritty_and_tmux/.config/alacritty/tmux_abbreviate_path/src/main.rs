use std::env::{Args, args};
use std::error::Error;
use std::process;
use std::path::{PathBuf, Path};
use std::str::Chars;

fn main() {
    let mut args: Args = args();
    if args.len() != 3 {
        eprintln!("Exactly 2 argument are expected, got: {}", args.len());
        process::exit(1);
    }
    let input_path: String = args.nth(1).unwrap();
    // .nth() is scrolling, so next one will be 0 again:
    let target_length: String = args.nth(0).unwrap();
    let abbreviate_res = abbreviate_to_length(input_path, target_length);
    match abbreviate_res {
        Ok(abbrv) => println!("{}", abbrv),
        Err(err) => eprintln!("Failure: {}", err)
    }
}

fn abbreviate_to_length(input_path_str: String, input_length: String) -> Result<String, Box<dyn Error>> {
    // there will be a separator in the end:
    let target_length: usize = usize::from_str_radix(input_length.as_str(), 10)? - 1;
    let input_path: PathBuf = PathBuf::from(input_path_str);
    let user_home: PathBuf = match dirs::home_dir() {
        Some(h) => h,
        None => PathBuf::new()
    };
    let path_from_home = if input_path.starts_with(user_home.as_path()) {
        let path_stripped_home: PathBuf = input_path.strip_prefix(user_home)?.to_path_buf();
        let path_tilda: PathBuf = PathBuf::from("~").join(path_stripped_home);
        path_tilda
    } else {
        input_path.to_path_buf()
    };
    let (count, mut total_length) = count_and_total_length(path_from_home.as_path());
    let mut path: String = String::with_capacity(target_length * 2);
    let mut index: usize = 0;
    for elem in path_from_home.iter() {
        // might be empty string?
        match elem.to_str() {
            None => {
                index += 1;
            }
            Some(e) => {
                if index > 0 {
                    path.push('/');
                }
                index += 1;
                // do not abbreviate last element:
                if index == count {
                    path.push_str(e);
                } else if total_length > target_length {
                    // have to abbreviate:
                    let mut chars: Chars = e.chars();
                    match chars.next() {
                        None => continue,
                        Some(c) => {
                            path.push(c);
                            // reduce total length by the element length:
                            total_length -= e.len();
                            // add single char back:
                            total_length += 1;
                            if c == '.' {
                                // need a second char if we had a '.':
                                match chars.next() {
                                    None => continue,
                                    Some(c2) => {
                                        path.push(c2);
                                        total_length += 1;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // enough space, no need to abbreviate:
                    path.push_str(e);
                }
            }
        }
    }
    // trim if longer:
    if path.len() > target_length {
        let overflow: usize = path.len() - target_length + 1;
        path.drain(..overflow);
        path.insert(0, '…');
    }
    // fill the rest with spaces if needed:
    while path.len() < target_length {
        path.push(' ');
    }
    path.push('⠿');
    return Ok(path);
}

fn count_and_total_length(path: &Path) -> (usize, usize) {
    let mut count: usize = 0;
    let mut total_length: usize = 0;
    for elem in path.iter() {
        count += 1;
        total_length += 1;
        total_length += elem.len();
    }
    return (count, total_length);
}
