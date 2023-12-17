// #![windows_subsystem = "windows"] // (Hides console on Windows when uncommented)
use raylib::prelude::*;
use rand::Rng;

/*  These are my personal settings, performance isn't guaranteed. (This was written pre-optimization, 
    now the random walk is only rendered once and stored so performance should remain consistent)
    (Only slows down when initially generating and rendering random walks)
    (150x performance increase from old version (which rendered all walks every frame) using rx 580 gpu & 3rd gen i5 cpu)
*/
const GRID_STEP: i32 = 4; // Recommended value of 5. (Determines gap between grid points in px)
const RUNS: i32 = 5; // Recommended value of 4 or 5. (How many separate walks to do, 1..=4 walk will be coloured, 5.. will be white)
const STEPS: i32 = 100000; // Recommended value of 10,000-25,000. (How many "steps" each random walk should do)

const COLOR_GREEN: Color = Color::GREEN;
const COLOR_BLUE: Color = Color::BLUE;
const COLOR_YELLOW: Color = Color::YELLOW;
const COLOR_RED: Color = Color::RED;
const COLOR_WHITE: Color = Color::WHITE;

fn get_walk_colour(index: usize) -> Color {
    match index {
        0 => COLOR_GREEN,
        1 => COLOR_BLUE,
        2 => COLOR_YELLOW,
        3 => COLOR_RED,
        _ => COLOR_WHITE,
    }
}

fn generate_random_walks() -> Vec<Vec<i32>> {
    let mut return_vector: Vec<Vec<i32>> = Vec::new();

    for _ in 0..RUNS {
        let mut step_vector: Vec<i32> = Vec::new();

        for _n in 0..STEPS {
            let dir = rand::thread_rng().gen_range(0..=3);
            step_vector.push(dir);
        }

        return_vector.push(step_vector);
    }

    return_vector
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .vsync()
        .title("Random Walk")
        .resizable()
        .build();

    let mut random_walks: Vec<Vec<i32>> = generate_random_walks();

    let mut redraw: bool = true;
    let mut texture: Texture2D = rl
        .load_texture_from_image(&thread, &Image::gen_image_color(1920, 1080, Color::BLACK))
        .expect("Failed to load dynamic texture");

    let mut frametime: f32 = 0.0;
    let mut prev_mouse_x: i32 = 0;
    let mut prev_mouse_y: i32 = 0;

    let mut offset_x: i32 = 0;
    let mut offset_y: i32 = 0;

    println!("INFO: CURRENT CONFIG");
    println!("INFO: GRID_STEP = {:?}PX", GRID_STEP);
    println!("INFO: RUNS = {:?}", RUNS);
    println!("INFO: STEPS = {:?}", STEPS);
    println!("INFO: TOTAL STEPS & LINE RENDERS = {:?}", RUNS * STEPS);

    while !rl.window_should_close() {
        let fps: u32 = rl.get_fps();
        let delta_time: f32 = rl.get_frame_time();
        let screen_width: i32 = rl.get_screen_width();
        let screen_height: i32 = rl.get_screen_height();
        let enter_pressed: bool = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
        let mouse_x: i32 = rl.get_mouse_x();
        let mouse_y: i32 = rl.get_mouse_y();

        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            offset_x += mouse_x - prev_mouse_x;
            offset_y += mouse_y - prev_mouse_y;
        }
        offset_x = offset_x.clamp(-((texture.width - screen_width) / 2), (texture.width - screen_width) / 2);
        offset_y = offset_y.clamp(-((texture.height - screen_height) / 2), (texture.height - screen_height) / 2);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if enter_pressed == true {
            random_walks = generate_random_walks();
            redraw = true;
        }

        if redraw == true {
            // Temporary image for initial draw.
            let mut img: Image = Image::gen_image_color(texture.width, texture.height, Color::BLACK);

            for (i, walk) in random_walks.iter().enumerate() {
                let walk_colour: Color = get_walk_colour(i);

                let mut location_x: i32 = texture.width / 2;
                let mut location_y: i32 = texture.height / 2;

                for n in walk {
                    match *n {
                        0 => {
                            for n in 1..=GRID_STEP {
                                // Draw each pixel to the dynamic texture individually as draw_line and draw_rectangle don't draw correctly.
                                let _ = &img.draw_pixel(location_x + n, location_y, walk_colour);
                            }
                            location_x += GRID_STEP;
                        },
                        1 => {
                            for n in 1..=GRID_STEP {
                                let _ = &img.draw_pixel(location_x, location_y + n, walk_colour);
                            }
                            location_y += GRID_STEP;
                        },
                        2 => {
                            for n in 1..=GRID_STEP {
                                let _ = &img.draw_pixel(location_x - n, location_y, walk_colour);
                            }
                            location_x -= GRID_STEP;
                        },
                        3 => {
                            for n in 1..=GRID_STEP {
                                let _ = &img.draw_pixel(location_x, location_y - n, walk_colour);
                            }
                            location_y -= GRID_STEP;
                        },
                        _ => continue
                    };
                }
            }

            // update_texture only takes a u8 slice, so we convert the image data (Color vector) to a Vector of rgba u8 values.
            let mut pixel_data: Vec<u8> = Vec::new();
            for color in img.get_image_data().to_vec().iter() {
                pixel_data.push(color.r);
                pixel_data.push(color.g);
                pixel_data.push(color.b);
                pixel_data.push(color.a);
            };

            texture.update_texture(pixel_data.as_slice());
            redraw = false;
        }

        d.draw_texture(&texture, -((&texture.width - screen_width)/2) + offset_x, -((&texture.height - screen_height)/2) + offset_y, Color::WHITE);

        if frametime >= 0.5 && frametime <= 1.0 {
            d.draw_text("PRESS ENTER TO REGENERATE", (screen_width / 2) - (measure_text("PRESS ENTER TO REGENERATE", 35) / 2), (screen_height / 2)+(screen_height / 4), 35, Color::PURPLE);
        } else if frametime > 1.0 {
            frametime = 0.0;
        }

        d.draw_text(&format!("X: {:?}, Y: {:?}", mouse_x, mouse_y), 50, 12, 20, COLOR_GREEN);
        d.draw_text(&format!("{:?}", fps), 12, 12, 20, COLOR_GREEN);
        
        frametime += delta_time;
        prev_mouse_x = mouse_x;
        prev_mouse_y = mouse_y;
    }
}
