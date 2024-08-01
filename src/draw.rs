pub mod draw {

    use std::io::{stdout, Write};
    use std::thread;
    use std::time::Duration;
    use termion::clear;
    use termion::color;
    use termion::cursor;
    use termion::style;
    use termion::terminal_size;

    #[allow(dead_code)]
    pub enum FlashType {
        Success,
        Warning,
        Danger,
    }

    pub fn flush() {
        stdout().flush().unwrap();
    }

    pub fn bold(text: String) -> String {
        format!("{}{}{}", style::Bold, text, style::Reset)
    }

    pub fn warning(txt: String) {
        text(format!(
            "{}{}{}",
            color::Fg(color::Yellow),
            txt,
            style::Reset
        ));
    }

    pub fn text_ln(text: String) {
        println!("\r{}", text);
    }

    pub fn text(text: String) {
        print!("\r{}", text);
        flush();
    }

    pub fn reset() {
        print!("{}{}{}", style::Reset, clear::All, cursor::Show);
        flush();
    }

    pub fn clear_all() {
        print!("{}", clear::All);
        flush();
    }

    pub fn clear_content() {
        let (_, terminal_height) = terminal_size().unwrap();

        print!(
            "{}{}{}{}",
            cursor::Goto(1, terminal_height - 1),
            clear::BeforeCursor,
            cursor::Goto(1, 1),
            cursor::Hide
        );

        flush();
    }

    //    pub fn cursor_top() {
    //
    //        let (_, terminal_height) = terminal_size().unwrap();
    //
    //        print!("\r{}", cursor::Goto(0, terminal_height));
    //
    //        flush();
    //    }

    pub fn clear_bottom() {
        let (_, terminal_height) = terminal_size().unwrap();

        print!("\r{}", cursor::Goto(1, terminal_height - 3));
        print!("{}", clear::AfterCursor);

        flush();
    }

    pub fn cursor_bottom(show: bool) {
        let (_, terminal_height) = terminal_size().unwrap();

        print!("\r{}", cursor::Goto(1, terminal_height));

        if show {
            print!("\r{}", cursor::Show);
        }

        flush();
    }

    pub fn flash_msg(flash_type: FlashType, message: String) {
        let color = match flash_type {
            FlashType::Success => color::Fg(color::Green).to_string(),
            FlashType::Warning => color::Fg(color::Yellow).to_string(),
            FlashType::Danger => color::Fg(color::Red).to_string(),
        };

        cursor_bottom(false);

        print!("\r{}{}{}", color, message, style::Reset);

        flush();

        thread::sleep(Duration::from_secs(1));

        clear_bottom();

        //thread::spawn(|| {}); todo don't block main thread
    }
}
