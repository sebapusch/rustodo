use crate::draw::{self, danger, position};
use crate::reader::Reader;
pub use crate::todo::{Todo, TodoList};
pub use crate::Settings;

use std::collections::HashMap;
use std::io::{stdout, Stdout, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::terminal_size;

#[derive(PartialEq, Eq)]
pub enum Operation {
    Create,
    Update,
}

#[derive(PartialEq, Eq)]
pub enum Event {
    Refresh,
    Quit,
    MoveUp,
    MoveDown,
    HighlightUp,
    HighlightDown,
    Toggle,
    Save,
    Delete,
    Input(Operation),
    Commit(Operation, String),
    KeyPressed(Key),
    IoError(String),
}

pub struct Panel {
    list: TodoList,
    highlighted: usize,
    stdout: RawTerminal<Stdout>,
    settings: Settings,
    buffer: String,
    event_sender: Sender<Event>,
    event_receiver: Receiver<Event>,
    event_queue: HashMap<Key, Vec<fn(&mut Panel)>>,
    reader: Reader,
}

impl Panel {
    pub fn new(list: TodoList, settings: Settings) -> Self {
        let stdout = stdout().into_raw_mode().unwrap();
        let (event_sender, event_receiver) = mpsc::channel();
        let event_queue = HashMap::new();
        let reader = Reader::new(event_sender.clone());
        Panel {
            list,
            highlighted: 0,
            stdout,
            reader,
            settings,
            buffer: String::new(),
            event_sender,
            event_receiver,
            event_queue,
        }
    }

    pub fn start(&mut self) {
        self.refresh();
        self.start_loop();
    }

    fn refresh(&mut self) {
        self.push(draw::clear_all());
        self.push(draw::hide_cursor());
        let content = self.draw();
        self.push(content);
        self.render();
    }

    fn render(&mut self) {
        self.stdout.write_all(self.buffer.as_bytes()).unwrap();
        self.stdout.flush().unwrap();
    }

    fn draw(&mut self) -> String {
        let mut out = String::new();

        if self.list.todos.len() == 0 {
            out = "Empty list...".into();
        } else {
            for i in 0..self.list.todos.len() {
                let todo = self.list.todos[i].clone();
                out.push_str(self.draw_todo(&todo, i).as_str());
            }
        }

        let title_bottom = format!("{}/{}", self.list.completed(), self.list.total());

        draw::bordered(out, self.list.name.clone(), title_bottom, 100)
    }

    fn push(&mut self, text: String) {
        self.buffer.push_str(text.as_str());
    }

    fn on_click(&mut self, k: Key, callback: fn(&mut Panel)) {
        if let Some(queue) = self.event_queue.get_mut(&k) {
            queue.push(callback);
        } else {
            self.event_queue.insert(k, vec![callback]);
        }
    }

    fn draw_todo(&mut self, todo: &Todo, i: usize) -> String {
        let mut out = String::new();

        if todo.done {
            out.push_str(self.settings.checked_symbol.as_str());
        } else {
            out.push_str(self.settings.unchecked_symbol.as_str());
        }

        out.push(' ');
        out.push_str(&todo.item.as_str());
        out.push('\n');

        if i == self.highlighted {
            draw::bold(out)
        } else {
            out
        }
    }

    fn delete_todo(&mut self) {
        let (_, h) = terminal_size().unwrap();
        self.push(position(danger("Are you sure? (y/n)".into()), 1, h));
        self.render();
        self.on_click(Key::Char('y'), |panel| {
            panel.list.todos.remove(panel.highlighted);
            if panel.list.todos.len() == 0 {
                panel.highlighted = 0;
            } else if panel.highlighted == panel.list.todos.len() {
                panel.highlighted -= 1;
            }
            panel.refresh();
        });
    }

    fn edit_todo(&mut self, item: String) {
        self.list.todos[self.highlighted].item = item;
    }

    fn add_todo(&mut self, item: String) {
        self.list.todos.push(Todo {
            id: 2,
            item,
            priority: 0,
            tags: vec![],
            done: false,
        });
    }

    fn move_down(&mut self) {
        let tmp = self.list.todos[self.highlighted].clone();
        self.list.todos[self.highlighted] = self.list.todos[self.highlighted + 1].clone();
        self.list.todos[self.highlighted + 1] = tmp;
        self.highlighted += 1;
    }

    fn move_up(&mut self) {
        let tmp = self.list.todos[self.highlighted].clone();
        self.list.todos[self.highlighted] = self.list.todos[self.highlighted - 1].clone();
        self.list.todos[self.highlighted - 1] = tmp;
        self.highlighted -= 1;
    }

    fn start_loop(&mut self) {
        self.reader.listen_events();
        self.handle_next_event();
    }

    fn handle_next_event(&mut self) {
        let event = self.event_receiver.recv().unwrap();

        match event {
            Event::Refresh => self.refresh(),
            Event::Quit => return,
            Event::Input(op) => match op {
                Operation::Create => self.draw_input("Todo".into()),
                Operation::Update => {
                    if self.list.todos.len() > 0 {
                        self.draw_input(self.list.todos[self.highlighted].item.clone())
                    }
                }
            },
            Event::Commit(op, content) => {
                self.stdout.activate_raw_mode().unwrap();
                match op {
                    Operation::Create => self.add_todo(content),
                    Operation::Update => self.edit_todo(content),
                }
                self.refresh();
            }
            Event::MoveUp => {
                if self.list.todos.len() >= 2 && self.highlighted < self.list.todos.len() - 1 {
                    self.move_down();
                    self.refresh();
                }
            }
            Event::MoveDown => {
                if self.list.todos.len() >= 2 && self.highlighted > 0 {
                    self.move_up();
                    self.refresh();
                }
            }
            Event::HighlightUp => {
                if self.highlighted > 0 {
                    self.highlighted -= 1;
                    self.refresh();
                }
            }
            Event::HighlightDown => {
                if self.highlighted < self.list.todos.len() - 1 {
                    self.highlighted += 1;
                    self.refresh();
                }
            }
            Event::Delete => {
                if self.list.todos.len() > 0 {
                    self.delete_todo();
                }
            }
            Event::Toggle => {
                if self.list.todos.len() > 0 {
                    self.list.todos[self.highlighted].toggle();
                    self.refresh();
                }
            }
            Event::Save => {
                self.list.save(&self.settings.todopath).expect("Error");
                self.flash_success("Successfully saved list".into());
            }
            Event::KeyPressed(k) => {
                if let Some(queue) = self.event_queue.get_mut(&k).cloned() {
                    for callback in queue {
                        callback(self);
                    }
                }
                self.event_queue.remove(&k);
            }
            Event::IoError(_) => {}
        }

        self.handle_next_event();
    }

    fn draw_input(&mut self, name: String) {
        self.stdout.suspend_raw_mode().unwrap();

        let (_, h) = terminal_size().unwrap();

        self.push(draw::input(name.as_str(), 1, h - 3));
        self.render();
    }

    fn flash_success(&mut self, message: String) {
        let (_, h) = terminal_size().unwrap();
        let sender = self.event_sender.clone();

        self.push(draw::position(draw::success(message), 1, h));
        self.render();

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            sender.send(Event::Refresh).unwrap();
        });
    }
}
