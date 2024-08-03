use std::fs::OpenOptions;
use std::io::Write;

#[allow(unused)]
pub fn debug_log(message: &str) {
    let mut file = OpenOptions::new().write(true).open("/dev/pts/2").unwrap();
    writeln!(file, "{}", message);
    file.flush();
}
