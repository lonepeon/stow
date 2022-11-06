use crate::{package, path, Error, LinkingError};

pub trait Linker {
    fn link(&mut self, source: &path::Source, destination: &path::Destination)
        -> Result<(), Error>;
}

pub struct Noop;

impl Linker for Noop {
    fn link(
        &mut self,
        _source: &path::Source,
        _destination: &path::Destination,
    ) -> Result<(), Error> {
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
    fn link(
        &mut self,
        source: &path::Source,
        destination: &path::Destination,
    ) -> Result<(), Error> {
        self.linker.link(source, destination)?;

        writeln!(self.logger, "ln -s {} {}", source, destination).map_err(|_| {
            Error::Linking(LinkingError {
                source: format!("{}", source),
                destination: format!("{}", destination),
                reason: "failed to print link log",
            })
        })
    }
}

pub struct Filesystem<W: std::io::Write> {
    logger: W,
}

impl<W: std::io::Write> Filesystem<W> {
    pub fn new(logger: W) -> Self {
        Self { logger }
    }
}

impl<W: std::io::Write> Linker for Filesystem<W> {
    fn link(
        &mut self,
        source: &path::Source,
        destination: &path::Destination,
    ) -> Result<(), Error> {
        writeln!(self.logger, "log only if overriding a file").map_err(|_| {
            Error::Linking(LinkingError {
                source: format!("{}", source),
                destination: format!("{}", destination),
                reason: "failed to print link log",
            })
        })?;

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
            let file_src_path = root_src.join(&file);
            let file_dest_path = root_dest.join(&file);
            let src = file_src_path.as_path().into();
            let dest = file_dest_path.as_path().into();

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
        let mut dryrunner = Verbose::new(&mut output, Noop);
        let source = "/from/path".into();
        let destination = "/to/path".into();
        dryrunner
            .link(&source, &destination)
            .expect("cannot link path");

        let content = String::from_utf8(output.into_inner().unwrap()).unwrap();

        assert_eq!("ln -s /from/path /to/path\n", content)
    }
}
