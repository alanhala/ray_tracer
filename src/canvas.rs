use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, color: Option<Color>) -> Self {
        Self {
            width,
            height,
            pixels: vec![color.unwrap_or(Color::black()); width * height],
        }
    }

    // TODO: we are not checking that x and y are within bounds
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = self.pixel_index(x, y);
        self.pixels[index] = color;
    }

    // TODO: we are not checking that row and col are within bounds
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let index = self.pixel_index(x, y);
        self.pixels[index]
    }

    fn pixel_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn creates_canvas_with_width_and_height() {
        let canvas = Canvas::new(10, 20, None);

        for pixel in &canvas.pixels {
            assert_eq!(pixel.r, 0.0);
            assert_eq!(pixel.g, 0.0);
            assert_eq!(pixel.b, 0.0);
        }
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
    }

    #[test]
    fn writes_pixel_to_canvas() {
        let mut canvas = Canvas::new(10, 20, None);
        let color = Color::new(1.0, 0.5, 0.0);

        canvas.set_pixel(2, 3, color);

        assert_eq!(canvas.get_pixel(2, 3), color);
    }
}
