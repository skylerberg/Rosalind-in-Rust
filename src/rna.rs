use std::io;
use std::str;

fn main() {
    let reader = io::stdin();
    let dna = reader.read_line();
    let rna = str::replace(dna, "T", "U");
    println(rna);
}
