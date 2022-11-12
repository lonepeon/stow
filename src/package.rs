use crate::{path, Error, ReadFileError};

#[derive(Debug)]
pub struct Package<'a> {
    path: std::path::PathBuf,
    name: &'a str,
}

impl<'a> Package<'a> {
    pub fn new(src_dir: &path::Source, name: &'a str) -> Result<Self, Error> {
        let path = src_dir.join(name);

        path.exists()
            .then_some(Self { name, path })
            .ok_or_else(|| Error::PackageNotFound(name.to_string()))
    }

    pub fn read_dirs(&self) -> Result<PackageIterator, Error> {
        Ok(PackageIterator {
            package: self,
            readdir: walkdir::WalkDir::new(&self.path).into_iter(),
            should_keep: |p| p.is_dir(),
        })
    }

    pub fn read_files(&self) -> Result<PackageIterator, Error> {
        Ok(PackageIterator {
            package: self,
            readdir: walkdir::WalkDir::new(&self.path).into_iter(),
            should_keep: |p| !p.is_dir(),
        })
    }
}

pub struct PackageIterator<'a> {
    package: &'a Package<'a>,
    readdir: walkdir::IntoIter,
    should_keep: fn(&std::fs::FileType) -> bool,
}

fn entry_to_filepath<'a>(
    package: &'a Package<'a>,
    entry: walkdir::Result<walkdir::DirEntry>,
    should_keep: fn(&std::fs::FileType) -> bool,
) -> Result<Option<String>, Error> {
    let entry = entry.map_err(|err| {
        Error::ReadFile(ReadFileError {
            file: package.name.to_string(),
            reason: err.to_string(),
        })
    })?;

    if !should_keep(&entry.file_type()) {
        return Ok(None);
    }

    entry
        .path()
        .strip_prefix(&package.path)
        .map_err(|_| {
            Error::ReadFile(ReadFileError {
                file: package.name.to_string(),
                reason: format!(
                    "cannot strip source folder {} from file path {}",
                    package.path.display(),
                    entry.path().display()
                ),
            })
        })
        .and_then(|path| {
            path.to_str()
                .ok_or_else(|| {
                    Error::ReadFile(ReadFileError {
                        file: package.name.to_string(),
                        reason: format!("cannot convert path {} to string", path.display()),
                    })
                })
                .map(|s| Some(s.to_string()))
        })
}

impl<'a> Iterator for PackageIterator<'a> {
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry = self.readdir.next()?;
        loop {
            match entry_to_filepath(self.package, entry, self.should_keep) {
                Ok(None) => {
                    entry = self.readdir.next()?;
                }
                Ok(Some(file)) => return Some(Ok(file)),
                Err(err) => return Some(Err(err)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;

    use super::*;

    #[test]
    fn package_not_found() {
        let err = Package::new(&"/not/a/folder".into(), "my-package").unwrap_err();
        assert_eq!(Error::PackageNotFound("my-package".to_string()), err)
    }

    #[test]
    fn package_exists() {
        Package::new(&"./golden-files".into(), "package-1").expect("package should exist");
    }

    #[test]
    fn read_dir() {
        let package =
            Package::new(&"./golden-files".into(), "package-1").expect("package should exist");
        let files: Vec<String> = package
            .read_files()
            .expect("should create a readdir iterator")
            .collect::<Result<Vec<String>, Error>>()
            .expect("should collect all files");
        assert_eq!(2, files.len(), "unexpected number of files in folder");
        assert_eq!("file-1".to_string(), files[0]);
        assert_eq!("file-2".to_string(), files[1]);
    }
}
