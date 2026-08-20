//! pmj_client_core__error

use std::{fmt, error};

#[derive(Debug, Clone)]
pub struct CCError {
    pub kind:CCErrKinds
}

#[derive(Debug, Clone)]
pub enum CCErrKinds {
    HandShakeError,
    Other
}

impl fmt::Display for CCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("CCError: {:?}", self.kind))
    }
}

impl error::Error for CCError {}
