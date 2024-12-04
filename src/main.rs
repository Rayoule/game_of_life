use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use game_of_life::World;
use text::Text;

pub mod game_of_life;
pub mod text;


const BASE_FRAME_RATE: usize = 30;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

pub const GAME_H: usize = 100;
pub const GAME_W: usize = 100;
pub const GAME_TOTAL: usize = GAME_H * GAME_W;

const ALIVE_COLOR: u32 = 0x00ffffff;
const DEAD_COLOR: u32 = 0x00000000;


fn main() {
    let mut game_paused: bool = true;

    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Game of Life",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Unable to create the window");

    let mut target_fps = BASE_FRAME_RATE;
    window.set_target_fps(target_fps);

    // Init UI
    let press_space_exit_text = Text::new(WIDTH, HEIGHT, 2);
    let clic_text = Text::new(WIDTH, HEIGHT, 2);
    let target_fps_text = Text::new(WIDTH, HEIGHT, 2);


    // Initialize world and render it
    let mut world = World::init();
    render_world(&mut buffer, &world, WIDTH, HEIGHT);


    // setup variables to make sure mouse clicks toggles once
    let mut prev_right_click = false;
    let mut prev_left_click = false;
    let mut prev_mouse_pos: (f32, f32) = (0.0, 0.0);

    // Game loop
    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Handles the pause
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            game_paused = !game_paused;
        }
        if window.get_mouse_down(MouseButton::Right) {
            if !prev_right_click {
                game_paused = !game_paused;
            }
            prev_right_click = true;
        } else {
            prev_right_click = false;
        }

        // Adjust FPS
        if let Some((_, scroll_y)) = window.get_scroll_wheel() {
            if scroll_y > 0. {
                target_fps = (target_fps + 1).min(300);
            } else if scroll_y < 0. {
                target_fps = (target_fps - 1).max(1);
            }
            window.set_target_fps(target_fps);
        }


        // If the game is not paused, then update the world
        if !game_paused {

            world.update();
            render_world(&mut buffer, &world, WIDTH, HEIGHT);

        } else {

            // Write in the buffer on mouse click
            if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
                let mut needs_redraw = false;

                if window.get_mouse_down(MouseButton::Left) {
                    if
                        !prev_left_click
                        || (x - prev_mouse_pos.0).abs() > 2. // makes that we can drag the mouse to paint
                        || (y - prev_mouse_pos.1).abs() > 2. // makes that we can drag the mouse to paint
                    {
                        toggle_world_cell_at_mouse_position(
                            &mut world,
                            x,
                            y,
                            WIDTH,
                            HEIGHT
                        );
                        needs_redraw = true;
                    }
                    prev_left_click = true;
                } else {
                    prev_left_click = false
                }

                prev_mouse_pos = (x, y);

                if needs_redraw { render_world(&mut buffer, &world, WIDTH, HEIGHT); }
            }
        }

        
        // UI
        // Show some basic text at the bottom of the screen
        press_space_exit_text.draw(&mut buffer, (20, HEIGHT - 20), "right click: pause / esc: exit");
        target_fps_text.draw(&mut buffer, (10, 10), &format!("fps: {:02}", target_fps.to_string()));
        if game_paused {
            clic_text.draw(&mut buffer, (20, HEIGHT - 40), "Game paused -> change cells: left click");
        }

        // We unwrap here as we want this code to exit if it fails
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}


fn toggle_world_cell_at_mouse_position(
    world: &mut World,
    mouse_x: f32,
    mouse_y: f32,
    screen_w: usize,
    screen_h: usize
) {
    let ratio_w = screen_w / GAME_W;
    let ratio_h = screen_h / GAME_H;
    let cell_x = mouse_x.floor() as usize / ratio_w;
    let cell_y = mouse_y.floor() as usize / ratio_h;
    let cell_index = (cell_y * GAME_W) + cell_x;
    world.toggle_cell(cell_index);
}


fn render_world(buffer: &mut Vec<u32>, world: &World, screen_w: usize, screen_h: usize) {
    let ratio_w = screen_w / GAME_W;
    let ratio_h = screen_h / GAME_H;
    for y in 0..screen_h {
        for x in 0..screen_w {
            let cell_x = x / ratio_w;
            let cell_y = y / ratio_h;
            let cell_idx = (GAME_W * cell_y) + cell_x;
            
            let cell_state = world.is_cell_alive(cell_idx);
            buffer[(screen_w * y) + x] = match cell_state {
                true => ALIVE_COLOR,    // Cell alive
                false => DEAD_COLOR,    // Cell dead
            }
        }
    }
}

