#[macro_use]
extern crate lazy_static;

use std::time::{Duration};

use rand::prelude::thread_rng;
use tokio::{select, time::{interval_at, Instant}};

use display::{CTDisplay, GameDisplay};
use game::{Chunk, Board, Entity, EntityBlock::*};
use controls::{CTControls, Command};
use utils::*;

mod controls;
mod display;
mod game;
mod utils;



#[tokio::main]
async fn main() {
    let mut rng = thread_rng();

    let mut board = Board::random_rng(5, 5, &mut rng);
    let entities = vec![Entity::new(Player(1), (1, 1)), Entity::new(Trap, (4, 4))];

    let mut display = CTDisplay::start();
    let mut controls = CTControls::new();


    // static NUMBER_OF_PLAYERS: u8 = 1;
    static MAX_MOVES: usize = 5;
    // let slider = (1..=NUMBER_OF_PLAYERS).cycle();
    let mut refresh = true;
    let mut moves = MAX_MOVES; // assume player one

    let mut timeout_cycle = OnceIn::new(10);
    let mut timeout = interval_at(Instant::now() + Duration::from_millis(200), Duration::from_millis(200));
    loop {
        if refresh {
            display.display(
                &mut board,
                entities.iter(),
            );
            refresh = false;
        }
        select! {
            command = controls.next() => {
                if moves > 0 { // assume player one (if this player has moves)
                    use Command::*;
                    match command {
                        Some(Exit) => { break }
                        Some(Slide(ref dir)) => { // assume it's a move
                            board.slide(dir.to_slide(0), Chunk::random(&mut rng));
                            moves -= 1;
                        }
                        _ => ()
                    }
                    refresh = true;

                    // if current_player == slider && all_players_out_of_moves
                    {
                        // slider can slide
                        // refresh = true;
                    }
                }
            },
            _ = timeout.tick() => {
                // TODO add additional time for sliding (game stages MOVING|SLIDING|...)
                if let Some(true) = timeout_cycle.next() {
                    // new game tick
                    // reset players' options
                    moves = MAX_MOVES;
                }
                refresh = true;
            }
        }
    }
}

/*

get cmd for Player p
    if p has moves
        do move && refresh
    elif players - p haven't moves and p turn to slide
        slide && refresh unless
if timeout for controls
    all players done
    new cycle
if timeout progress
    refresh

*/
