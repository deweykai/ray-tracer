use crate::color::Color;

#[derive(Debug)]
pub struct Canvas {
    pub width: isize,
    pub height: isize,
    pub pixels: Vec<Color>,
}

#[derive(Debug)]
pub enum CanvasError {
    ReadError,
    WriteError,
}

fn line_wrap(s: String) -> String {
    if s.len() <= 70 {
        return s;
    }
    for i in (0..70).rev() {
        if s.as_bytes()[i] == b' ' {
            return format!("{}\n{}", &s[..i], line_wrap(s[i + 1..].to_string()));
        }
    }
    s
}

impl Canvas {
    pub fn new(width: isize, height: isize) -> Canvas {
        let pixels: Vec<Color> = (0..(width * height))
            .into_iter()
            .map(|_| Color::new(0., 0., 0.))
            .collect();
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(mut self, x: isize, y: isize, color: Color) -> Canvas {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return self;
        }
        self.pixels[(y * self.width + x) as usize] = color;
        self
    }

    pub fn read_pixel(&self, x: isize, y: isize) -> Result<Color, CanvasError> {
        Ok(self.pixels[(y * self.width + x) as usize])
    }

    pub fn to_ppm(&self) -> String {
        let header = format!("P3\n{} {}\n255", self.width, self.height);
        let body = (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| self.read_pixel(x, y).unwrap())
                    .map(|p| p.to_string())
                    .reduce(|acc, s| acc + " " + &s)
                    .unwrap()
            })
            .map(line_wrap)
            .reduce(|acc, s| acc + "\n" + &s)
            .unwrap();
        format!("{}\n{}\n", header, body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        c.pixels
            .iter()
            .for_each(|color| assert_eq!(color, &Color::new(0., 0., 0.)));
    }
    #[test]
    fn write_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1., 0., 0.);
        c = c.write_pixel(2, 3, red);
        assert_eq!(c.read_pixel(2, 3).expect("failed to read pixel"), red);
    }
    #[test]
    fn create_ppm_header() {
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        let c = Canvas::new(5, 3)
            .write_pixel(0, 0, c1)
            .write_pixel(2, 1, c2)
            .write_pixel(4, 2, c3);
        let ppm = c.to_ppm();
        let ppm_lines: Vec<&str> = ppm.split('\n').collect();
        assert_eq!(ppm_lines[0], "P3");
        assert_eq!(ppm_lines[1], "5 3");
        assert_eq!(ppm_lines[2], "255");
        assert_eq!(ppm_lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(ppm_lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(ppm_lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }
    #[test]
    fn split_long_lines() {
        let s = String::from(
            "001 002 003 004 005 006 007 008 009 010 011 012 013 014 015 016 017 018 019 020",
        );
        assert_eq!(
            line_wrap(s),
            "001 002 003 004 005 006 007 008 009 010 011 012 013 014 015 016 017\n018 019 020"
        );
    }
    #[test]
    fn split_ppm_long_lines() {
        let c1 = Color::new(1.0, 0.8, 0.6);
        let c = (0..10).fold(Canvas::new(10, 2), |c, x| {
            (0..2).fold(c, |c, y| c.write_pixel(x, y, c1))
        });
        let ppm = c.to_ppm();
        let ppm_lines: Vec<&str> = ppm.split('\n').collect();
        assert_eq!(
            ppm_lines[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            ppm_lines[4],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            ppm_lines[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            ppm_lines[6],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }
    #[test]
    fn ppm_ends_with_newline() {
        let c = Canvas::new(3, 3);
        let ppm = c.to_ppm();
        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }
}
