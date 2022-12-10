/*
Low level traits shared between dot-display

*/

pub trait DotDisplay {
    fn refresh(&mut self, _path: &std::path::Path);
    fn draw_pixel(&mut self, x: i32, y: i32, _on: bool);
    fn print_fb(&mut self);
}
