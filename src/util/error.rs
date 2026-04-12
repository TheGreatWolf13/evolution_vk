#[derive(Debug)]
pub struct Throwable(Box<Inner>);

impl Throwable {
    pub fn illegal_state_exception(message: &str, cause: Option<Throwable>) -> Self {
        Throwable(Box::new(Inner {
            message: message.into(),
            cause,
            ty: ThrowableType::Exception(ExceptionType::RuntimeException(RuntimeExceptionType::IllegalStateException)),
        }))
    }
}

#[derive(Debug)]
struct Inner {
    message: String,
    ty: ThrowableType,
    cause: Option<Throwable>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ThrowableType {
    Error(ErrorType),
    Exception(ExceptionType),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ErrorType {
    DriverError,
    LinkageError,
    OSError,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ExceptionType {
    IOException(IOExceptionType),
    RuntimeException(RuntimeExceptionType),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IOExceptionType {
    FileNotFoundException,
    UnknownHostException,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RuntimeExceptionType {
    ConcurrentModificationException,
    IllegalArgumentException,
    IllegalStateException,
    UnsupportedOperationException,
}