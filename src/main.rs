use rand::Rng;
use std::ffi::{c_char, CString};

#[link(name = "get_status")]
extern "C" {
    pub fn get_status(drive_path: *const c_char) -> std::ffi::c_int;
    pub fn eject(drive_path: *const c_char);
}

fn sleep_rnd() {
    let sleeptime = rand::thread_rng().gen_range(5..300);
    eprintln!("Sleeping for {} secs", &sleeptime);
    std::thread::sleep(std::time::Duration::from_secs(sleeptime));
}

fn main() {
    let dr_name = CString::new(std::env::args().collect::<Vec<String>>()[1].as_bytes()).unwrap();
    println!("{:?}", unsafe { get_status(dr_name.as_ptr()) });
    unsafe {
        loop {
            if get_status(dr_name.as_ptr()) != 0x1 {
                while get_status(dr_name.as_ptr()) != 0x1_i32 {
                    eprintln!("Status != 0x1, sleeping 100ms");
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                sleep_rnd()
            }
            eject(dr_name.as_ptr());
            sleep_rnd();
        }
    }
}
