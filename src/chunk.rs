use std::ops::{Index, IndexMut};

use perlin2d::PerlinNoise2D;
use tui::{style::{Color, Style}, text::Span, widgets::canvas::Context};

use crate::blocks::{BlockKind, stones::Stones};

pub const CHUNK_SIZE: i32 = 16;

pub enum Terrain {
    Water,
    Grass,
    Stone
}

impl Terrain {
    pub fn color(&self) -> Color {
        match self {
            Terrain::Water => Color::Cyan,
            Terrain::Grass => Color::Green,
            Terrain::Stone => Color::DarkGray,
        }
    }

    pub fn style(&self) -> Style {
        Style::default().bg(self.color())
    }

    pub fn span<'a>(&self) -> Span<'a> {
        match self {
            Terrain::Water => Span::styled("~", self.style()),
            Terrain::Grass => Span::styled(" ", self.style()),
            Terrain::Stone => Span::styled(" ", self.style()),
        }
    }
}

pub struct Chunk(pub i32, pub i32, pub Vec<(Terrain, Option<BlockKind>)>);

impl Chunk {
    pub fn new(col: i32, row: i32, perlin: &PerlinNoise2D) -> Self {
        let mut terrain = Vec::new();
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                let x = (col*CHUNK_SIZE + i) as f64;
                let y = (row*CHUNK_SIZE + j) as f64;
                let value = perlin.get_noise(x, y);
                if value >= 0.98 {
                    terrain.push((Terrain::Stone, Some(BlockKind::Stones(Stones::new()))))
                } else if value >= 0.85 {
                    terrain.push((Terrain::Stone, None))
                } else if value >= -1.0 {
                    terrain.push((Terrain::Grass, None))
                } else {
                    terrain.push((Terrain::Water, None))
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
