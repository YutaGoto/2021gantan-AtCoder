use whiteread::parse_line;

fn main() {
    let _: usize = parse_line().unwrap();
    let v: Vec<usize> = parse_line().unwrap();

    let mut c: usize = 10000000000;

    for e in v.iter() {
        let mut t: usize = *e;
        let mut k: usize = 0;
        while t % 2 == 0 {
            t = t / 2;
            k = k + 1;
        }

        if k < c {
            c = k;
        }
    }
    println!("{}", c);
}
