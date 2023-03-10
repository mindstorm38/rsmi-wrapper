//! Module for error types definition.

use std::fmt::Display;

use rsmi_wrapper_sys::*;


pub enum RsmiError {
    LibloadingError(libloading::Error),
    FailedToLoadSymbol(String),
    InvalidUtf8,
    InvalidArgs,
    NotSupported,
    File,
    Permission,
    OutOfResources,
    Internal,
    InputOutOfBounds,
    Init,
    NotYetImplemented,
    NotFound,
    InsufficientSize,
    Interrupt,
    UnexpectedSize,
    NoData,
    UnexpectedData,
    Busy,
    RefcountOverflow,
    Unknown,
    Unexpected(rsmi_status_t),
}

impl Display for RsmiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RsmiError::*;
        match self {
            LibloadingError(err) => write!(f, "a libloading error occurred: {err}"),
            InvalidUtf8 => write!(f, "a function returned invalid utf8 encoding and the wrapper cannot convert it"),
            InvalidArgs => write!(f, "passed in arguments are not valid"),
            NotSupported => write!(f, "the requested information or action is not available 
            for the given input, on the given system"),
            File => write!(f, "problem accessing a file, this may because the operation 
            is not supported by the linux kernel version running on the executing machine"),
            Permission => write!(f, "permission denied/EACCESS file error, many functions
            require root access to run"),
            _ => todo!()
        }

    }
}

impl From<libloading::Error> for RsmiError {
    fn from(err: libloading::Error) -> Self {
        Self::LibloadingError(err)
    }
}

pub fn rsmi_try(code: rsmi_status_t) -> Result<(), RsmiError> {
    #[allow(non_upper_case_globals)]
    match code {
        rsmi_status_t_RSMI_STATUS_SUCCESS => Ok(()),
        rsmi_status_t_RSMI_STATUS_INVALID_ARGS => Err(RsmiError::InvalidArgs),
        rsmi_status_t_RSMI_STATUS_NOT_SUPPORTED => Err(RsmiError::NotSupported),
        rsmi_status_t_RSMI_STATUS_FILE_ERROR => Err(RsmiError::File),
        rsmi_status_t_RSMI_STATUS_PERMISSION => Err(RsmiError::Permission),
        rsmi_status_t_RSMI_STATUS_OUT_OF_RESOURCES => Err(RsmiError::OutOfResources),
        rsmi_status_t_RSMI_STATUS_INTERNAL_EXCEPTION => Err(RsmiError::Internal),
        rsmi_status_t_RSMI_STATUS_INPUT_OUT_OF_BOUNDS => Err(RsmiError::InputOutOfBounds),
        rsmi_status_t_RSMI_STATUS_INIT_ERROR => Err(RsmiError::Init),
        rsmi_status_t_RSMI_STATUS_NOT_YET_IMPLEMENTED => Err(RsmiError::NotYetImplemented),
        rsmi_status_t_RSMI_STATUS_NOT_FOUND => Err(RsmiError::NotFound),
        rsmi_status_t_RSMI_STATUS_INSUFFICIENT_SIZE => Err(RsmiError::InsufficientSize),
        rsmi_status_t_RSMI_STATUS_INTERRUPT => Err(RsmiError::Interrupt),
        rsmi_status_t_RSMI_STATUS_UNEXPECTED_SIZE => Err(RsmiError::UnexpectedSize),
        rsmi_status_t_RSMI_STATUS_NO_DATA => Err(RsmiError::NoData),
        rsmi_status_t_RSMI_STATUS_UNEXPECTED_DATA => Err(RsmiError::UnexpectedData),
        rsmi_status_t_RSMI_STATUS_BUSY => Err(RsmiError::Busy),
        rsmi_status_t_RSMI_STATUS_REFCOUNT_OVERFLOW => Err(RsmiError::RefcountOverflow),
        rsmi_status_t_RSMI_STATUS_UNKNOWN_ERROR => Err(RsmiError::Unknown),
        _ => Err(RsmiError::Unexpected(code))
    }
}

pub fn rsmi_sym<T: Clone>(res: &Result<T, libloading::Error>) -> Result<T, RsmiError> {
    match res {
        Ok(t) => Ok(t.clone()),
        Err(e) => Err(RsmiError::FailedToLoadSymbol(e.to_string()))
    }
}
