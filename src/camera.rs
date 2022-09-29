use crate::canvas::Canvas;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::tuple::Point;
use crate::world::World;
use rayon::prelude::*;

pub struct Camera {
    hsize: u32,
    vsize: u32,

    #[allow(dead_code)]
    field_of_view: f64,

    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    transform: Matrix4,
    inv_transform: Matrix4,
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = half_width * 2.0 / hsize as f64;

        Camera {
            hsize,
            vsize,
            field_of_view,
            half_height,
            half_width,
            pixel_size,
            transform: Matrix4::identity(4),
            inv_transform: Matrix4::identity(4),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix4) {
        self.inv_transform = transform.inverse().expect("Fail to inverse camera matrix");
        self.transform = transform;
    }

    pub fn ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        // untransformed coordinates in world space
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // camera matrix to transform the canvas point and origin
        // canvas at z = -1
        let pixel =
            Point::try_from(&self.inv_transform * Point::new(world_x, world_y, -1.0)).unwrap();
        let origin = Point::try_from(&self.inv_transform * Point::new(0.0, 0.0, 0.0)).unwrap();
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize as isize, self.vsize as isize);

        (0..self.vsize)
            .into_par_iter()
            .flat_map(|y| (0..self.hsize).into_par_iter().map(move |x| (x, y)))
            .map(|(x, y)| (x, y, self.ray_for_pixel(x, y)))
            .map(|(x, y, ray)| (x, y, world.color_at(ray)))
            .collect::<Vec<_>>()
            .iter()
            .for_each(|(x, y, color)| {
                //image.pixels[(y * image.width as u32 + x) as usize] = color;
                image.write_pixel(*x as isize, *y as isize, *color);
            });

        image
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::transformations;
    use crate::tuple::Vector;
    use crate::world::default_world;
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let c = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(c.hsize, hsize);
        assert_eq!(c.vsize, vsize);
        assert_eq!(c.field_of_view, field_of_view);
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let camera = Camera::new(125, 200, PI / 2.0);
        assert!((camera.pixel_size - 0.01).abs() < f64::EPSILON);
    }
    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let camera = Camera::new(200, 125, PI / 2.0);
        assert!((camera.pixel_size - 0.01).abs() < f64::EPSILON);
    }

    #[test]
    fn constructing_a_ray_through_center() {
        let camera = Camera::new(201, 101, PI / 2.0);
        let r = camera.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_ray_through_corner() {
        let camera = Camera::new(201, 101, PI / 2.0);
        let r = camera.ray_for_pixel(0, 0);
        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_ray_when_camera_transformed() {
        let mut camera = Camera::new(201, 101, PI / 2.0);
        camera.set_transform(
            transformations::rotation_y(PI / 4.0) * transformations::translation(0.0, -2.0, 5.0),
        );
        let r = camera.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vector::new(2f64.sqrt() / 2.0, 0.0, -2f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rendering_a_world_with_camera() {
        let world = default_world();
        let mut camera = Camera::new(11, 11, PI / 2.0);
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        camera.set_transform(transformations::view_transform(from, to, up));
        let image = camera.render(&world);
        assert_eq!(
            image.read_pixel(5, 5).unwrap(),
            Color::new(0.38066, 0.47583, 0.2855)
        );
    }
}
