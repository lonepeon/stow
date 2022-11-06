#[derive(Debug, PartialEq, Eq)]
pub struct FileReadingError {
    pub package: String,
    pub reason: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LinkingError {
    pub source: String,
    pub destination: String,
    pub reason: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CreateFolderError {
    pub folder: String,
    pub reason: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Generic(String),
    ReadFile(FileReadingError),
    CreateFolder(CreateFolderError),
    ParentFolder(String),
    PackageNotFound(String),
    CreateSymlink(LinkingError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Generic(msg) => {
                write!(f, "{}", msg)
            }
            Self::CreateFolder(err) => {
                write!(f, "folder {} cannot be created: {}", err.folder, err.reason)
            }
            Self::ParentFolder(folder) => {
                write!(f, "parent folder of {} is not a valid folder", folder)
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
                    err.package, err.reason
                )
            }
        }
    }
}

impl std::error::Error for Error {}
