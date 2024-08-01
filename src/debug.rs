use std::fs::OpenOptions;
use std::io::{self, Write};

#[allow(unused)]
pub fn debug_log(message: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).open("/dev/pts/3")?;
    writeln!(file, "{}", message)?;
    file.flush()
}
