use crate::heap::AllocationError;

macro_rules! impl_from_error {
    ($($variant:ident($error_type:ty)),* $(,)?) => {
        $(
            impl From<$error_type> for Error {
                fn from(err: $error_type) -> Self {
                    Self::$variant(err)
                }
            }

            impl From<$error_type> for Error_ {
                fn from(err: $error_type) -> Self {
                    Error::from(err).into()
                }
            }
        )*
    };
}

#[derive(Debug)]
pub enum Error {
    NotInitialized,
    Locked,
    AllocationError(AllocationError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "Not initialized"),
            Self::Locked => write!(f, "Locked"),
            Self::AllocationError(err) => write!(f, "{}", err),
        }
    }
}

impl_from_error! {
    AllocationError(AllocationError)
}

impl Error {
    pub fn with_context(self, context: &'static str) -> Error_ {
        let err: Error_ = self.into();
        err.with_context(context)
    }
}

#[derive(Debug)]
pub struct Error_ {
    kind: Error,
    context: Option<&'static str>,
}

impl core::error::Error for Error_ {}

impl core::fmt::Display for Error_ {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.kind)?;

        if let Some(c) = self.context {
            write!(f, " ({})", c)?;
        }

        Ok(())
    }
}

impl Error_ {
    pub fn should_retry(&self) -> bool {
        matches!(self.kind, Error::Locked)
    }

    pub fn with_context(mut self, context: &'static str) -> Self {
        self.context = Some(context);
        self
    }
}

impl From<Error> for Error_ {
    fn from(kind: Error) -> Self {
        Self {
            kind,
            context: None,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error_>;
