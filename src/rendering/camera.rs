use glam::{Mat4, Vec3};
use std::time::Duration;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseScrollDelta, VirtualKeyCode},
};

#[derive(Debug)]
pub struct FirstPersonCamera {
    fov_radians: f32,
    aspect_ratio: f32,
    z_far: f32,
    z_near: f32,
    pub position: Vec3,
    // horizontal rotation
    yaw_radians: f32,
    // vertical rotation
    pitch_radians: f32,
}

impl FirstPersonCamera {
    pub fn new(
        position: Vec3,
        yaw_degrees: f32,
        pitch_degrees: f32,
        fov_degrees: f32,
        aspect_ratio: f32,
        z_far: f32,
        z_near: f32,
    ) -> Self {
        Self {
            fov_radians: fov_degrees.to_radians(),
            aspect_ratio,
            z_far,
            z_near,
            position,
            yaw_radians: yaw_degrees.to_radians(),
            pitch_radians: pitch_degrees.to_radians(),
        }
    }

    pub fn set_aspect_ration(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        println!("new aspect_ratio: {}", aspect_ratio);
    }

    pub fn view_matrix(&self) -> Mat4 {
        let (sin_pitch, cos_pitch) = self.pitch_radians.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw_radians.sin_cos();

        // TODO: should we use right handed or left handed?
        Mat4::look_at_rh(
            self.position,
            self.position
                + Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vec3::Y,
        )
    }

    pub fn projection_matrix(&self) -> Mat4 {
        // TODO: should we use right handed or left handed?
        Mat4::perspective_rh(self.fov_radians, self.aspect_ratio, self.z_near, self.z_far)
    }
}

#[derive(Debug)]
pub struct CameraController {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }

    pub fn handle_keyboard_event(&mut self, key: VirtualKeyCode, state: ElementState) {
        println!("keyboard key: {:?}", key);

        let amount = if state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };

        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.amount_forward = amount;
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.amount_backward = amount;
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.amount_left = amount;
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.amount_right = amount;
            }
            VirtualKeyCode::Space => {
                self.amount_up = amount;
            }
            VirtualKeyCode::LShift => {
                self.amount_down = amount;
            }
            _ => (),
        }
    }

    pub fn handle_mouse_event(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn handle_scroll_event(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 0.5,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => *scroll as f32,
        }
    }

    pub fn update(&mut self, camera: &mut FirstPersonCamera, delta_time: Duration) {
        let delta_time = delta_time.as_secs_f32();

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw_radians.sin_cos();
        let forward = Vec3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position +=
            forward * (self.amount_forward - self.amount_backward) * self.speed * delta_time;
        camera.position += right * (self.amount_right - self.amount_left) * self.speed * delta_time;

        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.
        let (pitch_sin, pitch_cos) = camera.pitch_radians.sin_cos();
        let scrollward = Vec3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        camera.position += scrollward * self.scroll * self.speed * self.sensitivity * delta_time;
        self.scroll = 0.0;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.position.y += (self.amount_up - self.amount_down) * self.speed * delta_time;

        // Rotate
        camera.yaw_radians += self.rotate_horizontal.to_radians() * self.sensitivity * delta_time;
        camera.pitch_radians += -self.rotate_vertical.to_radians() * self.sensitivity * delta_time;

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        // Keep the camera's angle from going too high/low.
        if camera.pitch_radians < -89.0_f32.to_radians() {
            camera.pitch_radians = -89.0_f32.to_radians();
        } else if camera.pitch_radians > 89.0_f32.to_radians() {
            camera.pitch_radians = 89.0_f32.to_radians();
        }
    }
}
