use clap::Parser;
use rand::distributions::{Alphanumeric, Distribution};
use rand::{thread_rng, Rng};

mod utils;
pub use crate::utils::passwd_check;
pub use crate::utils::AllCharacters;

static VERSION: &str = env!("CARGO_PKG_VERSION");
static AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
static ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Parser)]
#[command(
    name = "password-generator",
    version = VERSION,
    author = AUTHOR,
    about = ABOUT
)]
struct Args {
    /// Length of the password
    #[arg(short, long, default_value_t = 8)]
    length: usize,

    /// Include special characters in the password
    #[arg(short, long)]
    special: bool,

    /// Number of passwords to generate
    #[arg(short, long, default_value_t = 20)]
    number: usize,
}

fn main() {
    let args = Args::parse();

    for i in 1..=args.number {
        let password = if args.special {
            passwd_generator(args.length, &AllCharacters, true)
        } else {
            passwd_generator(args.length, &Alphanumeric, false)
        };

        print!("{}", password);

        if i % 5 == 0 {
            println!();
        } else {
            print!("\t");
        }
    }

    // Ensure the output ends with a newline
    if args.number % 5 != 0 {
        println!();
    }
}

fn passwd_generator<T: Distribution<u8>>(length: usize, dister: &T, special_char: bool) -> String {
    loop {
        let password = passwd_create(length, dister);
        if passwd_check(&password, special_char) {
            return password;
        }
    }
}

fn passwd_create<T: Distribution<u8>>(length: usize, dister: &T) -> String {
    let mut rng = thread_rng();
    (&mut rng)
        .sample_iter(dister)
        .take(length)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Alphanumeric;

    #[test]
    fn test_password_len() {
        assert_eq!(10, passwd_create(10, &Alphanumeric).len());
        assert_eq!(1, passwd_create(1, &Alphanumeric).len());
        assert_eq!(20, passwd_create(20, &Alphanumeric).len());
        assert_eq!(10, passwd_create(10, &AllCharacters).len());
        assert_eq!(1, passwd_create(1, &AllCharacters).len());
        assert_eq!(20, passwd_create(20, &AllCharacters).len());
    }

    #[test]
    fn test_password_function() {
        let s1: String = passwd_create(8, &Alphanumeric);
        let s2: String = passwd_create(8, &AllCharacters);
        assert!(s1.is_ascii());
        assert!(s2.is_ascii());
    }
}
