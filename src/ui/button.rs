use raylib::prelude::*;

pub struct Button {
    rect: Rectangle,
    text: String,
    font_size: i32,
    normal_color: Color,
    hover_color: Color,
    text_color: Color,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: &str) -> Self {
        Button {
            rect: Rectangle::new(x, y, width, height),
            text: text.to_string(),
            font_size: 30,
            normal_color: Color::new(70, 70, 70, 255),
            hover_color: Color::new(100, 100, 100, 255),
            text_color: Color::WHITE,
        }
    }

    pub fn is_hovered(&self, mouse_pos: Vector2) -> bool {
        self.rect.check_collision_point_rec(mouse_pos)
    }

    pub fn is_clicked(&self, mouse_pos: Vector2, mouse_clicked: bool) -> bool {
        self.is_hovered(mouse_pos) && mouse_clicked
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, mouse_pos: Vector2) {
        let color = if self.is_hovered(mouse_pos) {
            self.hover_color
        } else {
            self.normal_color
        };

        d.draw_rectangle_rec(self.rect, color);
        d.draw_rectangle_lines_ex(self.rect, 2.0, Color::WHITE);

        let text_width = d.measure_text(&self.text, self.font_size);
        let text_x = self.rect.x + (self.rect.width - text_width as f32) / 2.0;
        let text_y = self.rect.y + (self.rect.height - self.font_size as f32) / 2.0;

        d.draw_text(
            &self.text,
            text_x as i32,
            text_y as i32,
            self.font_size,
            self.text_color,
        );
    }
}
