use rand::Rng;
use std::{thread, time};

// use termion::event::Key;
//use std::io::stdin;
// use rustfft::{FftPlanner, num_complex::Complex};

fn print_frame_depr(bars: &[u32], max_height: u32) {
    for i in 0..max_height {
        for j in 0..bars.len() {
            if bars[j] <= (max_height as u32)-i {
                print!(" ");
            } else {
                print!("*");
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
    for i in 0..max_height {
        for j in 0..bars.len() {
            if bars[j] <= (max_height as u32)-i {
                frame.push_str(" ");
            } else {
                frame.push_str("*");
            }
        }
        frame.push_str("\n    ");
    }
    for _i in 0..bars.len() {
        frame.push_str("-");
    }
    frame.push_str("\n    ");
    frame.push_str("    0Hz        10Hz        100Hz        1kHz        10kHz        100kHz    ");
    frame.push_str("\n");
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

    const BAR_WIDTH: usize = 75;
    const MAX_BAR_HEIGHT: u32 = 25;
    const FRAME_TIME: u64 = 16_667; //16_667; // us
    const DATA_TIME: u64 = 300_000; // us

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
