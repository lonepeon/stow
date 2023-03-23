pub struct Noop;

impl std::io::Write for Noop {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn write() {
        let size = Noop.write("hello!".as_bytes()).unwrap();
        assert_eq!(6, size)
    }

    #[test]
    fn flush() {
        Noop.flush().unwrap();
    }
}
