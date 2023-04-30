pub mod macos;
pub mod ubuntu;

pub trait System {
    fn new(show_output: bool) -> Self;
    fn refresh(&self);
    fn upgrade(&self);
}