use std::error;
use std::fmt;

use crate::enums;

#[derive(Debug)]
pub struct AlpmError {
    error_no: enums::ErrorNo,
}

impl AlpmError {
    pub fn new(error_no: enums::ErrorNo) -> AlpmError {
        AlpmError { error_no: error_no }
    }
}

impl fmt::Display for AlpmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "alpm_error({})", self.error_no as i32)
    }
}

// This is important for other errors to wrap this one.
impl error::Error for AlpmError {
    fn description(&self) -> &str {
        "Alpm error."
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}
