use impfuzzy;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let h = impfuzzy::hash_from_file(args[1].clone()).unwrap();
    println!("{:?}", h);
}
