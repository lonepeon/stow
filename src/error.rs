#[derive(Debug, PartialEq, Eq)]
pub struct ReadFileError {
    pub file: String,
    pub reason: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeleteFileError {
    pub file: String,
    pub reason: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CreateSymlinkError {
    pub source: String,
    pub destination: String,
    pub reason: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CreateDirectoryError {
    pub directory: String,
    pub reason: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Generic(String),
    ReadFile(ReadFileError),
    CreateDirectory(CreateDirectoryError),
    DeleteFile(DeleteFileError),
    ParentDirectory(String),
    PackageNotFound(String),
    CreateSymlink(CreateSymlinkError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Generic(msg) => {
                write!(f, "{}", msg)
            }
            Self::CreateDirectory(err) => {
                write!(
                    f,
                    "directory {} cannot be created: {}",
                    err.directory, err.reason
                )
            }
            Self::DeleteFile(err) => {
                write!(f, "file {} cannot be removed: {}", err.file, err.reason)
            }
            Self::ParentDirectory(directory) => {
                write!(
                    f,
                    "parent directory of {} is not a valid directory",
                    directory
                )
            }
            Self::PackageNotFound(package) => {
                write!(f, "package {} does not exist", package)
            }
            Self::CreateSymlink(err) => {
                write!(
                    f,
                    "cannot link from {} to {}: {}",
                    err.source, err.destination, err.reason
                )
            }
            Self::ReadFile(err) => {
                write!(
                    f,
                    "cannot read file or directory {}: {}",
                    err.file, err.reason
                )
            }
        }
    }
}

impl std::error::Error for Error {}
