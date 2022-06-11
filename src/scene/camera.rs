use crate::{math::vector, Color, Ray, Vec3, World};

pub struct Camera {
    pos: Vec3,
    up: Vec3,
    right: Vec3,
    forward: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, vertical_fov_degrees: f32, pos: Vec3, look_at: Vec3) -> Camera {
        let rad_angle = std::f32::consts::PI * vertical_fov_degrees / 180.0;
        let h = 2.0 * f32::tan(rad_angle / 2.0);
        let w = h * aspect_ratio;
        let normalized_dir = (look_at - pos).normalized();
        let up = vector::UP;
        let orthogonal_up = (up - (&up * &normalized_dir) * normalized_dir).normalized();
        let right = orthogonal_up.cross(&normalized_dir).normalized();

        Camera {
            pos: pos,
            up: h * orthogonal_up,
            right: w * right,
            forward: normalized_dir,
        }
    }

    pub fn color(&self, x: f32, y: f32, world: &World) -> Color {
        let view_x = 0.5 - x;
        let view_y = 0.5 - y;
        let focal_center = self.forward;
        let x_delta = view_x * self.right;
        let y_delta = view_y * self.up;
        let mut ray = Ray {
            pos: self.pos,
            dir: (focal_center + y_delta + x_delta).normalized(),
        };

        return Ray::color(&mut ray, world, 10);
    }
}
