use raylib::prelude::*;

pub struct Checkbox {
    rect: Rectangle,
    label: String,
    is_checked: bool,
    font_size: i32,
}

impl Checkbox {
    pub fn new(x: f32, y: f32, size: f32, label: &str, initial_value: bool) -> Self {
        Checkbox {
            rect: Rectangle::new(x, y, size, size),
            label: label.to_string(),
            is_checked: initial_value,
            font_size: 25,
        }
    }

    pub fn is_hovered(&self, mouse_pos: Vector2) -> bool {
        self.rect.check_collision_point_rec(mouse_pos)
    }

    pub fn handle_click(&mut self, mouse_pos: Vector2, mouse_clicked: bool) -> bool {
        if self.is_hovered(mouse_pos) && mouse_clicked {
            self.is_checked = !self.is_checked;
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn is_checked(&self) -> bool {
        self.is_checked
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.is_checked = checked;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, mouse_pos: Vector2) {
        let box_color = if self.is_hovered(mouse_pos) {
            Color::new(100, 100, 100, 255)
        } else {
            Color::new(70, 70, 70, 255)
        };

        // Draw checkbox box
        d.draw_rectangle_rec(self.rect, box_color);
        d.draw_rectangle_lines_ex(self.rect, 2.0, Color::WHITE);

        // Draw checkmark if checked
        if self.is_checked {
            let padding = 4.0;
            let inner_rect = Rectangle::new(
                self.rect.x + padding,
                self.rect.y + padding,
                self.rect.width - padding * 2.0,
                self.rect.height - padding * 2.0,
            );
            d.draw_rectangle_rec(inner_rect, Color::GREEN);
        }

        // Draw label
        let label_x = self.rect.x + self.rect.width + 15.0;
        let label_y = self.rect.y + (self.rect.height - self.font_size as f32) / 2.0;
        d.draw_text(
            &self.label,
            label_x as i32,
            label_y as i32,
            self.font_size,
            Color::WHITE,
        );
    }
}
