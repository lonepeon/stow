use crate::{path, CreateFolderError, Error, FileReadingError, LinkingError};

pub trait Linker {
    fn create_symlink(
        &mut self,
        source: &path::Source,
        destination: &path::Destination,
    ) -> Result<(), Error>;

    fn create_folder(&mut self, folder: &std::path::Path) -> Result<(), Error>;

    fn folder_exists(&mut self, folder: &std::path::Path) -> Result<bool, Error>;

    fn read_link(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error>;
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

    fn read_link(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error> {
        Ok(file.to_path_buf())
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
        writeln!(self.logger, "ln -s {} {}", source, destination).map_err(|e| {
            Error::CreateSymlink(LinkingError {
                source: format!("{}", source),
                destination: format!("{}", destination),
                reason: e.to_string(),
            })
        })?;

        self.linker.create_symlink(source, destination)
    }

    fn create_folder(&mut self, folder: &std::path::Path) -> Result<(), Error> {
        writeln!(self.logger, "mkdir -p {}", folder.display()).map_err(|e| {
            Error::CreateFolder(CreateFolderError {
                folder: format!("{}", folder.display()),
                reason: e.to_string(),
            })
        })?;

        self.linker.create_folder(folder)
    }

    fn folder_exists(&mut self, folder: &std::path::Path) -> Result<bool, Error> {
        self.linker.folder_exists(folder)
    }

    fn read_link(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error> {
        writeln!(self.logger, "readlink {}", file.display()).map_err(|e| {
            Error::ReadFile(FileReadingError {
                package: file.display().to_string(),
                reason: e.to_string(),
            })
        })?;

        self.linker.read_link(file)
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
            Error::CreateSymlink(LinkingError {
                source: format!("{}", source),
                destination: format!("{}", destination),
                reason: e.to_string(),
            })
        })
    }

    fn create_folder(&mut self, folder: &std::path::Path) -> Result<(), Error> {
        std::fs::create_dir_all(folder).map_err(|e| {
            Error::CreateFolder(CreateFolderError {
                folder: format!("{}", folder.display()),
                reason: e.to_string(),
            })
        })
    }

    fn folder_exists(&mut self, folder: &std::path::Path) -> Result<bool, Error> {
        if folder.exists() && !folder.is_dir() {
            return Err(Error::ParentFolder(folder.display().to_string()));
        }

        Ok(folder.exists())
    }

    fn read_link(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error> {
        std::fs::read_link(file).map_err(|e| {
            Error::ReadFile(FileReadingError {
                package: file.display().to_string(),
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
