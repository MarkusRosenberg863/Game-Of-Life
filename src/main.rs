use std::{time::Duration, collections::HashMap};

use sdl2::{pixels::Color, event::Event, keyboard::Keycode, rect::Rect, mouse::{MouseButton, MouseWheelDirection}};

fn main() {
    let window_width: u32 = 1600;
    let window_height: u32 = 900;
    let mut scope = (0..window_width as i32, 0..window_height as i32);
    let mut hold_middle_mouse_button = false;
    let mut scope_drag_start_x = 0;
    let mut scope_drag_start_y = 0;

    let mut hold_left_mouse_button = false;
    let mut add_cell = true;
    
    //TODO: replace with array
    let mut board: HashMap<(i32, i32), bool> = HashMap::new();
    let mut tile_size: u32 = 25;
    let mut running = false;

    let mut frame_count: u32 = 0;

    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();

    let window = video_subsystem.window("Convey's Game Of Life", window_width, window_height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = context.event_pump().unwrap();
    'running: loop {
        frame_count += 1;

        //Event handler
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    running = !running;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    board = HashMap::new();
                    running = false;
                    tile_size = 25;
                    scope = (0..window_width as i32, 0..window_height as i32);
                    frame_count = 0;
                },
                Event::MouseWheel { direction: MouseWheelDirection::Normal , y, ..} => {
                    if y == 1 && tile_size < 100 {
                        tile_size += 2;
                    }
                    else if y == -1 && tile_size > 5{
                        tile_size -= 2;
                    }
                }
                Event::MouseButtonUp { mouse_btn: MouseButton::Middle, .. } => {
                    hold_middle_mouse_button = false;
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Middle, x, y, .. } => {
                    hold_middle_mouse_button = true;
                    scope_drag_start_x = x;
                    scope_drag_start_y = y;
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                    hold_left_mouse_button = false;
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    hold_left_mouse_button = true;
                    let i = (x + scope.0.start) / tile_size as i32;
                    let j = (y + scope.1.start) / tile_size as i32;

                    add_cell = board.get(&(i, j)).is_none();
                },
                _ => {}
            }
        }

        //adding cell if left mouse button is hold
        //TODO: not accurate placement
        if hold_left_mouse_button {
            if running {continue;}

            let x = event_pump.mouse_state().x();
            let y = event_pump.mouse_state().y();

            let i = (x + scope.0.start) / tile_size as i32;
            let j = (y + scope.1.start) / tile_size as i32;

            if add_cell {
                board.insert((i, j), true);
            }
            else {
                board.remove(&(i, j));
            }
        }

        //move scope if middle mouse button is hold
        if hold_middle_mouse_button {
            let x = event_pump.mouse_state().x();
            let y = event_pump.mouse_state().y();

            let dir_x = scope_drag_start_x - x;
            let dir_y = scope_drag_start_y - y;

            scope = ((scope.0.start + dir_x)..(scope.0.end + dir_x), (scope.1.start + dir_y)..(scope.1.end + dir_y));
            scope_drag_start_x = x;
            scope_drag_start_y = y;
        }

        //Mechanic
        //TODO: add speed changer
        if running && frames_per_second(frame_count, 10) {
            let board_state = board.clone();
            for ((x, y), _) in board_state.clone() {
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        let i = x + dr;
                        let j = y + dc;

                        let adjacent = count_adjacent(board_state.clone(), i, j);

                        if board_state.get(&(i, j)).is_some() && adjacent < 2 {
                            board.remove(&(i, j));
                        }
                        else if board_state.get(&(i, j)).is_some() && ((2..=3).contains(&adjacent)){
                            continue;
                        }
                        else if board_state.get(&(i, j)).is_some() && adjacent > 2 {
                            board.remove(&(i, j));
                        }
                        else if board_state.get(&(i, j)).is_none() && adjacent == 3 {
                            board.insert((i, j), true);
                        }
                    }
                }
            }
        }

        //Background
        canvas.set_draw_color(Color::RGB(60, 60, 60));
        canvas.clear();

        //Entities
        let offset: i32 = 1;

        //TODO: generate full board
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        for x in (0..window_width as i32).step_by(tile_size as usize) {
            for y in (0..window_height as i32).step_by(tile_size as usize) {
                let scope_offset_x: i32 = -scope.0.start % tile_size as i32;
                let scope_offset_y: i32 = -scope.1.start % tile_size as i32;

                let pos_x = x + offset + scope_offset_x;
                let pos_y = y + offset + scope_offset_y;
                let width = tile_size - offset as u32 * 2;
                let height = tile_size - offset as u32 * 2;

                canvas.fill_rect(
                    Rect::new(pos_x, pos_y, width, height)
                ).unwrap();
            }
        }

        //TODO: remove cutting off
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        for ((x, y), _) in board.clone(){
            if scope.0.contains(&(x * tile_size as i32)) && scope.1.contains(&(y * tile_size as i32)){
                let scope_offset_x: i32 = -scope.0.start;
                let scope_offset_y: i32 = -scope.1.start;

                canvas.fill_rect(
                    Rect::new(
                        x * tile_size as i32 + offset + scope_offset_x, 
                        y * tile_size as i32 + offset + scope_offset_y,
                        tile_size - offset as u32 * 2, 
                        tile_size - offset as u32 * 2)
                ).unwrap();
            }
        }
        
        canvas.present();

        if frame_count == 60 {frame_count = 0;}
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn count_adjacent(board: HashMap<(i32, i32), bool>, x0: i32, y0: i32) -> i32 {
    let mut count: i32 = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 { continue; }

            if board.get(&(x0 + dr, y0 + dc)).is_some() {
                count += 1
            }
        }
    }

    return count;
}

fn frames_per_second(frame_count: u32, frames: u32) -> bool {
    return frame_count % (60 / frames) == 0;
}