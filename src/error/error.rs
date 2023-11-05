use std::fmt::{self};
use std::error::Error;

#[derive(Debug)]
pub enum XTTError {
    NoConfigFile,
    NoRepoConfigured,
    PasswordNotConfigured,
    CloningError,
    CannotCreateSymlink,
    DataDirNotFound,
    NonUTF8Path,
    NonExistingTemplate
}

impl fmt::Display for XTTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoConfigFile => write!(f, "No configuration file"),
            Self::NoRepoConfigured => write!(f, "No repo configured"),
            Self::CloningError => write!(f, "There was a problem clonning the repo"),
            Self::PasswordNotConfigured => write!(f, "Password is not configured"),
            Self::CannotCreateSymlink => write!(f, "Cannot create symlink"),
            Self::DataDirNotFound => write!(f, "Data dir not found"),
            Self::NonUTF8Path => write!(f, "Non UTF8-valid path"),
            Self::NonExistingTemplate => write!(f, "Non existing template")
        }
    }
}

impl Error for XTTError {}
