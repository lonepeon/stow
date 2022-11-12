use crate::{linker, package, path, Error};

pub struct Command<'a, W: std::io::Write, L: linker::Linker + ?Sized> {
    logger: W,
    linker: &'a mut L,
}

impl<'a, W: std::io::Write, L: linker::Linker + ?Sized> Command<'a, W, L> {
    pub fn new(logger: W, linker: &'a mut L) -> Self {
        Self { logger, linker }
    }

    pub fn stow(
        &mut self,
        root_src: &path::Source,
        root_dest: &path::Destination,
        packages: Vec<String>,
    ) -> Result<(), Error> {
        for p in packages.iter() {
            let package = package::Package::new(root_src, p)?;
            for file in package.read_files()? {
                let file = file?;
                let file_src_path = root_src.join(p).join(&file);
                let file_dest_path = root_dest.join(&file);

                let parent_directory = file_dest_path
                    .parent()
                    .ok_or_else(|| Error::ParentDirectory(file_dest_path.display().to_string()))?;

                if !self.linker.directory_exists(parent_directory)? {
                    self.linker.create_directory(parent_directory)?;
                }

                let src = file_src_path.as_path();
                let dest = file_dest_path.as_path();

                if let Ok(current_src) = self.linker.read_link(dest) {
                    if current_src.as_path().eq(src) {
                        continue;
                    }

                    writeln!(
                        self.logger,
                        "warning: override symlink {} from {} to {}",
                        dest.display(),
                        current_src.display(),
                        src.display()
                    )
                    .map_err(|e| Error::Generic(format!("failed to print warning: {}", e)))?;
                }

                if self.linker.file_exists(dest)? {
                    writeln!(self.logger, "warning: delete file {}", dest.display(),)
                        .map_err(|e| Error::Generic(format!("failed to print warning: {}", e)))?;
                    self.linker.delete_file(dest)?;
                }

                self.linker.create_symlink(&src.into(), &dest.into())?
            }
        }

        Ok(())
    }

