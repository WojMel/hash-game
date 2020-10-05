use std::fmt::Display;
use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor, execute, queue,
    style::{self, style, Attribute, Color, StyledContent},
    terminal::{self, size},
};

use crate::display::{Board, Entity, GameDisplay};
use crate::game::{Block, EntityBlock, CHUNK_SIZE};

pub struct CTDisplay {
    cols: u16,
    rows: u16,
    stdout: Stdout,
}

impl GameDisplay for CTDisplay {
    fn start() -> Self {
        let mut stdout = stdout();
        terminal::enable_raw_mode()
            .expect("impl GameDisplay for CTDisplay: fn start: terminal::enable_raw_mode()");

        let (cols, rows) = size()
            .expect("impl GameDisplay for CTDisplay: fn start: let (mut cols, mut rows) = size()");

        execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)
            .expect("impl GameDisplay for CTDisplay: fn start: execute!(");

        CTDisplay { cols, rows, stdout }
    }

    fn display<'a>(
        &mut self,
        board: &Board,
        entities: impl Iterator<Item = &'a Entity>,
    ) -> Option<()> {
        let start_x = self.cols / 2 - board.width() as u16 * CHUNK_SIZE;
        let start_y = (self.rows - board.height() as u16 * CHUNK_SIZE) / 2;
        let print_h_wall = style::PrintStyledContent(
            style(
                WALL_SHAPE
                    .chars()
                    .cycle()
                    .take((board.width() as usize * CHUNK_SIZE as usize + 2) * 2)
                    .collect::<String>(),
            )
            .with(WALL_COLOR_FG)
            .on(WALL_COLOR_BG)
            .attribute(WALL_ATTRIBUTE),
        );

        queue!(
            self.stdout,
            cursor::MoveTo(start_x - 3, start_y - 1),
            print_h_wall,
            cursor::MoveTo(start_x - 3, start_y + CHUNK_SIZE * board.height() as u16),
            print_h_wall,
        )
        .expect("");

        queue!(self.stdout, cursor::MoveTo(start_x - 1, start_y)).expect("err queue move row");

        board.chunks().chunks(board.width().into()).for_each(|row| {
            row.iter()
                .cycle()
                .take((board.width() as u16 * CHUNK_SIZE) as usize)
                .enumerate()
                .for_each(|(n, chunk)| {
                    queue!(
                        self.stdout,
                        style::PrintStyledContent(
                            chunk.blocks[0 + (n / board.width() as usize) * CHUNK_SIZE as usize]
                                .styled_content()
                        ),
                        style::PrintStyledContent(
                            chunk.blocks[1 + (n / board.width() as usize) * CHUNK_SIZE as usize]
                                .styled_content()
                        ),
                        style::PrintStyledContent(
                            chunk.blocks[2 + (n / board.width() as usize) * CHUNK_SIZE as usize]
                                .styled_content()
                        )
                    )
                    .expect("err print borad");
                    if n as u16 % (board.width() as u16) + 1 == board.width() as u16 {
                        queue!(
                            self.stdout,
                            style::PrintStyledContent(Block::Wall.styled_content()),
                            cursor::MoveToColumn(start_x - 2),
                            style::PrintStyledContent(Block::Wall.styled_content()),
                            cursor::MoveDown(1),
                        )
                        .expect("err new line");
                    }
                });
        });

        entities.for_each(|e| {
            queue!(
                self.stdout,
                cursor::MoveTo(start_x + e.pos.0 * 2 - 1, start_y + e.pos.1),
                style::PrintStyledContent(e.block.styled_content())
            )
            .expect("err print entities");
        });

        self.stdout.flush().expect("flush");
        None
    }
}

impl Drop for CTDisplay {
    fn drop(&mut self) {
        terminal::disable_raw_mode()
            .expect("impl Drop for CTDisplay: fn drop: terminal::disable_raw_mode()");
        execute!(self.stdout, cursor::Show, terminal::LeaveAlternateScreen)
            .expect("impl Drop for CTDisplay: fn drop: execute!(self.stdout, cursor::Show, terminal::LeaveAlternateScreen)");
    }
}

trait CTPrint<'a, D: Display + Clone + 'a> {
    fn styled_content(&self) -> StyledContent<D>;
}

static WALL_SHAPE: &'static str = "[]";
static WALL_COLOR_FG: Color = Color::DarkBlue;
static WALL_COLOR_BG: Color = Color::Black;
static WALL_ATTRIBUTE: Attribute = Attribute::Bold;

impl CTPrint<'_, &'static str> for Block {
    fn styled_content(&self) -> StyledContent<&'static str> {
        lazy_static! {
            static ref WALL: StyledContent<&'static str> = style(WALL_SHAPE)
                .with(WALL_COLOR_FG)
                .on(WALL_COLOR_BG)
                .attribute(WALL_ATTRIBUTE);
            static ref EMPTY: StyledContent<&'static str> = style("  ");
        }
        match self {
            Block::Wall => *WALL,
            Block::Empty => *EMPTY,
        }
    }
}

impl CTPrint<'_, &'static str> for EntityBlock {
    fn styled_content(&self) -> StyledContent<&'static str> {
        lazy_static! {
            static ref PLAYER: [StyledContent<&'static str>; 4] = [
                style("<>").with(Color::DarkGreen).on(Color::Black),
                style("<>").with(Color::DarkRed).on(Color::Black),
                style("<>").with(Color::DarkBlue).on(Color::Black),
                style("<>").with(Color::DarkBlue).on(Color::Black),
            ];
            static ref TRAP: StyledContent<&'static str> =
                style("##").with(Color::Cyan).on(Color::Black);
            static ref POINT: StyledContent<&'static str> =
                style("()").with(Color::Yellow).on(Color::Black);
            static ref NONE: StyledContent<&'static str> = style("");
        }

        match self {
            EntityBlock::Player(n) if n > &0 && n < &4 => PLAYER[*n as usize],
            EntityBlock::Trap => *TRAP,
            _ => *NONE,
        }
    }
}
