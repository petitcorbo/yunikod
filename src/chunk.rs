use std::ops::{Index, IndexMut};

use perlin2d::PerlinNoise2D;
use rand::{thread_rng, Rng};
use tui::{style::{Color, Style}, text::Span, widgets::canvas::Context};

use crate::blocks::{BlockKind, stones::Stones, tree::Tree, Block, sticks::Sticks, rock::Rock};

pub const CHUNK_SIZE: i64 = 16;

pub enum Terrain {
    DeepWater,
    Water,
    Grass,
    Stone
}

impl Terrain {
    pub fn color(&self) -> Color {
        match self {
            Terrain::DeepWater => Color::Rgb(54, 181, 201),
            Terrain::Water => Color::Rgb(54, 201, 148),
            Terrain::Grass => Color::Rgb(70, 201, 54),
            Terrain::Stone => Color::Rgb(84, 106, 78),
        }
    }

    pub fn style(&self) -> Style {
        Style::default().bg(self.color())
    }

    pub fn span<'a>(&self) -> Span<'a> {
        match self {
            Terrain::DeepWater => Span::styled(" ", self.style()),
            Terrain::Water => Span::styled(" ", self.style()),
            Terrain::Grass => Span::styled(" ", self.style()),
            Terrain::Stone => Span::styled(" ", self.style()),
        }
    }
}

pub struct Chunk(pub i64, pub i64, pub Vec<(Terrain, Option<BlockKind>)>);

impl Chunk {
    pub fn new(col: i64, row: i64, perlin: &PerlinNoise2D) -> Self {
        let mut terrain = Vec::new();
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                let x = (col*CHUNK_SIZE + i) as f64;
                let y = (row*CHUNK_SIZE + j) as f64;
                let value = perlin.get_noise(x, y);
                if value >= 40.0 {
                    terrain.push((Terrain::Stone, Some(Rock::generate())))
                } else if value >= 30.0 {
                    terrain.push((Terrain::Stone, None))
                } else if value >= 10.0 {
                    if thread_rng().gen_ratio(1, 15) {
                        terrain.push((Terrain::Grass, Some(Tree::generate())))
                    } else if thread_rng().gen_ratio(1, 100) {
                        terrain.push((Terrain::Grass, Some(Stones::generate())))
                    } else if thread_rng().gen_ratio(1, 100) {
                        terrain.push((Terrain::Grass, Some(Sticks::generate())))
                    } else {
                        terrain.push((Terrain::Grass, None))
                    }
                } else if value >= 0.0 {
                    terrain.push((Terrain::Grass, None))
                } else if value >= -25.0 {
                    terrain.push((Terrain::Water, None))
                } else {
                    terrain.push((Terrain::DeepWater, None))
                }
            }
        }
        Self(col, row, terrain)
    }

    pub fn draw(&self, ctx: &mut Context) {
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                let x = (self.0*CHUNK_SIZE + i) as f64;
                let y = (self.1*CHUNK_SIZE + j) as f64;
                let span = self[(i as usize, j as usize)].0.span();
                ctx.print(x, y, span);
                if let Some(block) = &self[(i as usize, j as usize)].1 {
                    ctx.print(x, y, block.shape());
                }
            }
        }
    }

    pub fn average_terrain(&self) -> Terrain {
        let mut counter = [0; 4];
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                match self[(i as usize, j as usize)].0 {
                    Terrain::DeepWater => counter[0] += 1,
                    Terrain::Water => counter[1] += 1,
                    Terrain::Grass => counter[2] += 1,
                    Terrain::Stone => counter[3] += 1,
                }
            }
        }
        let mut max = 0;
        let mut idx = 0;
        for (i, c) in counter.iter().enumerate() {
            if c > &max {
                idx = i;
                max = *c;
            }
        }
        match idx {
            0 => Terrain::DeepWater,
            1 => Terrain::Water,
            2 => Terrain::Grass,
            3 => Terrain::Stone,
            _ => Terrain::Grass
        }
    }
}

impl Index<(usize, usize)> for Chunk {
    type Output = (Terrain, Option<BlockKind>);

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.2[i * (CHUNK_SIZE as usize) + j]
    }
}

impl IndexMut<(usize, usize)> for Chunk {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.2[i * (CHUNK_SIZE as usize) + j]
    }
}
