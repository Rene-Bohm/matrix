use std::fmt::Debug;
use std::fmt::Display;

enum MatrixCreation {
    BufferError(),
    GenericError(),
}

enum outcome<Success, ErrorType>
where
    ErrorType: Error,
{
    Ok(Success),
    Err(ErrorType),
}

trait Error: Display + Debug {
    fn source(&self) -> Option<&(dyn Error)>;

    fn backtrace(&self) -> String;
}
