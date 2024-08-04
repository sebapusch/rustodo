use crate::panel::{Event, Operation, UiSection};
use std::{io::stdin, sync::mpsc::Sender, thread};
use termion::event::Key;
use termion::input::TermRead;

pub struct Reader {
    event_sender: Sender<Event>,
}

impl Reader {
    pub fn new(event_sender: Sender<Event>) -> Self {
        Reader { event_sender }
    }

    pub fn listen_events(&mut self) {
        let sender = self.event_sender.clone();
        thread::spawn(move || {
            for k in stdin().keys() {
                let event = match k {
                    Ok(key) => match key {
                        Key::Char('a') => {
                            sender.send(Event::Input(Operation::Create)).unwrap();
                            Event::Commit(Operation::Create, Reader::input())
                        }
                        Key::Char('q') | Key::Esc => Event::Quit,
                        Key::Up => Event::HighlightUp,
                        Key::Down => Event::HighlightDown,
                        Key::Char('\n') => Event::Toggle,
                        Key::Char('s') => Event::Save,
                        Key::Char('e') => {
                            sender.send(Event::Input(Operation::Update)).unwrap();
                            Event::Commit(Operation::Update, Reader::input())
                        }
                        Key::Char('d') => {
                            sender.send(Event::Input(Operation::Delete)).unwrap();
                            if Reader::confirm() {
                                Event::Commit(Operation::Delete, String::new())
                            } else {
                                Event::Clear(Some(UiSection::Status))
                            }
                        }
                        Key::Char('r') => Event::Redraw,
                        Key::Char('f') => Event::Filter,
                        Key::Right => Event::MoveUp,
                        Key::Left => Event::MoveDown,
                        other => Event::KeyPressed(other),
                    },
                    Err(err) => Event::IoError(err.to_string()),
                };
                sender.send(event).unwrap();
            }
        });
    }

    fn confirm() -> bool {
        for e in stdin().keys() {
            if let Some(confirmed) = match e {
                Ok(k) => match k {
                    Key::Char('y') => Some(true),
                    Key::Char('n') => Some(false),
                    _ => None,
                },
                Err(_) => Some(false),
            } {
                return confirmed;
            }
        }
        false
    }

    fn input() -> String {
        let mut buffer = String::with_capacity(30);
        stdin().read_line(&mut buffer).unwrap();
        buffer.trim().into()
    }
}
