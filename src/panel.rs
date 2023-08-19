pub use crate::Settings;
pub use crate::todo::{TodoList, Todo};
pub use crate::draw::draw;

use std::io::{stdout, stdin, StdoutLock};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::event::Key;
use termion::input::TermRead;
use draw::FlashType;


#[derive(PartialEq, Eq)]
enum KeyOutput {
    QUIT,
    COMMAND,
}

pub struct Panel<'a> {
    list: TodoList,
    highlighted: usize,
    stdout: RawTerminal<StdoutLock<'a>>,
    settings: Settings,
}

fn clear_last_ln() {
}

impl<'a> Panel<'a> {

    pub fn new (list: TodoList, settings: Settings) -> Self {
        let stdout = stdout();
        let stdout = stdout.lock().into_raw_mode().unwrap();

        Panel {
            list,
            highlighted: 0,
            stdout,
            settings,
        }
    }

    pub fn start (&mut self) {

        draw::clear_all();
        self.print_list();

        let output = self.process_key();

        if output == KeyOutput::QUIT {
            draw::reset();
            return;
        }
    }

    fn print_list(&mut self) {

        draw::clear_content();

        for i in 0..self.list.todos.len() {
            let todo = self.list.todos[i].clone();
            self.print_todo(&todo, i);
        }
    }

    fn print_todo(&mut self, todo: &Todo, i: usize) {

        let check;

        if todo.done {
            check = self.settings.checked_symbol.clone();
        } else {
            check = self.settings.unchecked_symbol.clone();
        }

        let mut item = todo.item.clone();

        if i == self.highlighted {

            item = draw::bold(item);
        }


        draw::text_ln(format!("{} {}", check, item));
    }

    fn delete_todo(&mut self) -> bool {
        
        draw::cursor_bottom(true);
        draw::warning(String::from("Are you sure? (y/n)"));

        let confirm = self.confirm();

        if confirm {
            self.list.todos.remove(self.highlighted);

            if self.highlighted == self.list.todos.len() {
                self.highlighted -= 1;
            }
        }

        draw::clear_bottom();

        confirm
    }

    fn edit_todo(&mut self) {

        draw::cursor_bottom(false);
        draw::text(format!("({}) ", self.list.todos[self.highlighted].item));

        self.list.todos[self.highlighted].item = self.input();
        
        clear_last_ln();
    }

    fn add_todo(&mut self) {
        
        draw::cursor_bottom(true);

        let item = self.input();

        self.list.todos.push(Todo {
            id: 2,
            item,
            priority: 0,
            tags: vec![],
            done: false,
        });
        
        draw::clear_bottom();
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
                    self.list.save(&self.settings.todopath)
                        .expect("Error");


                    draw::flash_msg(FlashType::Success, String::from("Successfully saved list"));
                },
                Key::Esc => return KeyOutput::COMMAND,
                Key::Char('a') => {
                    self.add_todo();
                    self.print_list();
                },
                Key::Char('e') => {
                    self.edit_todo();
                    self.print_list();
                }
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

    fn input(&mut self) -> String {

        self.stdout.suspend_raw_mode().unwrap();

        let mut buffer = String::with_capacity(20);

        stdin().read_line(&mut buffer).unwrap();

        let input = buffer.to_owned().trim_end().parse().unwrap();
        self.stdout.activate_raw_mode().unwrap();

        input
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

}
