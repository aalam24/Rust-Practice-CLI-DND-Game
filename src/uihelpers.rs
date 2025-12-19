use macroquad::prelude::*;

pub fn draw_button(text: &str, x: f32, y: f32, w: f32, h: f32) -> bool {
    let mouse_pos = mouse_position();
    let hovering = mouse_pos.0 >= x
        && mouse_pos.0 <= x + w
        && mouse_pos.1 >= y
        && mouse_pos.1 <= y + h;

        let bg_color = if hovering {
            Color::new(0.3, 0.3, 0.35, 1.0)
        } else {
            Color::new(0.2, 0.2, 0.25, 1.0)
        };
    let border_color = if hovering {GOLD} else {LIGHTGRAY};
    draw_rectangle(x, y, w, h, 2.0, bg_color);
    draw_rectangle_lines(x, y, w, h, 2.0, border_color);
    
    

}