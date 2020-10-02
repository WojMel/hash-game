#[macro_use]
extern crate lazy_static;

use crate::display::{CTDisplay, GameDisplay};
use crate::game::{Board, Entity, EntityBlock::*};
use std::time::Duration;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use rand::prelude::thread_rng;

mod controls;
mod display;
mod game;
mod utils;

fn main() {
    let mut rng = thread_rng();

    let board = Board::random_rng(11, 11, &mut rng);
    let mut display = CTDisplay::start();
    loop {
        display.display(
            &board,
            vec![Entity::new(Player(1), (1, 1)), Entity::new(Trap, (4, 4))].iter(),
        );
        Duration::from_millis(500);
        if let Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
        }) = read().expect("read failed")
        {
            break;
        }
    }
}
