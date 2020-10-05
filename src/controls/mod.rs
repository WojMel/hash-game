use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers},
};
use futures::stream::StreamExt;

use crate::game::Slide;

#[derive(Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn to_slide(&self, n: u8) -> Slide {
        use Direction::*;
        match self {
            Up => Slide::Up(n),
            Right => Slide::Right(n),
            Down => Slide::Down(n),
            Left => Slide::Left(n),
        }
    }
}

#[derive(Clone)]
pub enum Command {
    Exit,
    Slide(Direction),
}


pub struct CTControls {
    reader: EventStream
}

impl CTControls {
    pub fn new() -> Self {
        Self {
            reader: EventStream::new(),
        }
    }

    pub async fn next(&mut self) -> Option<Command> {
        use Command::*;
        match self.reader.next().await {
                Some(Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }))) => Some(Exit),
            Some(Ok(Event::Key(KeyEvent {
                code: KeyCode::Up,
                ..
            }))) => Some(Slide(Direction::Up)),
            Some(Ok(Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }))) => Some(Slide(Direction::Right)),
            Some(Ok(Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }))) => Some(Slide(Direction::Down)),
            Some(Ok(Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }))) => Some(Slide(Direction::Left)),
            _ => None,
        }

    }
}