    pub fn unstow(
        &mut self,
        root_src: &path::Source,
        root_dest: &path::Destination,
        packages: Vec<String>,
    ) -> Result<(), Error> {
        for p in packages.iter() {
            let package = package::Package::new(root_src, p)?;
            for dir in package.read_dirs()? {
                let dir = dir?;
                let dir_src_path = root_src.join(p).join(&dir);
                let dir_dest_path = root_dest.join(&dir);
                if !self.linker.directory_exists(&dir_dest_path)? {
                    continue;
                }

                for destination_file in self.linker.list_symlinks(&dir_dest_path)? {
                    let target_link = self.linker.read_link(&destination_file)?;
                    let target_link = target_link
                        .parent()
                        .ok_or_else(|| {
                            Error::Generic(format!("cannot get {} dirname", target_link.display(),))
                        })
                        .and_then(|p| self.linker.canonicalize(p))?;

                    let dir_src_path = self.linker.canonicalize(&dir_src_path)?;

                    if target_link == dir_src_path {
                        self.linker.delete_file(&destination_file)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linker;

    #[test]
    fn stow_packages() {
        let mut commands_output = std::io::BufWriter::new(Vec::new());
        let mut warnings_output = std::io::BufWriter::new(Vec::new());
        let mut linker = Box::new(linker::Verbose::new(
            &mut commands_output,
            linker::Noop::default(),
        ));

        let src: path::Source = "golden-files".into();
        let dest: path::Destination = "/home/jdoe".into();

        Command::new(&mut warnings_output, linker.as_mut())
            .stow(
                &src,
                &dest,
                vec!["package-1".to_string(), "package-2".to_string()],
            )
            .expect("shouldn't fail");

        let output = String::from_utf8(commands_output.into_inner().unwrap()).unwrap();
        let warning = String::from_utf8(warnings_output.into_inner().unwrap()).unwrap();

        let expected_output = vec![
            "mkdir -p /home/jdoe",
            "readlink /home/jdoe/file-1",
            "ln -s golden-files/package-1/file-1 /home/jdoe/file-1",
            "readlink /home/jdoe/file-2",
            "ln -s golden-files/package-1/file-2 /home/jdoe/file-2",
            "mkdir -p /home/jdoe/subdirectory",
            "readlink /home/jdoe/subdirectory/file-2",
            "ln -s golden-files/package-2/subdirectory/file-2 /home/jdoe/subdirectory/file-2",
            "readlink /home/jdoe/file-1",
            "rm /home/jdoe/file-1",
            "ln -s golden-files/package-2/file-1 /home/jdoe/file-1",
        ];

        let actual_output = output.trim().split('\n').collect::<Vec<&str>>();
        assert_eq!(expected_output.len(), actual_output.len());
        assert!(find_subset(&actual_output, &expected_output[0..3]));
        assert!(find_subset(&actual_output, &expected_output[3..5]));
        assert!(find_subset(&actual_output, &expected_output[3..5]));
        assert!(find_subset(&actual_output, &expected_output[5..8]));
        assert!(find_subset(&actual_output, &expected_output[8..11]));

        assert_eq!(vec![
            "warning: override symlink /home/jdoe/file-1 from golden-files/package-1/file-1 to golden-files/package-2/file-1",
            "warning: delete file /home/jdoe/file-1",
        ], warning.trim().split('\n').collect::<Vec<&str>>());
    }

    #[test]
    fn unstow_packages() {
        let mut commands_output = std::io::BufWriter::new(Vec::new());
        let mut warnings_output = std::io::BufWriter::new(Vec::new());
        let mut noop = linker::Noop::default();
        noop.directories = vec!["/home/jdoe".into(), "/home/jdoe/subdirectory".into()];
        noop.files = vec![
            (
                "/home/jdoe/file-other".into(),
                "golden-files/package-3/file-other".into(),
            ),
            (
                "/home/jdoe/subdirectory/file-2".into(),
                "golden-files/package-2/subdirectory/file-2".into(),
            ),
            (
                "/home/jdoe/file-1".into(),
                "golden-files/package-1/file-1".into(),
            ),
            (
                "/home/jdoe/file-2".into(),
                "golden-files/package-2/file-2".into(),
            ),
        ];
        let mut linker = Box::new(linker::Verbose::new(&mut commands_output, noop));

        let src: path::Source = "golden-files".into();
        let dest: path::Destination = "/home/jdoe".into();

        Command::new(&mut warnings_output, linker.as_mut())
            .unstow(
                &src,
                &dest,
                vec!["package-1".to_string(), "package-2".to_string()],
            )
            .expect("shouldn't fail");

        let output = String::from_utf8(commands_output.into_inner().unwrap()).unwrap();
        let warning = String::from_utf8(warnings_output.into_inner().unwrap()).unwrap();

        assert_eq!(
            vec![
                // handle package1 root
                "readlink /home/jdoe/file-other",
                "readlink /home/jdoe/subdirectory/file-2",
                "readlink /home/jdoe/file-1",
                "rm /home/jdoe/file-1",
                "readlink /home/jdoe/file-2",
                // handle package2 root
                "readlink /home/jdoe/file-other",
                "readlink /home/jdoe/subdirectory/file-2",
                "readlink /home/jdoe/file-2",
                "rm /home/jdoe/file-2",
                // handle package2 subdirectory
                "readlink /home/jdoe/file-other",
                "readlink /home/jdoe/subdirectory/file-2",
                "rm /home/jdoe/subdirectory/file-2"
            ],
            output.trim().split('\n').collect::<Vec<&str>>(),
        );
        assert_eq!("", warning.trim())
    }

    fn find_subset(haystack: &[&str], needles: &[&str]) -> bool {
        for i in 0..haystack.len() - needles.len() + 1 {
            if haystack[i..i + needles.len()] == needles[..] {
                return true;
            }
        }

        false
    }
}
