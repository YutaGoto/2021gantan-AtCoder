use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();

    for _i in 0..n {
        let s: String = parse_line().unwrap();
        let mut chars: Vec<_> = s.chars().collect(); 
        chars.sort();
        let t: String = chars.into_iter().collect();

        if t == "ddeeinnow" {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
