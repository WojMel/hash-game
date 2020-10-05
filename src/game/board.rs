use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};

#[derive(Clone, Copy)]
pub enum Block {
    Wall,
    Empty,
}

pub const CHUNK_SIZE: u16 = 3;

pub struct Chunk {
    pub blocks: [Block; (CHUNK_SIZE * CHUNK_SIZE) as usize],
}

pub struct Board {
    width: u8,
    height: u8,
    chunks: Vec<Chunk>,
}

pub enum Slide {
    Up(u8),
    Right(u8),
    Down(u8),
    Left(u8),
}

impl Board {
    pub fn random_rng(width: u8, height: u8, rng: &mut ThreadRng) -> Self {
        let width = width * 2 + 1;
        let height = height * 2 + 1;
        let mut chunks = Vec::<Chunk>::with_capacity((width * height).into());
        (0..height).for_each(|_y| (0..height).for_each(|_x| chunks.push(Chunk::random(rng))));
        Self {
            width,
            height,
            chunks,
        }
    }

    pub fn chunks<'a>(&'a self) -> &'a Vec<Chunk> {
        &self.chunks
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn slide(&mut self, slide: Slide, mut new: Chunk) {
        use Slide::*;
        let width = self.width as usize;
        let height = self.height as usize;
        match slide {
            Up(n) => {
                for i in 1..=height {
                    std::mem::swap(
                        &mut self.chunks[n as usize * 2 + 1 + (height - i) * width],
                        &mut new,
                    );
                }
            }
            Right(n) => {
                for i in 0..width {
                    std::mem::swap(&mut self.chunks[(n as usize * 2 + 1) * width + i], &mut new);
                }
            }
            Down(n) => {
                for i in 0..height {
                    std::mem::swap(&mut self.chunks[n as usize * 2 + 1 + i * width], &mut new);
                }
            }
            Left(n) => {
                for i in 1..=width {
                    std::mem::swap(
                        &mut self.chunks[(n as usize * 2 + 1) * width + (width - i)],
                        &mut new,
                    );
                }
            }
        }
    }
}

impl Chunk {
    pub fn random(rng: &mut ThreadRng) -> Self {
        lazy_static! {
            static ref DISTR: Uniform<u8> = Uniform::from(1..16);
            static ref IDX: [usize; 4] = [1, 3, 5, 7];
        }

        let mut new = Self::new();

        for _ in 0..((DISTR.sample(rng) as f32).log2() as i32 - 4).abs() {
            new.blocks[IDX[DISTR.sample(rng) as usize / 4]] = Block::Wall;
        }

        new
    }

    pub fn new() -> Self {
        Self {
            blocks: [
                Block::Wall,
                Block::Empty,
                Block::Wall,
                Block::Empty,
                Block::Empty,
                Block::Empty,
                Block::Wall,
                Block::Empty,
                Block::Wall,
            ],
        }
    }
}
