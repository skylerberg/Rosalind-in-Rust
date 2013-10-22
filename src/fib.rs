use std::io;

fn main() {
    let data = io::stdin().read_line();
    let vec = data.split_iter(' ').to_owned_vec();
    let n = match from_str::<int>(vec[0]) {
        Some(x) => x,
        None    => fail!("Invalid input")
    };
    let k = match from_str::<int>(vec[1]) {
        Some(x) => x,
        None    => fail!("Invalid input")
    };

    let mut cur = 1;
    let mut prev = 1;
    for i in range(2,n) {
        cur = cur + prev*k;
        prev = cur - prev*k;
    }

    println(fmt!("%d", cur));
}
