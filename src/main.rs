use rand::Rng;
use std::{thread, time};

// use termion::event::Key;
//use std::io::stdin;
// use rustfft::{FftPlanner, num_complex::Complex};

const FOREGROUND_BLACK:   &str = "\x1b[30m";
const FOREGROUND_RED:     &str = "\x1b[31m";
const FOREGROUND_GREEN:   &str = "\x1b[32m";
const FOREGROUND_YELLOW:  &str = "\x1b[33m";
const FOREGROUND_BLUE:    &str = "\x1b[34m";
const FOREGROUND_MAGENTA: &str = "\x1b[35m";
const FOREGROUND_CYAN:    &str = "\x1b[36m";
const FOREGROUND_WHITE:   &str = "\x1b[37m";
const BACKGROUND_BLACK:   &str = "\x1b[40m";
const BACKGROUND_RED:     &str = "\x1b[41m";
const BACKGROUND_GREEN:   &str = "\x1b[42m";
const BACKGROUND_YELLOW:  &str = "\x1b[43m";
const BACKGROUND_BLUE:    &str = "\x1b[44m";
const BACKGROUND_MAGENTA: &str = "\x1b[45m";
const BACKGROUND_CYAN:    &str = "\x1b[46m";
const BACKGROUND_WHITE:   &str = "\x1b[47m";
const RESET_COLOR:        &str = "\x1b[0m";
const HIDE_CURSOR:        &str = "\x1b[?25l";
const COLOR_SCALE:     &[&str] = &[FOREGROUND_GREEN,
                                   FOREGROUND_YELLOW,
                                   FOREGROUND_RED];

fn print_frame_depr(bars: &[u32], max_height: u32) {
    for i in 0..max_height {
        for j in 0..bars.len() {
            if bars[j] <= (max_height as u32)-i {
                print!(" ");
            } else {
                print!("{}*", FOREGROUND_GREEN);
            }
        }
        print!("\n    ");
    }
    for _i in 0..bars.len() {
        print!("-");
    }
    print!("\n    ");
    print!("    0Hz        10Hz        100Hz        1kHz        10kHz        100kHz    ");
    print!("\n");
}

fn build_frame(bars: &[u32], max_height: u32) -> String {
    let mut frame: String = "".to_string();
    let mut color:   &str;

    frame.push_str(&HIDE_CURSOR);

    for i in 0..max_height {
        if i < max_height/3 {
            color = COLOR_SCALE[2]
        } else if i < max_height*2/3 {
            color = COLOR_SCALE[1]
        } else {
            color = COLOR_SCALE[0]
        }
        for j in 0..bars.len() {
            if bars[j] <= (max_height as u32)-i {
                frame.push_str(" ");
            } else {
                frame.push_str(&format!("{}*", color).to_string());
            }
        }
        frame.push_str("\n    ");
    }
    frame.push_str(&RESET_COLOR.to_string());
    for _i in 0..bars.len() {
        frame.push_str("-");
    }
    frame.push_str("\n    ");
    frame.push_str("    0Hz        10Hz        100Hz        1kHz        10kHz        100kHz    ");
    frame.push_str("\n    ");
    return frame;
}

fn print_frame(frame: String) {
    print!("{}", frame);
}

//struct bars {
//    let bars: Vec<Vec<u32>>;
//}

fn main() {
//    let mut planner = FftPlanner::new();
//    let fft = planner.plan_fft_forward(1234);
//
//    let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1234];
//    fft.process(&mut buffer);
//    let stdin = stdin();
//        for evt in stdin.keys() {
//            match evt {
//                Key::Char('q') => println!("Nice!"),
//            }
//        }

    const BAR_WIDTH:      usize = 75;
    const MAX_BAR_HEIGHT: u32   = 25;
    const FRAME_TIME:     u64   = 16_667; //16_667; // us
    const DATA_TIME:      u64   = 300_000; // us

    let frame_time = time::Duration::from_micros(FRAME_TIME);
    let data_time = time::Duration::from_micros(DATA_TIME);
    let mut data_bars: [u32; BAR_WIDTH] = [0 as u32; BAR_WIDTH];
    let mut current_bars: [u32; BAR_WIDTH] = [0 as u32; BAR_WIDTH];

    print_frame(build_frame(&current_bars, MAX_BAR_HEIGHT));

    for _t in 0..100 {
        let data_instant = time::Instant::now();

        while data_instant.elapsed() < data_time {
            let frame_instant = time::Instant::now();

            for i in 0..current_bars.len() {
            // TODO: Maybe interp from one state to the next within a certain
            //       number of frames instead (Variable speed)
                if current_bars[i] < data_bars[i] {
                    if data_bars[i] != 0 && current_bars[i] < data_bars[i] - 1 {
                        current_bars[i] += 2;
                    } else {
                        current_bars[i] += 1;
                    }
                } else if current_bars[i] > data_bars[i] {
                    if data_bars[i] != MAX_BAR_HEIGHT && current_bars[i] > data_bars[i] + 1 {
                        current_bars[i] -= 2;
                    } else {
                        current_bars[i] -= 1;
                    }
                }
            }

            print!("{esc}c", esc = 27 as char);
            //print!("\x1B[2J\n");
            print_frame(build_frame(&current_bars, MAX_BAR_HEIGHT));
            if frame_instant.elapsed() <= frame_time {
                thread::sleep(frame_time - frame_instant.elapsed());
            }
        }

        for i in 0..data_bars.len() {
            data_bars[i] = rand::thread_rng().gen_range(0..MAX_BAR_HEIGHT);
        }

        thread::sleep(frame_time);
    }
}
