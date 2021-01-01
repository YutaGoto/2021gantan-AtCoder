use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    if n == 12 {
        println!("{}", 1);
    } else {
        println!("{}", n + 1)
    }
}
