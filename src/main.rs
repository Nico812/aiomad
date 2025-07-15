mod constants;
mod game;
mod graphics;
mod input;

use std::{
    sync::{Arc, Mutex},
    thread, time,
};

use input::{CharacterMovement, InputState};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use game::game::{Game, GameExports, GameState};
use graphics::graphics::{Graphics, GraphicsState};

struct Aiomad {
    graphics: Option<Arc<Mutex<Graphics>>>,
    draw_thread: Option<thread::JoinHandle<()>>,
    game: Arc<Mutex<Game>>,
    game_thread: Option<thread::JoinHandle<()>>,
    input_state: Arc<Mutex<input::InputState>>,
    game_exports: Arc<Mutex<game::game::GameExports>>,
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let game = Game::new();
    let init_game_exports = game.get_exports();

    let mut aiomad = Aiomad {
        graphics: None,
        draw_thread: None,
        game: Arc::new(Mutex::new(game)),
        game_thread: None,
        input_state: Arc::new(Mutex::new(InputState::new())),
        game_exports: Arc::new(Mutex::new(init_game_exports)),
    };

    aiomad.game_thread = Some(thread::spawn({
        let game = Arc::clone(&aiomad.game);
        let game_exports = Arc::clone(&aiomad.game_exports);
        let input_state = Arc::clone(&aiomad.input_state);

        move || {
            let mut running = true;
            let mut delta_time = time::Instant::now();
            while running {
                let mut character_movement = CharacterMovement::new();

                if let Ok(input_state) = input_state.lock() {
                    character_movement = input_state.get_character_movement();
                }
                if let Ok(mut game) = game.lock() {
                    if game.update(character_movement) == GameState::DEAD {
                        running = false;
                    };
                    if let Ok(mut game_exports) = game_exports.lock() {
                        *game_exports = game.get_exports();
                    }
                }

                if delta_time.elapsed() <= time::Duration::from_millis(constants::THICK_RATE_MILLIS)
                {
                    thread::sleep(
                        time::Duration::from_millis(constants::THICK_RATE_MILLIS)
                            - delta_time.elapsed(),
                    );
                }
                delta_time = time::Instant::now();
            }
        }
    }));

    event_loop.run_app(&mut aiomad).unwrap();
}

impl Aiomad {
    fn terminate(&mut self, event_loop: &ActiveEventLoop) {
        println!("Terminating...");

        if let Some(thread) = self.draw_thread.take() {
            thread.join().unwrap();
            println!("Thread draw_thread catched");
        }
        if let Ok(mut game) = self.game.lock() {
            game.terminate();
            println!("Game terminated");
        }
        if let Some(thread) = self.game_thread.take() {
            thread.join().unwrap();
            println!("Thread game_thread catched");
        }
        if let Some(graphics) = self.graphics.take() {
            drop(graphics);
            println!("Graphics dropped to help Mr. SegFault");
        }
        println!("Exit called from thread: {:?}", std::thread::current().id());
        event_loop.exit();
    }

    fn frontend_loop(&mut self) {
        let graphics = Arc::clone(self.graphics.as_ref().unwrap());
        let game_exports = Arc::clone(&self.game_exports);
        let input_state = Arc::clone(&self.input_state);

        self.draw_thread = Some(thread::spawn({
            move || {
                let mut running = true;
                let mut frame_count = 0;
                let mut fps_timer = time::Instant::now();
                let mut delta_time = time::Instant::now();
                let mut game_exports_copy = GameExports::new();
                while running {
                    if let Ok(mut graphics) = graphics.lock() {
                        if let Ok(game_exports) = game_exports.lock() {
                            game_exports_copy = game_exports.clone();
                        }
                        Self::redraw(&game_exports_copy, &mut graphics);

                        if let Ok(mut input_state) = input_state.lock() {
                            let (mut state, mut camera) = {
                                let graphics_ref = &mut *graphics;
                                (&mut graphics_ref.state, &mut graphics_ref.camera)
                            };
                            input_state.do_your_job(&mut state, &mut camera, &mut running);
                        }
                    }

                    if fps_timer.elapsed() >= time::Duration::from_secs(1) {
                        println!("FPS: {}", frame_count);
                        frame_count = 0;
                        fps_timer = time::Instant::now();
                    }
                    if delta_time.elapsed()
                        <= time::Duration::from_millis(constants::MAX_FRAMERATE_MILLIS)
                    {
                        thread::sleep(
                            time::Duration::from_millis(constants::MAX_FRAMERATE_MILLIS)
                                - delta_time.elapsed(),
                        );
                    }
                    delta_time = time::Instant::now();
                    frame_count += 1;
                }
            }
        }));
    }

    fn redraw(positions: &game::game::GameExports, graphics: &mut Graphics) {
        match graphics.update_all(&positions) {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {}
            Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                println!("OutOfMemory");
            }
            Err(wgpu::SurfaceError::Timeout) => {
                println!("Surface timeout")
            }
        };
    }
}

impl ApplicationHandler for Aiomad {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Airomad"))
                .unwrap(),
        );
        if let Ok(game) = self.game.lock() {
            let graphics = pollster::block_on(Graphics::new(window.clone(), game.get_for_init()));
            self.graphics = Some(Arc::new(Mutex::new(graphics)));
        }

        self.frontend_loop();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => self.terminate(event_loop),

            WindowEvent::RedrawRequested => {
                println!("I DONT CARE ABOUT YOUR STUPID REDRAWREQUEST");
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state,
                        repeat,
                        ..
                    },
                ..
            } => {
                if let Ok(mut input_state) = self.input_state.lock() {
                    input_state.update_key(key, state, repeat);
                }
                if key == winit::keyboard::KeyCode::Escape {
                    self.terminate(event_loop);
                }
            }

            WindowEvent::Resized(size) => {
                if let Ok(mut graphics) = self.graphics.as_ref().unwrap().lock() {
                    graphics.resize(size);
                }
            }
            _ => (),
        }
    }
}
