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
            for file in package.read_dir()? {
                let file = file?;
                let file_src_path = root_src.join(p).join(&file);
                let file_dest_path = root_dest.join(&file);

                let parent_folder = file_dest_path
                    .parent()
                    .ok_or_else(|| Error::ParentFolder(file_dest_path.display().to_string()))?;

                if !self.linker.folder_exists(parent_folder)? {
                    self.linker.create_folder(parent_folder)?;
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

        assert_eq!(
            vec![
                "mkdir -p /home/jdoe",
                "readlink /home/jdoe/file-1",
                "ln -s golden-files/package-1/file-1 /home/jdoe/file-1",
                "readlink /home/jdoe/file-2",
                "ln -s golden-files/package-1/file-2 /home/jdoe/file-2",
                "mkdir -p /home/jdoe/subfolder",
                "readlink /home/jdoe/subfolder/file-2",
                "ln -s golden-files/package-2/subfolder/file-2 /home/jdoe/subfolder/file-2",
                "readlink /home/jdoe/file-1",
                "rm /home/jdoe/file-1",
                "ln -s golden-files/package-2/file-1 /home/jdoe/file-1",
            ],
            output.trim().split('\n').collect::<Vec<&str>>(),
        );
        assert_eq!(vec![
            "warning: override symlink /home/jdoe/file-1 from golden-files/package-1/file-1 to golden-files/package-2/file-1",
            "warning: delete file /home/jdoe/file-1",
        ], warning.trim().split('\n').collect::<Vec<&str>>());
    }
}
