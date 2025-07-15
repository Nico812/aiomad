use cgmath::InnerSpace;
use std::collections::HashSet;
use winit::event::ElementState;
use winit::keyboard::KeyCode;

use crate::GraphicsState;
use crate::graphics::camera::Camera;

pub struct InputState {
    current: HashSet<KeyCode>,
    previous: HashSet<KeyCode>,
    character_movement: CharacterMovement,
}

#[derive(Clone, Copy)]
pub struct CharacterMovement {
    pub moving: bool,
    pub direction: [f32; 2],
}

impl CharacterMovement {
    pub fn new() -> Self {
        Self {
            moving: false,
            direction: [0.0, 0.0],
        }
    }
}

impl InputState {
    pub fn new() -> Self {
        Self {
            current: HashSet::new(),
            previous: HashSet::new(),
            character_movement: CharacterMovement::new(),
        }
    }

    pub fn get_character_movement(&self) -> CharacterMovement {
        self.character_movement
    }

    pub fn update_key(&mut self, key: KeyCode, state: ElementState, repeat: bool) {
        if repeat {
            return;
        };
        match state {
            ElementState::Pressed => {
                self.current.insert(key);
                println!("Key: {:?}, State: {:?}", key, state);
            }
            ElementState::Released => {
                self.current.remove(&key);
            }
        }
    }

    fn end_frame(&mut self) {
        self.previous = self.current.clone();
    }

    fn is_pressed(&self, key: KeyCode) -> bool {
        self.current.contains(&key)
    }

    fn is_just_pressed(&self, key: KeyCode) -> bool {
        self.current.contains(&key) && !self.previous.contains(&key)
    }

    fn is_released(&self, key: KeyCode) -> bool {
        !self.current.contains(&key) && self.previous.contains(&key)
    }

    pub fn do_your_job(
        &mut self,
        graphics_state: &mut GraphicsState,
        camera: &mut Camera,
        running: &mut bool,
    ) {
        // Others
        if self.is_just_pressed(KeyCode::KeyM) {
            if *graphics_state == GraphicsState::WORLD_MAP {
                *graphics_state = GraphicsState::CHUNK_MAP
            } else if *graphics_state == GraphicsState::CHUNK_MAP {
                *graphics_state = GraphicsState::WORLD_MAP
            };
        }

        if self.is_just_pressed(KeyCode::Escape) {
            *running = false;
        }

        // Character movement
        let mut character_movement_straight = 0;
        let mut character_movement_lateral = 0;

        if self.is_pressed(KeyCode::KeyW) {
            character_movement_straight += 1;
        }
        if self.is_pressed(KeyCode::KeyS) {
            character_movement_straight -= 1;
        }
        if self.is_pressed(KeyCode::KeyD) {
            character_movement_lateral += 1;
        }
        if self.is_pressed(KeyCode::KeyA) {
            character_movement_lateral -= 1;
        }

        self.update_character_movement(
            character_movement_straight,
            character_movement_lateral,
            camera,
        );

        // Camera
        camera.movement.left = self.is_pressed(KeyCode::KeyH);
        camera.movement.right = self.is_pressed(KeyCode::KeyK);
        camera.movement.zoom = self.is_pressed(KeyCode::KeyU);
        camera.movement.unzoom = self.is_pressed(KeyCode::KeyJ);
        camera.movement.pz = self.is_pressed(KeyCode::KeyZ);
        camera.movement.mz = self.is_pressed(KeyCode::KeyI);

        self.end_frame();
    }

    fn update_character_movement(
        &mut self,
        character_movement_straight: i32,
        character_movement_lateral: i32,
        camera: &Camera,
    ) {
        //if let Ok(mut movement) = self.character_movement.write() {
        //    let mut movement_angle = movement.direction[1].atan2(movement.direction[0]);
        //    if self.movement.right {
        //        movement_angle += 0.1;
        //    } else {
        //        movement_angle += -0.1;
        //    }
        //    movement.direction[0] = movement_angle.cos();
        //    movement.direction[1] = movement_angle.sin();
        //}

        let direction = camera.get_eye_target_xy_direction_perp(true)
            * character_movement_lateral as f32
            + camera.get_eye_target_xy_direction() * character_movement_straight as f32;

        if direction.magnitude2() < 0.1 {
            self.character_movement.direction = [0.0, 0.0];
            self.character_movement.moving = false;
        } else {
            self.character_movement.direction = direction.normalize().into();
            self.character_movement.moving = true;
        }
    }
}
