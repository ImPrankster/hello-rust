fn main() {
    let file = std::fs::read_to_string("./src/project/text").unwrap();
    file.lines().for_each(|l| println!("{}", l))
}
