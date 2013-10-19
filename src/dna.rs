use std::io;

fn main() {
    let data = io::stdin().read_line();
    let mut a_cnt = 0;
    let mut t_cnt = 0;
    let mut c_cnt = 0;
    let mut g_cnt = 0;
    for letter in data.iter() {
        match letter {
            'A' => a_cnt += 1,
            'T' => t_cnt += 1,
            'C' => c_cnt += 1,
            'G' => g_cnt += 1,
            _ => println("Something has gone terribly wrong")
        }
    }
    println(fmt!("%d %d %d %d", a_cnt, c_cnt, g_cnt, t_cnt));
}
