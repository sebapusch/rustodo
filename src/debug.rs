use std::fs::OpenOptions;
use std::io::Write;

#[allow(unused)]
pub fn debug_log(message: &str) {
    match OpenOptions::new().write(true).open("/dev/pts/1") {
        Ok(mut file) => {
            writeln!(file, "{}", message);
            file.flush();
        }
        Err(_) => {}
    }
}
