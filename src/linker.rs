use crate::{package, path, Error, LinkingError};

pub trait Linker {
    fn link(
        &mut self,
        source: &std::path::Path,
        destination: &std::path::Path,
    ) -> Result<(), Error>;
}

pub struct NoopLinker;

impl Linker for NoopLinker {
    fn link(
        &mut self,
        _source: &std::path::Path,
        _destination: &std::path::Path,
    ) -> Result<(), Error> {
        Ok(())
    }
}

pub struct VerboseLinker<W: std::io::Write, L: Linker> {
    logger: W,
    linker: L,
}
impl<W: std::io::Write, L: Linker> VerboseLinker<W, L> {
    pub fn new(logger: W, linker: L) -> Self {
        VerboseLinker { logger, linker }
    }
}

impl<W: std::io::Write, L: Linker> Linker for VerboseLinker<W, L> {
    fn link(
        &mut self,
        source: &std::path::Path,
        destination: &std::path::Path,
    ) -> Result<(), Error> {
        self.linker.link(source, destination)?;

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

pub struct OSLinker;

impl Linker for OSLinker {
    fn link(
        &mut self,
        _source: &std::path::Path,
        _destination: &std::path::Path,
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

            linker.link(&src, &dest)?
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verbose_noop_link() {
        let mut output = std::io::BufWriter::new(Vec::new());
        let mut dryrunner = VerboseLinker::new(&mut output, NoopLinker);
        let source = std::path::Path::new("/from/path");
        let destination = std::path::Path::new("/to/path");
        dryrunner
            .link(source, destination)
            .expect("cannot link path");

        let content = String::from_utf8(output.into_inner().unwrap()).unwrap();

        assert_eq!("ln -s /from/path /to/path\n", content)
    }
}
