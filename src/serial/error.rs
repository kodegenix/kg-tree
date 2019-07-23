//FIXME (jc)
#[derive(Debug)]
pub enum Error {
    SerializationError(u32),
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

impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
