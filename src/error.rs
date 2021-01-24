#[derive(Debug)]
pub enum AppError {
    Sqlx(sqlx::Error),
    Reqwest(reqwest::Error),
    Io(std::io::Error),
}

pub(crate) fn custom_error(message: &str) -> AppError {
    let err = std::io::Error::new(std::io::ErrorKind::Other, message);
    AppError::Io(err)
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Sqlx(ref e) => e.fmt(f),
            AppError::Reqwest(ref e) => e.fmt(f),
            AppError::Io(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            AppError::Sqlx(ref e) => Some(e),
            AppError::Reqwest(ref e) => Some(e),
            AppError::Io(ref e) => Some(e),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> AppError {
        Self::Sqlx(e)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> AppError {
        Self::Reqwest(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError {
        Self::Io(e)
    }
}
