use crate::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}
