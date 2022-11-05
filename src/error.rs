#[derive(Debug, PartialEq, Eq)]
pub struct FileReadingError {
    pub package: String,
    pub reason: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LinkingError {
    pub source: String,
    pub destination: String,
    pub reason: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    FileReading(FileReadingError),
    PackageNotFound(String),
    Linking(LinkingError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::PackageNotFound(package) => {
                write!(f, "package {} does not exist", package)
            }
            Self::Linking(err) => {
                write!(
                    f,
                    "cannot link from {} to {}: {}",
                    err.source, err.destination, err.reason
                )
            }
            Self::FileReading(err) => {
                write!(
                    f,
                    "cannot read file or directory {}: {}",
                    err.package, err.reason
                )
            }
        }
    }
}

impl std::error::Error for Error {}
