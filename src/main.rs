use rand::Rng;
use std::ffi::{c_char, CString};

extern "C" {
    pub fn c_get_status(drive_path: *const c_char) -> std::ffi::c_int;
    pub fn c_eject(drive_path: *const c_char);
}

fn sleep_rnd(start: u64, end: u64) {
    let sleeptime = rand::thread_rng().gen_range(start..=end);
    eprintln!("Sleeping for {} secs", &sleeptime);
    std::thread::sleep(std::time::Duration::from_secs(sleeptime));
}

#[derive(PartialEq, Debug)]
enum Status {
    Opened,  // 0
    Closed,  // 1
    Invalid, // 2
}

#[inline(always)]
fn get_status(drive_path: &CString) -> Status {
    let status = unsafe { c_get_status(drive_path.as_ptr()) };
    match status {
        0 => Status::Opened,
        1 => Status::Closed,
        2 => Status::Invalid,
        _ => panic!("Invalide status code from C module"),
    }
}

#[inline(always)]
fn eject(drive_path: &CString) {
    unsafe { c_eject(drive_path.as_ptr()) }
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
    let dr_name = CString::new(args[1].as_bytes()).unwrap();
    loop {
        if get_status(&dr_name) != Status::Closed {
            while get_status(&dr_name) != Status::Closed {
                eprintln!(
                    "Status != closed ({:?}), sleeping 250ms",
                    get_status(&dr_name)
                );
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            sleep_rnd(start, end)
        }
        eject(&dr_name);
        sleep_rnd(start, end);
    }
}
