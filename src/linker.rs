use crate::{path, CreateFolderError, CreateSymlinkError, DeleteFileError, Error, ReadFileError};

pub trait Linker {
    fn create_symlink(
        &mut self,
        source: &path::Source,
        destination: &path::Destination,
    ) -> Result<(), Error>;

    fn create_folder(&mut self, folder: &std::path::Path) -> Result<(), Error>;

    fn folder_exists(&mut self, folder: &std::path::Path) -> Result<bool, Error>;

    fn file_exists(&mut self, file: &std::path::Path) -> Result<bool, Error>;

    fn read_link(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error>;

    fn delete_file(&mut self, file: &std::path::Path) -> Result<(), Error>;
}

pub struct Noop;

impl Linker for Noop {
    fn create_symlink(
        &mut self,
        _source: &path::Source,
        _destination: &path::Destination,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn create_folder(&mut self, _folder: &std::path::Path) -> Result<(), Error> {
        Ok(())
    }

    fn folder_exists(&mut self, _folder: &std::path::Path) -> Result<bool, Error> {
        Ok(true)
    }

    fn file_exists(&mut self, _file: &std::path::Path) -> Result<bool, Error> {
        Ok(false)
    }

    fn read_link(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error> {
        Ok(file.to_path_buf())
    }

    fn delete_file(&mut self, _file: &std::path::Path) -> Result<(), Error> {
        Ok(())
    }
}

pub struct Verbose<W: std::io::Write, L: Linker> {
    logger: W,
    linker: L,
}
impl<W: std::io::Write, L: Linker> Verbose<W, L> {
    pub fn new(logger: W, linker: L) -> Self {
        Verbose { logger, linker }
    }
}

impl<W: std::io::Write, L: Linker> Linker for Verbose<W, L> {
    fn create_symlink(
        &mut self,
        source: &path::Source,
        destination: &path::Destination,
    ) -> Result<(), Error> {
        writeln!(self.logger, "ln -s {} {}", source, destination)
            .map_err(|e| Error::Generic(format!("cannot write ln -s log: {}", e)))?;

        self.linker.create_symlink(source, destination)
    }

    fn create_folder(&mut self, folder: &std::path::Path) -> Result<(), Error> {
        writeln!(self.logger, "mkdir -p {}", folder.display())
            .map_err(|e| Error::Generic(format!("cannot write mkdir log: {}", e)))?;

        self.linker.create_folder(folder)
    }

    fn folder_exists(&mut self, folder: &std::path::Path) -> Result<bool, Error> {
        self.linker.folder_exists(folder)
    }

    fn file_exists(&mut self, file: &std::path::Path) -> Result<bool, Error> {
        self.linker.file_exists(file)
    }

    fn read_link(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error> {
        writeln!(self.logger, "readlink {}", file.display())
            .map_err(|e| Error::Generic(format!("cannot write readlink log: {}", e)))?;

        self.linker.read_link(file)
    }

    fn delete_file(&mut self, file: &std::path::Path) -> Result<(), Error> {
        writeln!(self.logger, "readlink {}", file.display())
            .map_err(|e| Error::Generic(format!("cannot write rm log: {}", e)))?;

        self.linker.delete_file(file)
    }
}

pub struct Filesystem;

impl Linker for Filesystem {
    fn create_symlink(
        &mut self,
        source: &path::Source,
        destination: &path::Destination,
    ) -> Result<(), Error> {
        std::os::unix::fs::symlink(source, destination).map_err(|e| {
            Error::CreateSymlink(CreateSymlinkError {
                source: source.to_string(),
                destination: destination.to_string(),
                reason: e.to_string(),
            })
        })
    }

    fn create_folder(&mut self, folder: &std::path::Path) -> Result<(), Error> {
        std::fs::create_dir_all(folder).map_err(|e| {
            Error::CreateFolder(CreateFolderError {
                folder: folder.display().to_string(),
                reason: e.to_string(),
            })
        })
    }

    fn folder_exists(&mut self, folder: &std::path::Path) -> Result<bool, Error> {
        if folder.exists() && !folder.is_dir() {
            return Err(Error::Generic(format!(
                "folder {} exists but is not a folder",
                folder.display()
            )));
        }

        Ok(folder.exists())
    }

    fn file_exists(&mut self, file: &std::path::Path) -> Result<bool, Error> {
        if file.exists() && !file.is_file() {
            return Err(Error::Generic(format!(
                "file {} exists but is not a file",
                file.display(),
            )));
        }

        Ok(file.exists())
    }

    fn read_link(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error> {
        std::fs::read_link(file).map_err(|e| {
            Error::ReadFile(ReadFileError {
                file: file.display().to_string(),
                reason: e.to_string(),
            })
        })
    }

    fn delete_file(&mut self, file: &std::path::Path) -> Result<(), Error> {
        std::fs::remove_file(file).map_err(|e| {
            Error::DeleteFile(DeleteFileError {
                file: file.display().to_string(),
                reason: e.to_string(),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verbose_noop_link() {
        let mut output = std::io::BufWriter::new(Vec::new());
        let mut dryrunner = Verbose::new(&mut output, Noop);
        let source = "/from/path".into();
        let destination = "/to/path".into();
        dryrunner
            .create_symlink(&source, &destination)
            .expect("cannot link path");

        let content = String::from_utf8(output.into_inner().unwrap()).unwrap();

        assert_eq!("ln -s /from/path /to/path\n", content)
    }
}
