fn main() {
    let n = get_number().unwrap_or(0);

    weird_algorithm(n);
}

fn get_number() -> Option<u64> {
    let mut buf = String::new();

    std::io::stdin().read_line(&mut buf).expect("cannot read the file");

    buf.trim().parse().ok()
}

fn weird_algorithm(n: u64) {
    print!("{n} ");

    if n == 1 {
        return;
    }

    if n % 2 == 0 {
        weird_algorithm(n / 2);
    } else {
        weird_algorithm(3 * n + 1);
    }
}
