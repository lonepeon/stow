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

                self.linker.create_symlink(&src.into(), &dest.into())?
            }
        }

        Ok(())
    }
}
