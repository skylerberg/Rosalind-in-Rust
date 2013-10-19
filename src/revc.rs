use std::io;
use std::str;

fn main() {
    let mut data = io::stdin().read_line();
    // Use X as a temporary storage value
    data = str::replace(data, "A", "X");
    data = str::replace(data, "T", "A");
    data = str::replace(data, "X", "T");

    data = str::replace(data, "C", "X");
    data = str::replace(data, "G", "C");
    data = str::replace(data, "X", "G");

    // I need to find a better way to reverse a string in Rust
    for nucleotide in data.rev_iter() {
        print(fmt!("%c", nucleotide));
    }
    println("");
}
