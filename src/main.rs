use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use game_of_life::World;

pub mod game_of_life;


const FRAME_RATE: usize = 12;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

pub const GAME_H: usize = 50;
pub const GAME_W: usize = 50;
pub const GAME_TOTAL: usize = GAME_H * GAME_W;

const ALIVE_COLOR: u32 = 0x00ffffff;
const DEAD_COLOR: u32 = 0x00000000;

fn main() {
    let mut game_paused: bool = true;

    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Game of Life - Enter to pause / ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Unable to create the window");

    window.set_target_fps(FRAME_RATE);


    // Initialize world and render it
    let mut world = World::init();
    render_world(&mut buffer, &world, WIDTH, HEIGHT);

    // Game loop
    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            game_paused = !game_paused;
        }

        // If the game is not paused, then apply game of life update
        if !game_paused {

            // Update World
            world.update();

            // Render Grid
            render_world(&mut buffer, &world, WIDTH, HEIGHT);

        } else {

            // Write in the buffer on mouse click
            if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
                // Allow only if game paused
                let mut needs_redraw = false;

                if window.get_mouse_down(MouseButton::Left) {
                    write_world_cell_at_mouse_position(&mut world, true, x, y, WIDTH, HEIGHT);
                    needs_redraw = true;
                }

                if window.get_mouse_down(MouseButton::Right) {
                    write_world_cell_at_mouse_position(&mut world, false, x, y, WIDTH, HEIGHT);
                    needs_redraw = true;
                }

                if needs_redraw { render_world(&mut buffer, &world, WIDTH, HEIGHT); }
            }
        }

        

        // We unwrap here as we want this code to exit if it fails
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}


fn write_world_cell_at_mouse_position(
    world: &mut World,
    set_alive: bool,
    mouse_x: f32,
    mouse_y: f32,
    screen_w: usize,
    screen_h: usize
) {
    let ratio_w = screen_w / GAME_W;
    let ratio_h = screen_h / GAME_H;
    let cell_x = mouse_x as usize / ratio_w;
    let cell_y = mouse_y as usize / ratio_h;
    let cell_index = (cell_y * GAME_W) + cell_x;
    world.set_cell(cell_index, set_alive);
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

