use std::{env, process, fmt, error};
use std::env::{args, Args};
use std::path::{Path, PathBuf, StripPrefixError};
use std::str::Chars;
use std::num::ParseIntError;

struct PathAndLength {
    path: PathBuf,
    length: usize,
}

#[derive(Debug)]
enum Errors {
    InvalidNumberOfArgumentsError(usize),
    CausedByIntParsing(std::num::ParseIntError),
    CausedByStripPrefix(std::path::StripPrefixError)
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Errors::InvalidNumberOfArgumentsError(num) => write!(f, "Exactly 2 argument are expected, got: {}", num),
            Errors::CausedByIntParsing(ref err) => write!(f, "{}", err),
            Errors::CausedByStripPrefix(ref err) => write!(f, "{}", err)
        }
    }
}

impl error::Error for Errors {}

impl From<std::path::StripPrefixError> for Errors {
    fn from(err: StripPrefixError) -> Self {
        Errors::CausedByStripPrefix(err);
    }
}

impl From<std::num::ParseIntError> for Errors {
    fn from(err: ParseIntError) -> Self {
        Errors::CausedByIntParsing(err);
    }
}

fn parseArgs() -> Result<PathAndLength, Errors> {
    let mut args: Args = args();
    if args.len() != 3 {
        return Err(Errors::InvalidNumberOfArgumentsError(args.len()));
    }
    // skipping application binary:
    args.next();
    match args.next() {
        None => return Err(Errors::InvalidNumberOfArgumentsError(1)),
        Some(pathArg) => {
            match args.next() {
                None => return Err(Errors::InvalidNumberOfArgumentsError(2)),
                Some(lenStr) => {
                    return usize::from_str_radix(lenStr.as_str(), 10)
//                        .map_err(|e| Errors::ArgumentNumberParsingError(e))
                        .map(|len| PathAndLength {
                            path: PathBuf::from(pathArg),
                            length: len
                        })
                }
            }
        }
    }
}

fn totalLength(path: PathBuf) -> usize {
    let mut totalLength: usize = 0;
    for elem in path.iter() {
        println!("3: {}", elem.to_str().expect("expected!"));
        totalLength += 1;
        totalLength += elem.len();
    }
    return totalLength;
}

fn main() {
    parseArgs()
        .and_then(|pnl|
            match dirs::home_dir() {
                None => {
                    return Ok(pnl);
                }
                Some(home_dir) => {
                    if pnl.path.starts_with(home_dir) {
                        let strippedHome: PathBuf = pnl.path.strip_prefix(home_dir)?.to_path_buf();
                        println!("1: {}", strippedHome.to_str().expect("expected!"));
                        let tildaPath: PathBuf = PathBuf::from("~").join(strippedHome);
                        println!("2: {}", tildaPath.to_str().expect("expected!"));
                        return Ok(PathAndLength {
                            path: tildaPath,
                            length: pnl.length,
                        })
                    } else {
                        return Ok(pnl);
                    }
                }
            })
        .map(|pnl| {
            // there will be a separator in the end:
            let pathLength: usize = pnl.length - 1;
            let mut totalLength: usize = totalLength(pnl.path);
            let mut path: String = String::with_capacity(pnl.length);
            let mut index: usize = 0;
            for elem in pnl.path.iter() {
                println!("4: {}", path);
                if index > 0 {
                    path.push('/');
                }
                // might be empty string?
                match elem.to_str() {
                    None => continue,
                    Some(e) => {
                        if totalLength > pathLength {
                            // have to abbreviate:
                            let mut chars: Chars = e.chars();
                            match chars.next() {
                                None => continue,
                                Some(c) => {
                                    path.push(c);
                                    // reduce total length by the element length:
                                    totalLength -= e.len();
                                    // add single char back:
                                    totalLength += 1;
                                    if c == '.' {
                                        // need a second char if we had a '.':
                                        match chars.next() {
                                            None => continue,
                                            Some(c2) => {
                                                path.push(c2);
                                                totalLength += 1;
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
            // fill the rest with spaces if needed:
            while path.len() < pathLength {
                path.push(' ');
            }
            // trim if longer:
            // TODO this seems to be highly inefficient:
            while path.len() > pathLength {
                path.remove(0);
            }
            path.push('⠿');
            println!("{}", path);
        }
        );
}
