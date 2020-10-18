use dotenv::Error as DotenvError;
use serenity::Error as SerenityError;
use sqlx::migrate::MigrateError;
use sqlx::Error as SqlxError;
use std::env::VarError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    // Trivia errors
    Trivia(String),
    // Crate errors
    Dotenv(DotenvError),
    Io(IoError),
    Serenity(SerenityError),
    Sqlx(SqlxError),
    Migrate(MigrateError),
    Var(VarError),
}

impl From<SerenityError> for Error {
    fn from(err: SerenityError) -> Error {
        Error::Serenity(err)
    }
}

impl From<DotenvError> for Error {
    fn from(err: DotenvError) -> Error {
        Error::Dotenv(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Error {
        Error::Var(err)
    }
}

impl From<SqlxError> for Error {
    fn from(err: SqlxError) -> Error {
        Error::Sqlx(err)
    }
}

impl From<MigrateError> for Error {
    fn from(err: MigrateError) -> Error {
        Error::Migrate(err)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            // Sushii
            Error::Trivia(ref inner) => inner.fmt(f),
            // Crates
            Error::Dotenv(ref inner) => inner.fmt(f),
            Error::Io(ref inner) => inner.fmt(f),
            Error::Serenity(ref inner) => inner.fmt(f),
            Error::Sqlx(ref inner) => inner.fmt(f),
            Error::Migrate(ref inner) => inner.fmt(f),
            Error::Var(ref inner) => inner.fmt(f),
        }
    }
}
