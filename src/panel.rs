pub use crate::todo::{TodoList, Todo};

use std::thread;
use std::io::{Read, Write, stdout, stdin, Bytes, StdoutLock};
use std::time::Duration;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::style;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::cursor;
use termion::terminal_size;
use termion::clear;


#[derive(PartialEq, Eq)]
enum KeyOutput {
    QUIT,
    COMMAND,
}

pub struct Panel<'a> {
    list: TodoList,
    highlighted: usize,
    stdout: RawTerminal<StdoutLock<'a>>,
}

impl<'a> Panel<'a> {

    pub fn new (list: TodoList) -> Self {
        let stdout = stdout();
        let stdout = stdout.lock().into_raw_mode().unwrap();

        Panel {
            list,
            highlighted: 0,
            stdout,
        }
    }

    pub fn start (&mut self) {

        print!("{}", clear::All);

        self.print_list();
        let output = self.process_key();

        if output == KeyOutput::QUIT {
            print!("{}{}", cursor::Show, style::Reset);
            return;
        }

        if output == KeyOutput::COMMAND {
            self.command();
            self.start();
        }
    }

    fn print_list(&mut self) {
        let (_, terminal_height) = terminal_size().unwrap();

        print!("{}{}{}{}",
                cursor::Goto(1, terminal_height - 1),
                clear::BeforeCursor,
                cursor::Goto(1, 1),
                cursor::Hide);

        for i in 0..self.list.todos.len() {
            let todo = self.list.todos[i].clone();
            self.print_todo(&todo, i);
        }
    }

    fn print_todo(&mut self, todo: &Todo, i: usize) {

        let check;

        if todo.done {
            check = "[x]";
        } else {
            check = "[ ]";
        }

        if i == self.highlighted {
            print!("\r{}{} {}{}\n", style::Bold, check, todo.item, style::Reset);
        } else {
            print!("\r{} {}\n", check, todo.item);
        }
    }

    fn command(&self) {
        print!("{}{}", cursor::Goto(0, 5), cursor::Show);
        println!("uhmmmm");

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{n} bytes read");
                println!("{input}");
            }
            Err(error) => println!("error: {error}"),
        }
    }

    fn success_message(&self, msg: String) {
        let (_, terminal_height) = terminal_size().unwrap();
        print!("\r{}{}{}{}",
                cursor::Goto(1, terminal_height),
                color::Fg(color::Green),
                msg,
                style::Reset);
        stdout().flush().unwrap();

        let clear_thread = thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            print!("{}{}",
                   cursor::Goto(1, terminal_height - 1),
                   clear::AfterCursor);
            stdout().flush().unwrap();
        });
    }

    fn process_key(&mut self) -> KeyOutput {

        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => return KeyOutput::QUIT,
                Key::Up => {
                    if self.highlighted > 0 {
                        self.highlighted -= 1;
                        self.print_list();
                    }
                },
                Key::Down => {
                    if self.highlighted < self.list.todos.len() - 1 {
                        self.highlighted += 1;
                        self.print_list();
                    }
                },
                Key::Char('\n') => {
                    self.list.todos[self.highlighted].toggle();
                    self.print_list();
                },
                Key::Char('s') => {
                    self.list.save(String::from("./stuff"))
                        .expect("Error");


                    self.success_message(String::from("Successfully saved list"));
                },
                Key::Esc => return KeyOutput::COMMAND,
                _ => {}
            }
        }

        return KeyOutput::QUIT;
    }
}