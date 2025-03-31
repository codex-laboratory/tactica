use engine_core::binds;

fn main() {
    let result = binds::run_calculation(3, 4);
    println!("Calculation result: {}", result);
}