use bevy::prelude::*;

pub fn within(x: f32, mid: f32, size: f32) -> bool {
    x >= (mid - size / 2.0) && x <= (mid + size / 2.0)
}

pub fn cursor_collision(cursor_position: Vec2, rect_position: Vec2, rect_size: Vec2) -> bool {
    let h: bool = within(cursor_position.x, rect_position.x, rect_size.x);
    let v: bool = within(cursor_position.y, rect_position.y, rect_size.y);
    h && v
}

pub fn relative_cursor_position(absolute_cursor_position: Vec2, window_width: f32, window_height: f32) -> Vec2 {
        let window_dimensions: Vec2 = Vec2::new(window_width / 2.0, window_height / 2.0);
        absolute_cursor_position - window_dimensions
}
