use crate::canvas::Canvas;

const MAX_COLOR_VALUE: f64 = 255.0;

pub fn ppm(canvas: &Canvas) -> String {
    let header = format!(
        "P3\n{} {}\n{}",
        canvas.width, canvas.height, MAX_COLOR_VALUE as u32
    );
    let mut body = String::new();
    for row in canvas.pixels.chunks(canvas.width) {
        let mut line = String::new();
        for pixel in row {
            let scaled_color = *pixel * MAX_COLOR_VALUE;
            let components = [
                scaled_color.r.clamp(0.0, MAX_COLOR_VALUE).round(),
                scaled_color.g.clamp(0.0, MAX_COLOR_VALUE).round(),
                scaled_color.b.clamp(0.0, MAX_COLOR_VALUE).round(),
            ];
            for component in components {
                let component_string = component.to_string();
                if line.len() + component_string.len() + 1 > 70 {
                    line.pop(); // Remove the trailing space
                    body.push_str(&line);
                    body.push('\n');
                    line.clear();
                }
                line.push_str(&component_string);
                line.push(' ');
            }
        }
        line.pop(); // Remove the trailing space
        body.push_str(&line);
        body.push('\n');
    }
    format!("{}\n{}", header, body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::Canvas;
    use crate::color::Color;

    #[test]
    fn ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3, None);
        let color1 = Color::new(1.0, 0.0, 0.0);
        let color2 = Color::new(0.0, 0.5, 0.0);
        let color3 = Color::new(-0.5, 0.0, 1.0);

        canvas.set_pixel(0, 0, color1);
        canvas.set_pixel(2, 1, color2);
        canvas.set_pixel(4, 2, color3);
        let expected_ppm = "P3\n\
            5 3\n\
            255\n\
            255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";
        assert_eq!(expected_ppm, ppm(&canvas));
    }

    #[test]
    fn splits_long_lines_in_ppm_files() {
        let canvas = Canvas::new(10, 2, Some(Color::new(1.0, 0.8, 0.6)));
        let expected_ppm = "P3\n\
            10 2\n\
            255\n\
            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
            153 255 204 153 255 204 153 255 204 153 255 204 153\n\
            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
            153 255 204 153 255 204 153 255 204 153 255 204 153\n";
        assert_eq!(expected_ppm, ppm(&canvas));
    }

    #[test]
    fn ppm_files_are_terminated_by_newline_character() {
        let canvas = Canvas::new(5, 3, None);
        let ppm_data = ppm(&canvas);
        assert!(
            ppm_data.ends_with('\n'),
            "PPM data should end with a newline character"
        );
    }
}
