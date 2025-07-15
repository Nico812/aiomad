use crate::constants::OPENGL_TO_WGPU_MATRIX;
use cgmath::{InnerSpace, Matrix4, Point3, Vector2, Vector3};

pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    pub aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,

    pub movement: CameraMovement,
    alpha: f32,
}

pub struct CameraMovement {
    pub left: bool,
    pub right: bool,
    pub zoom: bool,
    pub unzoom: bool,
    pub pz: bool,
    pub mz: bool,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraMovement {
    pub fn new() -> Self {
        Self {
            left: false,
            right: false,
            zoom: false,
            unzoom: false,
            pz: false,
            mz: false,
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            eye: (15.0, 0.0, 10.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vector3::unit_z(),
            aspect: 3 as f32 / 2 as f32,
            fovy: 20.0,
            znear: 0.1,
            zfar: 100.0,
            movement: CameraMovement::new(),
            alpha: 0.0,
        }
    }

    pub fn get_proj_matrix(&self) -> Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn get_camera_uniform(&self) -> CameraUniform {
        CameraUniform {
            view_proj: self.get_proj_matrix().into(),
            view_position: self.eye.to_homogeneous().into(),
        }
    }

    pub fn update(&mut self, target: [f32; 3]) {
        self.alpha = self.alpha % (2.0 * std::f32::consts::PI);
        let mut rx = self.eye.x - self.target.x;
        let mut ry = self.eye.y - self.target.y;
        let r = f32::sqrt(rx * rx + ry * ry);

        self.target = target.into();

        let targetx = self.target.x;
        let targety = self.target.y;

        self.eye.x = rx + targetx;
        self.eye.y = ry + targety;

        if self.movement.left || self.movement.right {
            if self.movement.right {
                self.alpha += 0.1;
            } else {
                self.alpha += -0.1;
            }
            self.eye.x = r * self.alpha.cos() + targetx;
            self.eye.y = r * self.alpha.sin() + targety;

            rx = self.eye.x - self.target.x;
            ry = self.eye.y - self.target.y;
        }
        if self.movement.zoom || self.movement.unzoom {
            let mut factor = 0.95;
            if self.movement.unzoom {
                factor = 1.05;
            }
            self.eye.x = rx * factor + targetx;
            self.eye.y = ry * factor + targety;
        }
        if self.movement.pz {
            self.eye.z *= 1.05;
        } else if self.movement.mz {
            self.eye.z *= 0.95;
        }
    }

    pub fn get_eye_target_xy_direction(&self) -> Vector2<f32> {
        return Vector2::new((self.target - self.eye).x, (self.target - self.eye).y).normalize();
    }

    pub fn get_eye_target_xy_direction_perp(&self, negative: bool) -> Vector2<f32> {
        let v = Vector2::new((self.eye - self.target).y, -(self.eye - self.target).x).normalize();
        if negative { -v } else { v }
    }
}
