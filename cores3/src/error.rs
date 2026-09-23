use crate::heap::AllocationError;
use alloc::string::String;
use esp_hal::i2c::master::ConfigError;

#[derive(Debug)]
pub enum ErrorKind {
    NotInitialized,
    Timeout,
    InvalidArgument,
    Allocation(AllocationError),
    I2cConfig(ConfigError),
    Hal(String),
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "Not initialized"),
            Self::Timeout => write!(f, "Timeout"),
            Self::InvalidArgument => write!(f, "Invalid argument"),
            Self::Allocation(err) => write!(f, "{}", err),
            Self::I2cConfig(err) => write!(f, "{}", err),
            Self::Hal(msg) => write!(f, "{}", msg),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    context: Option<&'static str>,
}

impl Error {
    pub fn hal(err: impl core::fmt::Debug) -> Self {
        ErrorKind::Hal(alloc::format!("{:?}", err)).into()
    }

    pub fn with_context(mut self, context: &'static str) -> Self {
        self.context = Some(context);
        self
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl core::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.kind)?;

        if let Some(c) = self.context {
            write!(f, " ({})", c)?;
        }

        Ok(())
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            kind,
            context: None,
        }
    }
}

impl From<AllocationError> for Error {
    fn from(err: AllocationError) -> Self {
        ErrorKind::Allocation(err).into()
    }
}

impl From<ConfigError> for Error {
    fn from(err: ConfigError) -> Self {
        ErrorKind::I2cConfig(err).into()
    }
}

pub type Result<T> = core::result::Result<T, Error>;
