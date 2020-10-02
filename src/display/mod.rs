mod display_crossterm;
use crate::game::{Board, Entity};
pub use display_crossterm::*;

pub trait GameDisplay {
    fn start() -> Self;
    fn display<'a>(
        &mut self,
        board: &Board,
        entities: impl Iterator<Item = &'a Entity>,
    ) -> Option<()>;
}
