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

#[derive(Debug, Clone, Copy)]
pub enum WallHit {
    Left,
    Right,
    Top,
    Bottom,
}

impl Quad {
    pub fn from_corners(a: Vec2i, b: Vec2i) -> Self {
        Self::new([
            Vec2i::new(a.x, a.y),
            Vec2i::new(b.x, a.y),
            Vec2i::new(b.x, b.y),
            Vec2i::new(a.x, b.y),
        ])
    }

    pub fn aabb_collides(&self, other: &Quad) -> bool {
        // Assumes from_corners vertex order: [TL, TR, BR, BL]
        let (a0, a2) = (self.vertices[0], self.vertices[2]);  // TL, BR
        let (b0, b2) = (other.vertices[0], other.vertices[2]);

            a0.x < b2.x &&
            a2.x > b0.x &&
            a0.y < b2.y &&
            a2.y > b0.y
    }

    pub fn wall_hit(&self, wall: &Quad) -> Option<WallHit> {
        let (a0, a2) = (self.vertices[0], self.vertices[2]);
        let (w0, w2) = (wall.vertices[0], wall.vertices[2]);

        if a0.x <= w0.x {
            Some(WallHit::Left)
        } else if a2.x >= w2.x {
            Some(WallHit::Right)
        } else if a0.y <= w0.y {
            Some(WallHit::Top)
        } else if a2.y >= w2.y {
            Some(WallHit::Bottom)
        } else {
            None
        }
    }
}