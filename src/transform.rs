use super::drawable::Coord;

pub trait Transform {
    fn translate(&self, by: Coord) -> Self;
}
