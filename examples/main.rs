use impfuzzy;

fn main() {
    let h = impfuzzy::hash_from_file("./test.exe").unwrap();
    println!("{:?}", h);
}
