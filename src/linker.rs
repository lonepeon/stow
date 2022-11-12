use crate::{
    path, CreateDirectoryError, CreateSymlinkError, DeleteFileError, Error, ReadFileError,
};

pub trait Linker {
    fn canonicalize(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error>;

    fn create_symlink(
        &mut self,
        source: &path::Source,
        destination: &path::Destination,
    ) -> Result<(), Error>;

    fn create_directory(&mut self, directory: &std::path::Path) -> Result<(), Error>;

    fn directory_exists(&mut self, directory: &std::path::Path) -> Result<bool, Error>;

    fn file_exists(&mut self, file: &std::path::Path) -> Result<bool, Error>;

    fn is_symlink(&mut self, file: &std::path::Path) -> bool;

    fn list_symlinks(
        &mut self,
        directory: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, Error>;

    fn read_link(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error>;

    fn delete_file(&mut self, file: &std::path::Path) -> Result<(), Error>;
}

#[derive(Default)]
pub struct Noop {
    pub directories: Vec<std::path::PathBuf>,
    pub files: Vec<(std::path::PathBuf, std::path::PathBuf)>,
}

impl Linker for Noop {
    fn canonicalize(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error> {
        Ok(file.to_path_buf())
    }

    fn create_symlink(
        &mut self,
        source: &path::Source,
        destination: &path::Destination,
    ) -> Result<(), Error> {
        if self.file_exists(destination.as_ref())? {
            return Err(Error::CreateSymlink(CreateSymlinkError {
                source: source.to_string(),
                destination: destination.to_string(),
                reason: "file already exists".to_string(),
            }));
        }

        self.files.push((
            destination.as_ref().to_path_buf(),
            source.as_ref().to_path_buf(),
        ));

        Ok(())
    }

    fn create_directory(&mut self, directory: &std::path::Path) -> Result<(), Error> {
        if self.directory_exists(directory)? {
            return Err(Error::CreateDirectory(CreateDirectoryError {
                directory: directory.display().to_string(),
                reason: "directory already exists".to_string(),
            }));
        }

        self.directories.push(directory.to_path_buf());

        Ok(())
    }

    fn directory_exists(&mut self, directory: &std::path::Path) -> Result<bool, Error> {
        Ok(self.directories.iter().any(|f| f.as_path() == directory))
    }

    fn file_exists(&mut self, file: &std::path::Path) -> Result<bool, Error> {
        Ok(self.files.iter().any(|(f, _)| f.as_path() == file))
    }

    fn is_symlink(&mut self, file: &std::path::Path) -> bool {
        self.files.iter().any(|(f, _)| f.as_path() == file)
    }

    fn list_symlinks(
        &mut self,
        directory: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, Error> {
        let symlinks = self
            .files
            .iter()
            .filter_map(|(file, _)| file.parent().map(|p| p == directory).and(Some(file)))
            .map(|p| p.to_path_buf())
            .collect::<Vec<std::path::PathBuf>>();

        Ok(symlinks)
    }

    fn read_link(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error> {
        if !self.file_exists(file)? {
            return Err(Error::ReadFile(ReadFileError {
                file: file.display().to_string(),
                reason: "file does not exist".to_string(),
            }));
        }

        let target = self
            .files
            .iter()
            .find(|(f, _)| f.as_path() == file)
            .map(|(_path, target)| target)
            .unwrap();

        Ok(target.to_path_buf())
    }

    fn delete_file(&mut self, file: &std::path::Path) -> Result<(), Error> {
        if !self.file_exists(file)? {
            return Err(Error::DeleteFile(DeleteFileError {
                file: file.display().to_string(),
                reason: "file does not exist".to_string(),
            }));
        }

        let index = self
            .files
            .iter()
            .enumerate()
            .find(|(_index, (f, _))| f.as_path() == file)
            .map(|(index, _file)| index)
            .unwrap();
        self.files.remove(index);
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
    fn canonicalize(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error> {
        self.linker.canonicalize(file)
    }

    fn create_symlink(
        &mut self,
        source: &path::Source,
        destination: &path::Destination,
    ) -> Result<(), Error> {
        writeln!(self.logger, "ln -s {} {}", source, destination)
            .map_err(|e| Error::Generic(format!("cannot write ln -s log: {}", e)))?;

        self.linker.create_symlink(source, destination)
    }

    fn create_directory(&mut self, directory: &std::path::Path) -> Result<(), Error> {
        writeln!(self.logger, "mkdir -p {}", directory.display())
            .map_err(|e| Error::Generic(format!("cannot write mkdir log: {}", e)))?;

        self.linker.create_directory(directory)
    }

    fn directory_exists(&mut self, directory: &std::path::Path) -> Result<bool, Error> {
        self.linker.directory_exists(directory)
    }

    fn file_exists(&mut self, file: &std::path::Path) -> Result<bool, Error> {
        self.linker.file_exists(file)
    }

    fn is_symlink(&mut self, file: &std::path::Path) -> bool {
        self.linker.is_symlink(file)
    }

    fn list_symlinks(
        &mut self,
        directory: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, Error> {
        self.linker.list_symlinks(directory)
    }

    fn read_link(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error> {
        writeln!(self.logger, "readlink {}", file.display())
            .map_err(|e| Error::Generic(format!("cannot write readlink log: {}", e)))?;

        self.linker.read_link(file)
    }

    fn delete_file(&mut self, file: &std::path::Path) -> Result<(), Error> {
        writeln!(self.logger, "rm {}", file.display())
            .map_err(|e| Error::Generic(format!("cannot write rm log: {}", e)))?;

        self.linker.delete_file(file)
    }
}

pub struct Filesystem;

impl Linker for Filesystem {
    fn canonicalize(&mut self, file: &std::path::Path) -> Result<std::path::PathBuf, Error> {
        file.canonicalize()
            .map_err(|e| Error::Generic(format!("cannot cannonicalize {}: {}", file.display(), e)))
    }

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

    fn create_directory(&mut self, directory: &std::path::Path) -> Result<(), Error> {
        std::fs::create_dir_all(directory).map_err(|e| {
            Error::CreateDirectory(CreateDirectoryError {
                directory: directory.display().to_string(),
                reason: e.to_string(),
            })
        })
    }

    fn directory_exists(&mut self, directory: &std::path::Path) -> Result<bool, Error> {
        if directory.exists() && !directory.is_dir() {
            return Err(Error::Generic(format!(
                "directory {} exists but is not a directory",
                directory.display()
            )));
        }

        Ok(directory.exists())
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

    fn is_symlink(&mut self, file: &std::path::Path) -> bool {
        file.is_symlink()
    }

    fn list_symlinks(
        &mut self,
        directory: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, Error> {
        let symlinks = std::fs::read_dir(directory)
            .map_err(|e| Error::Generic(e.to_string()))?
            .filter_map(|dir| dir.ok().map(|dir| dir.path()))
            .filter(|p| p.is_symlink())
            .collect::<Vec<std::path::PathBuf>>();
        Ok(symlinks)
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
    use std::io::Write;

    use super::*;

    #[test]
    fn verbose_noop_link() {
        let mut output = std::io::BufWriter::new(Vec::new());
        let mut dryrunner = Verbose::new(&mut output, Noop::default());
        let source = "/from/path".into();
        let destination = "/to/path".into();
        dryrunner
            .create_symlink(&source, &destination)
            .expect("cannot link path");

        let content = String::from_utf8(output.into_inner().unwrap()).unwrap();

        assert_eq!("ln -s /from/path /to/path\n", content)
    }

    #[test]
    fn verbose_create_directory() {
        let mut output = std::io::BufWriter::new(Vec::new());
        let mut dryrunner = Verbose::new(&mut output, Noop::default());
        dryrunner
            .create_directory("a/nice/path".as_ref())
            .expect("cannot create directory");

        let content = String::from_utf8(output.into_inner().unwrap()).unwrap();

        assert_eq!("mkdir -p a/nice/path\n", content)
    }

    #[test]
    fn verbose_directory_exists() {
        let mut output = std::io::BufWriter::new(Vec::new());
        let mut dryrunner = Verbose::new(&mut output, Noop::default());
        dryrunner
            .directory_exists("a/nice/path".as_ref())
            .expect("cannot check directory presence");

        let content = String::from_utf8(output.into_inner().unwrap()).unwrap();

        assert_eq!("", content)
    }

    #[test]
    fn verbose_file_exists() {
        let mut output = std::io::BufWriter::new(Vec::new());
        let mut dryrunner = Verbose::new(&mut output, Noop::default());
        dryrunner
            .file_exists("a/nice/path".as_ref())
            .expect("cannot check file presence");

        let content = String::from_utf8(output.into_inner().unwrap()).unwrap();

        assert_eq!("", content)
    }

    #[test]
    fn verbose_read_link() {
        let mut output = std::io::BufWriter::new(Vec::new());
        let mut dryrunner = Verbose::new(&mut output, Noop::default());

        let source = "/from/path".into();
        let destination = "a/nice/path".into();
        dryrunner
            .create_symlink(&source, &destination)
            .expect("cannot link path");

        dryrunner
            .read_link("a/nice/path".as_ref())
            .expect("cannot read link");

        let content = String::from_utf8(output.into_inner().unwrap()).unwrap();

        assert_eq!(
            "ln -s /from/path a/nice/path\nreadlink a/nice/path\n",
            content
        )
    }

    #[test]
    fn verbose_delete_file() {
        let mut output = std::io::BufWriter::new(Vec::new());
        let mut dryrunner = Verbose::new(&mut output, Noop::default());

        let source = "/from/path".into();
        let destination = "a/nice/path".into();
        dryrunner
            .create_symlink(&source, &destination)
            .expect("cannot link path");

        dryrunner
            .delete_file("a/nice/path".as_ref())
            .expect("cannot delete path");

        let content = String::from_utf8(output.into_inner().unwrap()).unwrap();

        assert_eq!("ln -s /from/path a/nice/path\nrm a/nice/path\n", content)
    }

    #[test]
    fn filesystem_create_symlink() {
        let ctx = TestWithTempDir::new("create-symlink");
        let src = ctx.dir.join("myfile.txt");
        let dest = ctx.dir.join("mylink.txt");
        let src_path = src.as_path();
        let dest_path = dest.as_path();
        let mut file = std::fs::File::create(ctx.dir.join("myfile.txt"))
            .expect("cannot create temporary file");
        file.write_all(b"some data")
            .expect("cannot write data to file");

        Filesystem
            .create_symlink(&src_path.into(), &dest_path.into())
            .expect("cannot create symlink");

        let link_target = std::fs::read_link(&dest).expect("cannot read symlink");

        assert_eq!(src_path, link_target)
    }

    #[test]
    fn filesystem_create_symlink_no_source() {
        let src_path = std::path::Path::new("a/non/existing/file");
        let dest_path = std::path::Path::new("a/non/existing/link");
        let err = Filesystem
            .create_symlink(&src_path.into(), &dest_path.into())
            .unwrap_err();
        assert_eq!(
            Error::CreateSymlink(CreateSymlinkError {
                source: "a/non/existing/file".to_string(),
                destination: "a/non/existing/link".to_string(),
                reason: "No such file or directory (os error 2)".to_string(),
            }),
            err
        )
    }

    #[test]
    fn filesystem_create_directory() {
        let ctx = TestWithTempDir::new("create-directory");
        let src = ctx.dir.join("my-directory");
        let src_path = src.as_path();

        assert!(!src_path.exists(), "directory shouldn't exist");

        Filesystem
            .create_directory(&src_path)
            .expect("cannot create directory");

        assert!(src_path.exists(), "directory should exist");
    }

    #[test]
    fn filesystem_directory_exists() {
        let ctx = TestWithTempDir::new("directory-exists");

        let exists = Filesystem
            .directory_exists(&ctx.dir)
            .expect("cannot check directory presence");

        assert!(exists, "directory should exist");
    }

    #[test]
    fn filesystem_directory_exists_do_no_exist() {
        let ctx = TestWithTempDir::new("directory-exists");
        let src_path = ctx.dir.join("mydirectory");

        let exists = Filesystem
            .directory_exists(&src_path)
            .expect("cannot check directory presence");

        assert!(!exists, "directory shouldn't exist")
    }

    #[test]
    fn filesystem_directory_exists_not_a_directory() {
        let ctx = TestWithTempDir::new("directory-exists");
        let src_path = ctx.dir.join("myfile.txt");
        std::fs::File::create(&src_path).expect("cannot create temporary file");

        let err = Filesystem.directory_exists(&src_path).unwrap_err();

        assert_eq!(
            Error::Generic(format!(
                "directory {} exists but is not a directory",
                src_path.display()
            )),
            err
        )
    }

    #[test]
    fn filesystem_file_exists() {
        let ctx = TestWithTempDir::new("file-exists");
        let src = ctx.dir.join("my-file.txt");
        let src_path = src.as_path();
        std::fs::File::create(&src_path).expect("cannot create temporary file");

        let exists = Filesystem
            .file_exists(&src_path)
            .expect("cannot check file presence");

        assert!(exists, "file should exist");
    }

    #[test]
    fn filesystem_file_exists_do_not_exist() {
        let ctx = TestWithTempDir::new("file-exists");
        let src = ctx.dir.join("my-file.txt");
        let src_path = src.as_path();

        let exists = Filesystem
            .file_exists(&src_path)
            .expect("cannot check file presence");

        assert!(!exists, "file shouldn't exist");
    }

    #[test]
    fn filesystem_file_exists_not_file() {
        let ctx = TestWithTempDir::new("file-exists");
        let src = ctx.dir.join("my-directory");
        let src_path = src.as_path();
        std::fs::create_dir(&src).expect("cannot create temporary directory");

        let err = Filesystem.file_exists(&src_path).unwrap_err();

        assert_eq!(
            Error::Generic(format!(
                "file {} exists but is not a file",
                src_path.display()
            )),
            err
        )
    }

    #[test]
    fn filesystem_read_link() {
        let ctx = TestWithTempDir::new("read-link");
        let src = ctx.dir.join("myfile.txt");
        let dest = ctx.dir.join("mylink.txt");
        let src_path = src.as_path();
        let dest_path = dest.as_path();
        let mut file = std::fs::File::create(ctx.dir.join("myfile.txt"))
            .expect("cannot create temporary file");
        file.write_all(b"some data")
            .expect("cannot write data to file");
        std::os::unix::fs::symlink(&src, &dest).expect("cannot create symlink");

        let link_target = Filesystem
            .read_link(&dest_path)
            .expect("cannot read symlink");

        assert_eq!(src_path, link_target)
    }

    #[test]
    fn filesystem_read_link_do_not_exist() {
        let ctx = TestWithTempDir::new("read-link");
        let dest = ctx.dir.join("mylink.txt");
        let dest_path = dest.as_path();

        let err = Filesystem.read_link(&dest_path).unwrap_err();

        assert_eq!(
            Error::ReadFile(ReadFileError {
                file: dest_path.display().to_string(),
                reason: "No such file or directory (os error 2)".to_string(),
            }),
            err
        )
    }

    #[test]
    fn filesystem_delete_file() {
        let ctx = TestWithTempDir::new("delete-file");
        let src = ctx.dir.join("my-file.txt");
        let src_path = src.as_path();
        std::fs::File::create(&src_path).expect("cannot create temporary file");

        assert!(src_path.exists(), "file should exist");
        Filesystem
            .delete_file(&src_path)
            .expect("cannot delete file");
        assert!(!src_path.exists(), "file shouldn't exist");
    }

    #[test]
    fn filesystem_delete_file_do_not_exist() {
        let ctx = TestWithTempDir::new("delete-file");
        let src = ctx.dir.join("my-file.txt");
        let src_path = src.as_path();

        let err = Filesystem.delete_file(&src_path).unwrap_err();

        assert_eq!(
            Error::DeleteFile(DeleteFileError {
                file: src_path.display().to_string(),
                reason: "No such file or directory (os error 2)".to_string(),
            }),
            err
        )
    }

    struct TestWithTempDir {
        dir: std::path::PathBuf,
    }

    impl TestWithTempDir {
        pub fn new(basename: &str) -> Self {
            let mut tmpdir = std::env::temp_dir();
            tmpdir.push(format!("{}-{}", basename, uuid::Uuid::new_v4().to_string()));
            std::fs::create_dir(&tmpdir)
                .expect(format!("cannot create temporary directory {}", tmpdir.display()).as_str());
            Self { dir: tmpdir }
        }
    }

    impl Drop for TestWithTempDir {
        fn drop(&mut self) {
            if let Err(err) = std::fs::remove_dir_all(&self.dir) {
                eprintln!(
                    "cannot cleanup temporary directory {}: {}",
                    self.dir.display(),
                    err
                );
            }
        }
    }
}
