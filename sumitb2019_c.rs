use whiteread::parse_line;

fn main() {
    let x: usize = parse_line().unwrap();
    let c: usize = x / 100;
    let m: usize = x % 100;

    if 5 * c >= m {
        println!("1");
    } else {
        println!("0");
    }
}
