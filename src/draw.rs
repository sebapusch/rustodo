use termion::clear;
use termion::color;
use termion::cursor;
use termion::style;

pub fn danger(text: String) -> String {
    format!("{}{}{}", color::Fg(color::Yellow), text, style::Reset)
}

pub fn success(text: String) -> String {
    format!("{}{}{}", color::Fg(color::Green), text, style::Reset)
}

pub fn bold(text: String) -> String {
    format!("{}{}{}", style::Bold, text, style::Reset)
}

pub fn input(name: &str, x: u16, y: u16) -> String {
    let mut out = String::new();

    out.push_str(cursor::Show.to_string().as_str());
    out.push('╭');
    out.push_str(name);
    for _ in 0..48 - name.len() {
        out.push('─');
    }
    out.push_str("╮\r\n");
    out.push('│');
    for _ in 0..48 {
        out.push(' ');
    }
    out.push_str("│\r\n╰");
    for _ in 0..48 {
        out.push('─');
    }
    out.push('╯');
    out = position(out, x, y);
    out.push_str(cursor::Goto(x + 1, y + 1).to_string().as_str());

    out
}

pub fn bordered(content: String, title: String, title_bottom: String, width: u16) -> String {
    let mut out = title_border_top(width, title);
    for line in content.split("\n") {
        out.push_str(line);
        let visible_len = visible_length(line);
        if visible_len == 0 {
            continue;
        }
        let padding = if visible_len >= width - 1 {
            0
        } else {
            width - visible_len - 1
        };
        for _ in 0..padding {
            out.push(' ');
        }
        out.push_str("│\r\n");
    }
    out.push_str(title_border_bottom(width, title_bottom).as_str());
    out
}

pub fn position(content: String, x: u16, y: u16) -> String {
    let mut out = String::new();
    let mut counter: u16 = 0;
    for line in content.split("\n") {
        out.push_str(cursor::Goto(x, y + counter).to_string().as_str());
        out.push_str(line);
        counter += 1;
    }
    out
}

pub fn title_border_top(mut length: u16, title: String) -> String {
    length = length - (title.len() as u16) - 2;
    let mut bar = String::new();
    for _ in 0..length / 2 {
        bar.push_str("─");
    }
    bar.push_str(title.as_str());
    for _ in length / 2..length {
        bar.push_str("─");
    }
    format!("╭{}╮\r\n", bar)
}

pub fn title_border_bottom(mut length: u16, title: String) -> String {
    length = length - (title.len() as u16) - 2;
    let mut bar = String::new();
    for _ in 0..length / 2 {
        bar.push_str("─");
    }
    bar.push_str(title.as_str());
    for _ in length / 2..length {
        bar.push_str("─");
    }
    format!("╰{}╯\r\n", bar)
}

pub fn clear_all() -> String {
    format!(
        "{}{}{}{}",
        style::Reset,
        clear::All,
        cursor::Show,
        cursor::Goto(1, 1)
    )
}

pub fn hide_cursor() -> String {
    format!("{}", cursor::Hide)
}

fn visible_length(input: &str) -> u16 {
    let mut count = 0;
    let mut in_escape = false;
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        if c == '\x1B' {
            in_escape = true;
        } else if in_escape && c == 'm' {
            in_escape = false;
        } else if !in_escape {
            count += 1;
        }
    }

    count
}
