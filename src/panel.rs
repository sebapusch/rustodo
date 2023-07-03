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
    }

    fn print_list(&mut self) {
        let (_, terminal_height) = terminal_size().unwrap();

        print!("{}{}{}{}",
                cursor::Goto(1, terminal_height - 1),
                clear::BeforeCursor,
                cursor::Goto(1, 1),
                cursor::Hide);

        stdout().flush().unwrap();

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

    fn success_message(&self, msg: String) {
        self.cursor_bottom();
        print!("\r{}{}{}",
               color::Fg(color::Green),
               msg,
               style::Reset);
        stdout().flush().unwrap();

        thread::sleep(Duration::from_secs(1));

        self.clear_last_ln();
    }

    fn delete_todo(&mut self) -> bool {
        self.cursor_bottom();
        print!("\r{} Are you sure? (y/n) {}", color::Fg(color::Red), style::Reset);
        stdout().flush().unwrap();

        let confirm = self.confirm();

        if confirm {
            self.list.todos.remove(self.highlighted);

            if self.highlighted == self.list.todos.len() {
                self.highlighted -= 1;
            }
        }

        self.clear_last_ln();

        confirm
    }

    fn confirm(&self) -> bool {
        let stdin = stdin();

        for c in stdin.keys() {
            return match c.unwrap() {
                Key::Char('y') => true,
                _ => false
            }
        }

        false
    }

    fn add_todo(&mut self) {

        self.cursor_bottom();
        print!("{}", cursor::Show);
        self.stdout.suspend_raw_mode().unwrap();
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        self.list.todos.push(Todo {
            id: 2,
            item: input.trim_end().to_string(),
            priority: 0,
            tags: vec![],
            done: false,
        });

        self.stdout.activate_raw_mode().unwrap();
        self.clear_last_ln();
    }

    fn cursor_bottom(&self) {
        let (_, terminal_height) = terminal_size().unwrap();
        print!("\r{}", cursor::Goto(1, terminal_height));
        stdout().flush().unwrap();
    }

    fn clear_last_ln(&self) {
        let (_, terminal_height) = terminal_size().unwrap();
        print!("\r{}", cursor::Goto(1, terminal_height - 3));
        print!("{}", clear::AfterCursor);
        stdout().flush().unwrap();
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
                Key::Char('a') => {
                    self.add_todo();
                    self.print_list();
                },
                Key::Char('d') => {
                    if self.delete_todo() {
                        self.print_list();
                    }
                },
                _ => {}
            }
        }

        return KeyOutput::QUIT;
    }
}