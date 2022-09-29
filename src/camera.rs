pub struct Camera {
    pub hsize: u32,
    pub vsize: u32,
    pub field_of_view: f64,
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, field_of_view: f64) -> Camera {
        Camera {
            hsize,
            vsize,
            field_of_view,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = std::f64::consts::PI / 2.0;

        let c = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(c.hsize, hsize);
        assert_eq!(c.vsize, vsize);
        assert_eq!(c.field_of_view, field_of_view);
    }
}
