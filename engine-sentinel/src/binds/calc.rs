extern "C" {
    fn calculate_move(x: i32, y: i32) -> i32;
}

pub fn run_calculation(x: i32, y: i32) -> i32 {
    unsafe { calculate_move(x, y) }
}
