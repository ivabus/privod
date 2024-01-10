use rand::Rng;
use std::ffi::{c_char, CString};

extern "C" {
    pub fn get_status(drive_path: *const c_char) -> std::ffi::c_int;
    pub fn eject(drive_path: *const c_char);
}

fn sleep_rnd(start: u64, end: u64) {
    let sleeptime = rand::thread_rng().gen_range(start..=end);
    eprintln!("Sleeping for {} secs", &sleeptime);
    std::thread::sleep(std::time::Duration::from_secs(sleeptime));
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 && args.len() != 4 {
        eprintln!(
            "Usage: {} <PATH_TO_DRIVE> [RANGE_START] [RANGE_END]",
            args[0]
        );
        std::process::exit(1);
    }
    let (start, end) = if args.len() == 4 {
        (args[2].parse().unwrap(), args[3].parse().unwrap())
    } else {
        (5, 300)
    };
    let dr_name = CString::new(std::env::args().collect::<Vec<String>>()[1].as_bytes()).unwrap();
    unsafe {
        loop {
            if get_status(dr_name.as_ptr()) != 0x1 {
                while get_status(dr_name.as_ptr()) != 0x1_i32 {
                    eprintln!("Status != closed, sleeping 250ms");
                    std::thread::sleep(std::time::Duration::from_millis(250));
                }
                sleep_rnd(start, end)
            }
            eject(dr_name.as_ptr());
            sleep_rnd(start, end);
        }
    }
}
