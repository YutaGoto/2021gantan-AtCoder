use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    println!("{}", n * 2);
}
