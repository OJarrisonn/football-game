pub mod game;
pub mod util;

pub trait GameObject {
    fn id(&self) -> String;
    fn path(&self, root: &str) -> String;
}