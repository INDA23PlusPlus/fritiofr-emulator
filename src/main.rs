use nes_emulator::cpu::CPU;
use rand::Rng;

use std::sync::mpsc;
use std::thread;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::input::*;
use piston::window::WindowSettings;

/// ROM for the snake game.
///
/// Raw assembly with code comments can be found here:
/// https://gist.github.com/wkjagt/9043907
const SNAKE_GAME: &'static [u8] = &[
    0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9, 0x02, 0x85,
    0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85, 0x12, 0xa9, 0x0f, 0x85,
    0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xa5, 0xfe, 0x85, 0x00, 0xa5, 0xfe,
    0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4d, 0x06, 0x20, 0x8d, 0x06, 0x20, 0xc3,
    0x06, 0x20, 0x19, 0x07, 0x20, 0x20, 0x07, 0x20, 0x2d, 0x07, 0x4c, 0x38, 0x06, 0xa5, 0xff, 0xc9,
    0x77, 0xf0, 0x0d, 0xc9, 0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0, 0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60,
    0xa9, 0x04, 0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85, 0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0,
    0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01, 0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04, 0x85, 0x02,
    0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05, 0xa9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x06,
    0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00, 0xc5, 0x10, 0xd0, 0x0d, 0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07,
    0xe6, 0x03, 0xe6, 0x03, 0x20, 0x2a, 0x06, 0x60, 0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06,
    0xb5, 0x11, 0xc5, 0x11, 0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c,
    0x35, 0x07, 0x60, 0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02,
    0x4a, 0xb0, 0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9,
    0x20, 0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28, 0x60, 0xe6,
    0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10, 0xb0,
    0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c, 0x60, 0xc6, 0x10, 0xa5, 0x10, 0x29,
    0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35, 0x07, 0xa0, 0x00, 0xa5, 0xfe, 0x91, 0x00, 0x60,
    0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10, 0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10, 0x60, 0xa2, 0x00, 0xea,
    0xea, 0xca, 0xd0, 0xfb, 0x60,
];

#[derive(Copy, Clone)]
struct NesMsg {
    screen_state: [u8; 32 * 32],
    game_over: bool,
}

#[derive(Copy, Clone)]
struct WinMsg {
    key: u8,
}

/// This function is used to get the latest message from a channel.
/// It will flush the channel and return the latest message.
/// If there are no messages, it will return None.
fn get_latest_message<T>(rx: &mpsc::Receiver<T>) -> Option<T> {
    let mut latest = None;
    while let Ok(msg) = rx.try_recv() {
        latest = Some(msg);
    }
    latest
}

fn main() {
    let (tx_nes, rx_nes) = mpsc::channel();
    let (tx_win, rx_win) = mpsc::channel();

    // The NES thread.
    // This thread runs the NES CPU and sends the screen state to the main thread.
    thread::spawn(move || {
        let mut rng = rand::thread_rng();

        let mut cpu = CPU::new();
        cpu.load(SNAKE_GAME.to_vec());
        // Since program execution will start from whatever is in memory location 0xFFFC, we need
        // to set that to the start of the program. This snake game differs from other NES games
        // where execution starts at 0x8000.
        cpu.mem_write_u16(0xFFFC, 0x0600);
        // Send the reset signal to the CPU signaling that a cartridge has been inserted.
        cpu.reset();

        cpu.run_with_callback(|cpu| {
            // This snake game requires us to insert a random number every step at this memory
            // location
            // This is just a unique quirk with this particular game and not a general NES thing.
            cpu.mem_write(0xfe, rng.gen_range(1, 16));

            // This checks if theres been any key presses and if so, inserts the key into the
            // memory. The snake game reads from this memory location to get the key presses.
            get_latest_message(&rx_win).map(|msg: WinMsg| {
                cpu.mem_write(0xff, msg.key);
            });

            // This snake game works differently from other NES games in that it doesn't use the
            // PPU to draw to the screen. Instead, it writes the screen to memory.
            //
            // This is kind of a hack since this loop will only work for this particular game.
            let mut screen_state = [0 as u8; 32 * 32];
            for x in 0..32 {
                for y in 0..32 {
                    let i = 0x200 + x + y * 32;
                    let color_idx = cpu.mem_read(i as u16);
                    screen_state[x + y * 32] = color_idx;
                }
            }
            tx_nes
                .send(NesMsg {
                    screen_state,
                    game_over: false,
                })
                .unwrap();

            std::thread::sleep(std::time::Duration::new(0, 70_000));
        });

        tx_nes
            .send(NesMsg {
                screen_state: [0 as u8; 32 * 32],
                game_over: true,
            })
            .unwrap();
    });

    // Boilerplate code for the window.
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("NES Snake", [320, 320])
        .samples(1)
        .graphics_api(opengl)
        .exit_on_esc(false)
        .resizable(false)
        .build()
        .unwrap();
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    // The main event loop for the window.
    while let Some(e) = events.next(&mut window) {
        // Receive the latest message from the NES thread.
        let msg = get_latest_message(&rx_nes);

        // If the game is over, break out of the loop.
        if let Some(msg) = msg {
            if msg.game_over {
                break;
            }
        }

        if let Some(args) = e.render_args() {
            use graphics::*;

            // Draws the screen state to the window. This code is specific for this particular
            // snake game. This wont work for other roms.
            if let Some(msg) = msg {
                gl.draw(args.viewport(), |c, gl| {
                    let screen_state = msg.screen_state;
                    clear([0.0, 0.0, 0.0, 1.0], gl);

                    for x in 0..32 {
                        for y in 0..32 {
                            let color_idx = screen_state[x + y * 32];
                            let color = color(color_idx);

                            rectangle(
                                color,
                                [x as f64 * 10.0, y as f64 * 10.0, 10.0, 10.0],
                                c.transform,
                                gl,
                            );
                        }
                    }
                });
            }
        }

        // If a key is pressed, send the key to the NES thread.
        // Since this snake game is made to work with only the NES cpu this code is specific to
        // this game and wont work for other roms.
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W => {
                    tx_win.send(WinMsg { key: 0x77 }).unwrap();
                }
                Key::S => {
                    tx_win.send(WinMsg { key: 0x73 }).unwrap();
                }
                Key::A => {
                    tx_win.send(WinMsg { key: 0x61 }).unwrap();
                }
                Key::D => {
                    tx_win.send(WinMsg { key: 0x64 }).unwrap();
                }
                _ => {}
            }
        }
    }
}

/// Mapping the snake game's color palette to a format that piston can understand.
fn color(byte: u8) -> [f32; 4] {
    match byte {
        // Black
        0 => [0.0, 0.0, 0.0, 1.0],
        // White
        1 => [1.0, 1.0, 1.0, 1.0],
        // Gray
        2 | 9 => [0.5, 0.5, 0.5, 1.0],
        // Red
        3 | 10 => [1.0, 0.0, 0.0, 1.0],
        // Green
        4 | 11 => [0.0, 1.0, 0.0, 1.0],
        // Blue
        5 | 12 => [0.0, 0.0, 1.0, 1.0],
        // Yellow
        6 | 13 => [1.0, 0.0, 1.0, 1.0],
        // Magenta
        7 | 14 => [1.0, 1.0, 0.0, 1.0],
        // Cyan
        _ => [0.0, 1.0, 1.0, 1.0],
    }
}
