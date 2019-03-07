use pancurses::*;

use std::f32::consts::PI;
use std::time::SystemTime;

fn main() {
    // let screen_width = 120;
    // let screen_height = 40;
    let window = initscr();
    nl();
    noecho();
    window.nodelay(true);
    
    let screen_width = window.get_max_x();
    let screen_height = window.get_max_y();

    let player_x = 14.7; // Player start position
    let player_y = 5.09;
    let player_a = 0.0; // player start rotation

    let map_width = 16;
    let map_height = 16;
    let field_of_view = PI / 4.0;

    let depth = 16.0;
    let speed = 5.0;

    let mut screen_buffer: Vec<char> = Vec::with_capacity((screen_width * screen_height) as usize);

    for i in 0..screen_width * screen_height {
        screen_buffer.push(' ');
    }

    let mut string_map = "".to_string();
    string_map += "################";
    string_map += "#..............#";
    string_map += "#..............#";
    string_map += "#..............#";
    string_map += "#..............#";
    string_map += "#..............#";
    string_map += "#..............#";
    string_map += "#..............#";
    string_map += "#..............#";
    string_map += "#..............#";
    string_map += "#..............#";
    string_map += "#..............#";
    string_map += "#..............#";
    string_map += "#..............#";
    string_map += "#..............#";
    string_map += "################";

    let mut map: Vec<_> = string_map.chars().collect();
    let mut time_1 = SystemTime::now();

    'gameloop: loop {
        let _elapsed = time_1.elapsed().unwrap();
        time_1 = SystemTime::now();
        

        match window.getch() {
            Some(Input::Character(q)) if q == 'q' => {
                curs_set(1);
                endwin();
                return;
            }
            Some(Input::Character(a)) if a == 'a' => {
                // player_a -= elapsed.checked_mul(speed * 0.75);
            }
            Some(Input::Character(d)) if d == 'd' => {
                // player_a += elapsed.checked_mul(speed * 0.75);
            }
            _ => {}
        }

        // Forwards && Collision
        // Backwards && Collision

        // Raycaster
        // For each column, calculate the projected ray angle into world space
        for x in 0..screen_width {
            let ray_angle = ((player_a- field_of_view) / 2.0) + ((x as f32 / screen_width as f32) * field_of_view);

            // Find distance to wall
            let step_size = 0.1;
            let mut distance_to_wall = 0.0;

            let mut hit_wall = false;
            let _boundary = false;

            let eye_x = ray_angle.sin();
            let eye_y = ray_angle.cos();

            // Cast ray from player along ray angle and test for intersection with block
            while !(hit_wall && (distance_to_wall < depth)) {
                distance_to_wall += step_size;

                let test_x = (player_x + eye_x * distance_to_wall) as i32;
                let test_y = (player_y + eye_y * distance_to_wall) as i32;

                // test if ray is out of bounds
                if test_x < 0 || test_x > map_width || test_y < 0 || test_y > map_height {
                    hit_wall = true;
                    distance_to_wall = depth;
                }
                else {
                    // Ray is in-bounds so test to see if the ray cell is a wall block
                    if map[(test_x * map_width + test_y) as usize] == '#' {

                        hit_wall = true;
                        // let mut corner_boundaries: Vec<(f32, f32)> = Vec::new();

                        // for tx in 0..2 {
                        //     for ty in 0..2 {
                        //         let vy = test_y as f32 + ty as f32- player_y;
                        //         let vx = test_x as f32 + tx as f32 - player_x;
                        //         let d = (vx * vx + vy * vy).sqrt();
                        //         let dot = (eye_x * vx / d) + (eye_y * vy / d);
                        //         corner_boundaries.push((d, dot));
                        //     }
                        // }

                        // Sort the pairs from closest to furthest.


                    }
                }
            }

            // Calculate the distance from ceiling to floor
            let ceiling = (screen_height as f32 / 2.0) - screen_height as f32 / distance_to_wall as f32;
            let floor = screen_height as f32 - ceiling;

            // Shade walls based on distance
            // Share the floor based on distance
            
            for y in 0..screen_height {
                if y as f32 <= ceiling {
                    screen_buffer[(y * screen_width + x) as usize] = ' ';
                }
                else if y as f32 > ceiling && y as f32 <= floor {
                    screen_buffer[(y * screen_width + x) as usize] = 'x';
                }
                else {
                    screen_buffer[(y * screen_width + x) as usize] = ' ';
                }

            }
        }

        window.mvaddstr(0,0, screen_buffer.iter().collect::<String>());
        window.refresh();
    }
}
