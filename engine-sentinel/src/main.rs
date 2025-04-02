use engine_sentinel::binds;

fn main() {
    let result = binds::run_calculation(3, 4);
    println!("Calculation result: {}", result);
}