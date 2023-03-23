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

#[cfg(test)]
mod tests {
    #[test]
    fn display_generic_error() {
        let display = format!("{}", super::Error::Generic("my error".to_string()));
        assert_eq!("my error", display)
    }

    #[test]
    fn create_directory_error() {
        let err = super::Error::CreateDirectory(super::CreateDirectoryError {
            directory: "/folder/".to_string(),
            reason: "permission denied".to_string(),
        });

        assert_eq!(
            "directory /folder/ cannot be created: permission denied",
            format!("{}", err)
        )
    }

    #[test]
    fn delete_file_error() {
        let err = super::Error::DeleteFile(super::DeleteFileError {
            file: "/folder/file.txt".to_string(),
            reason: "permission denied".to_string(),
        });

        assert_eq!(
            "file /folder/file.txt cannot be removed: permission denied",
            format!("{}", err)
        )
    }

    #[test]
    fn parent_directory_error() {
        let err = super::Error::ParentDirectory("/folder".to_string());

        assert_eq!(
            "parent directory of /folder is not a valid directory",
            format!("{}", err)
        )
    }

    #[test]
    fn package_not_found_error() {
        let err = super::Error::PackageNotFound("my-package".to_string());

        assert_eq!("package my-package does not exist", format!("{}", err))
    }

    #[test]
    fn create_symlink_error() {
        let err = super::Error::CreateSymlink(super::CreateSymlinkError {
            source: "/source/file.txt".to_string(),
            destination: "/dest/file.txt".to_string(),
            reason: "permission denied".to_string(),
        });

        assert_eq!(
            "cannot link from /source/file.txt to /dest/file.txt: permission denied",
            format!("{}", err)
        )
    }

    #[test]
    fn read_file_error() {
        let err = super::Error::ReadFile(super::ReadFileError {
            file: "/folder/file.txt".to_string(),
            reason: "permission denied".to_string(),
        });

        assert_eq!(
            "cannot read file or directory /folder/file.txt: permission denied",
            format!("{}", err)
        )
    }
}
