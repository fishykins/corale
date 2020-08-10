use super::VertexIndex;
use super::IndexType;

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FaceIndex<Ix = crate::core::DefaultIx>(Ix);

#[derive(Clone)]
pub struct Face {
    verts: Vec<VertexIndex>,
}

impl Face {
    pub fn new(verts: Vec<VertexIndex>) -> Self {
        Self {
            verts,
        }
    }

    pub fn empty() -> Self {
        Self {
            verts: Vec::new(),
        }
    }

    pub fn capacity(cap: usize) -> Self {
        Self {
            verts: Vec::with_capacity(cap),
        }
    }

    pub fn add_vert(&mut self, vert: VertexIndex) {
        self.verts.push(vert);
    }

    pub fn verts(&self) -> Vec<VertexIndex> {
        self.verts.clone()
    }

    pub fn consume(self) -> Vec<VertexIndex> {
        self.verts
    }
}

impl<Ix: IndexType> FaceIndex<Ix> {
    #[inline]
    pub fn new(x: usize) -> Self {
        FaceIndex(IndexType::new(x))
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0.index()
    }

    #[inline]
    pub fn end() -> Self {
        FaceIndex(IndexType::max())
    }
}