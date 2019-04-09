use super::lib::{CoreErrorMessage, M64pError};

use std::convert::From;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct MupenError {
    message: String,
}

impl Display for MupenError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl Error for MupenError {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl From<M64pError> for MupenError {
    fn from(cause: M64pError) -> Self {
        unsafe {
            let err_message = CoreErrorMessage(cause.clone());
            let message = CStr::from_ptr(err_message).to_string_lossy().into_owned();
            eprintln!("{}", message);
            MupenError { message }
        }
    }
}
