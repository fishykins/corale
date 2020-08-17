use num::Num;

pub trait Area<T> where T: Num {
    fn area(&self) -> T;
}