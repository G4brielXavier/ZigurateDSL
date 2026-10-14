use std::fmt;
use std::error::Error;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum ZigurateError {

    IOError(String),
    ByteConversionFailed(Utf8Error),
    SerdeError(serde_yaml::Error),

    AlreadyExists(String),
    NotFound(String),
    PropertyNotFound(String),
    UnknownFunction(String),
    PropertyLimitExceeded(String),
    TypeNotSupported(String),

    VectorTypesValueIsMixed(String),
    ArrayIndexExceeded(String),

    ParametersNotProvided(String),
    ParametersLimitExceeded(String),
    GotWrongTypeValue(String),

    EnkiFunctionNotFound(String),

    SyntaxError(String),
    UnexpectedSymbol,

    GishOptionNotFound(String),

    ProjectNotFoundOrNotExist,
    UnknownCallFunctionName(String),
    PropertiesExpected(String)

}


impl fmt::Display for ZigurateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {

            ZigurateError::IOError(msg) => write!(f, "{}", msg),
            ZigurateError::ByteConversionFailed(msg) => write!(f, "{}", msg),
            ZigurateError::UnexpectedSymbol => write!(f, "Unexpected symbol found."),
            ZigurateError::EnkiFunctionNotFound(msg) => write!(f, "{} not exist or not found", msg),
            ZigurateError::SyntaxError(msg) => write!(f, "{}", msg),
            
            ZigurateError::SerdeError(e) => write!(f, "{}", e),
            
            ZigurateError::ProjectNotFoundOrNotExist => write!(f, "Project not found or not exists"),
            ZigurateError::PropertiesExpected(e) => write!(f, "{}", e),
            ZigurateError::UnknownCallFunctionName(e) => write!(f, "{}", e),
        
            ZigurateError::GishOptionNotFound(e) => write!(f, "{}", e),

            ZigurateError::AlreadyExists(e) =>  write!(f, "{}", e),
            ZigurateError::NotFound(e) =>  write!(f, "{}", e),
            ZigurateError::PropertyLimitExceeded(e) =>  write!(f, "{}", e),
            ZigurateError::PropertyNotFound(e) =>  write!(f, "{}", e),
            ZigurateError::UnknownFunction(e) =>  write!(f, "{}", e),
            ZigurateError::TypeNotSupported(e) =>  write!(f, "{}", e),
            ZigurateError::VectorTypesValueIsMixed(e) =>  write!(f, "{}", e),
            ZigurateError::ParametersNotProvided(e) =>  write!(f, "{}", e),
            ZigurateError::ParametersLimitExceeded(e) =>  write!(f, "{}", e),
            ZigurateError::GotWrongTypeValue(e) =>  write!(f, "{}", e),
            ZigurateError::ArrayIndexExceeded(e) =>  write!(f, "{}", e),

            
            
        }
    }
}



impl Error for ZigurateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {

            ZigurateError::ByteConversionFailed(err) => Some(err),

            ZigurateError::IOError(_) => None,
            ZigurateError::UnexpectedSymbol => None,
            ZigurateError::EnkiFunctionNotFound(_) => None,
            ZigurateError::SyntaxError(_) => None,
            ZigurateError::ProjectNotFoundOrNotExist => None,
            ZigurateError::SerdeError(_) => None,
            ZigurateError::UnknownCallFunctionName(_) => None,
            ZigurateError::PropertiesExpected(_) => None,
            ZigurateError::GishOptionNotFound(_) => None,

            ZigurateError::NotFound(_) => None,
            ZigurateError::PropertyLimitExceeded(_) => None,
            ZigurateError::PropertyNotFound(_) => None,
            ZigurateError::AlreadyExists(_) => None,
            ZigurateError::UnknownFunction(_) => None,
            ZigurateError::TypeNotSupported(_) => None,
            ZigurateError::VectorTypesValueIsMixed(_) => None,
            ZigurateError::ParametersNotProvided(_) => None,
            ZigurateError::ParametersLimitExceeded(_) => None,
            ZigurateError::GotWrongTypeValue(_) => None,
            ZigurateError::ArrayIndexExceeded(_) => None,
            
        }
    }
}



impl From<Utf8Error> for ZigurateError {
    fn from(err: Utf8Error) -> Self {
        ZigurateError::ByteConversionFailed(err)
    }
}
