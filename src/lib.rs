use std::error::Error;
use std::fmt;

type Result<T> = std::result::Result<T, UnsolvedError>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
pub struct UnsolvedError;

impl fmt::Display for UnsolvedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not solve this problem")
    }
}

impl Error for UnsolvedError {}
