use vek::Vec3;
use crate::core::GridNum;

pub struct GridObject<T, I> where T: GridNum, I: PartialEq + Clone {
    position: Vec3<T>,
    item: I,
}

impl<T, I> GridObject<T, I> where T: GridNum, I: PartialEq + Clone {
    pub fn new(position: Vec3<T>, item: I) -> Self {
        Self {
            position,
            item,
        }
    }

    pub fn item(&self) -> &I {
        &self.item
    }

    pub fn position(&self) -> Vec3<T> {
        self.position.clone()
    }
}