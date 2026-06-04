use crate::math::Vec2i;

pub struct Line {
    pub vertices: [Vec2i; 2],
}

impl Line {
    pub fn new(a: Vec2i, b: Vec2i) -> Self {
        Self { vertices: [a, b] }
    }
}

pub struct Polygon<const N: usize> {
    pub vertices: [Vec2i; N],
}

impl<const N: usize> Polygon<N> {
    pub fn new(vertices: [Vec2i; N]) -> Self {
        Self { vertices }
    }

    pub fn edges(&self) -> Vec<Line> {
        let mut edges = Vec::new();

        for i in 0..N {
            let a = self.vertices[i];
            let b = self.vertices[(i + 1) % N];
            edges.push(Line::new(a, b));
        }

        edges
    }
}

pub type LineShape = Polygon<2>;
pub type Triangle = Polygon<3>;
pub type Quad = Polygon<4>;

impl Quad {
    pub fn from_corners(a: Vec2i, b: Vec2i) -> Self {
        Self::new([
            Vec2i::new(a.x, a.y),
            Vec2i::new(b.x, a.y),
            Vec2i::new(b.x, b.y),
            Vec2i::new(a.x, b.y),
        ])
    }
}