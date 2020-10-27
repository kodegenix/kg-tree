use kg_diag::Severity;
//FIXME (jc)
#[derive(Debug, Display, Detail)]
pub enum Error {
    #[display(fmt = "SerializationError : {_0}")]
    SerializationError(u32),
    #[display(fmt = "DeserializationError : {_0}")]
    DeserializationError(u32),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        unimplemented!()
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        unimplemented!()
    }
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        println!("{}", msg);
        Error::SerializationError(line!())
    }
}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        println!("{}", msg);
        Error::DeserializationError(line!())
    }
}

pub type Result<T> = std::result::Result<T, self::Error>;
