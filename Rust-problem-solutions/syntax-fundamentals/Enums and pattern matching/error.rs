// 29. Create a `Error` enum with different error types and display messages.

use std::fmt;


#[derive(Debug)]

enum Error{
    FileNotFound(String),
    PermissionDenied,
    InvalidInput(String),
    Unknown,
    NetworkTimeout(u32),
}

impl fmt::Display for Error{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Error::FileNotFound(filename) => {
                write!(f,"File not found: {}" , filename)
            }
            Error::InvalidInput(reason) =>{
                write!(f, "Invalid input: {}", reason)
            }
            Error::PermissionDenied => {
                write!(f, "Permission denied")
            }
            Error::Unknown => {
                write!(f, "Unknown error occurred")
            }
            Error::NetworkTimeout(duration) => {
                write!(f, "Network timeout after {} seconds", duration)
            }
        }
    }
}

fn main() {
    let errors = vec![
        Error::FileNotFound("config.toml".to_string()),
        Error::InvalidInput("Username cannot be empty".to_string()),
        Error::PermissionDenied,
        Error::NetworkTimeout(30),
        Error::Unknown,
    ];

    for err in errors {
        println!("Error: {}", err);
    }
}
