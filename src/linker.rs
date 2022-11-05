use crate::{package, path, Error, LinkingError};

pub trait Linker {
    fn link(
        &mut self,
        source: std::path::PathBuf,
        destination: std::path::PathBuf,
    ) -> Result<(), Error>;
}

pub struct DryRunLinker<W: std::io::Write> {
    logger: W,
}
impl<W: std::io::Write> DryRunLinker<W> {
    pub fn new(w: W) -> Self {
        DryRunLinker { logger: w }
    }
}

impl<W: std::io::Write> Linker for DryRunLinker<W> {
    fn link(
        &mut self,
        source: std::path::PathBuf,
        destination: std::path::PathBuf,
    ) -> Result<(), Error> {
        writeln!(
            self.logger,
            "ln -s {} {}",
            source.display(),
            destination.display()
        )
        .map_err(|_| {
            Error::Linking(LinkingError {
                source: format!("{}", source.display()),
                destination: format!("{}", destination.display()),
                reason: "failed to print link log",
            })
        })
    }
}

#[derive(Default)]
pub struct OSLinker {}

impl Linker for OSLinker {
    fn link(
        &mut self,
        _source: std::path::PathBuf,
        _destination: std::path::PathBuf,
    ) -> Result<(), Error> {
        Ok(())
    }
}

pub fn copy<L: Linker + ?Sized>(
    linker: &mut L,
    root_src: &path::Source,
    root_dest: &path::Destination,
    packages: Vec<String>,
) -> Result<(), Error> {
    for p in packages.iter() {
        let package = package::Package::new(root_src, p)?;
        for file in package.read_dir()? {
            let file = file?;
            let src = root_src.join(&file);
            let dest = root_dest.join(&file);

            linker.link(src, dest)?
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dry_run_link() {
        let mut output = std::io::BufWriter::new(Vec::new());
        let mut dryrunner = DryRunLinker::new(&mut output);
        dryrunner
            .link("/from/path".into(), "/to/path".into())
            .expect("cannot link path");

        let content = String::from_utf8(output.into_inner().unwrap()).unwrap();

        assert_eq!("ln -s /from/path /to/path\n", content)
    }
}
