use std::fmt;

#[derive(Debug)]
pub enum Error {
    OnlyRunAsSubcommand,
    ManifestNotFound,
    UnableToParse(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OnlyRunAsSubcommand => write!(f, "Can only run as cargo subcommand"),
            Self::ManifestNotFound => write!(f, "Manifest not found"),
            Self::UnableToParse(reason) => write!(f, "Unable to parse, reason: {reason}"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
