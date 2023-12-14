fn main() {
    let mut it = include_str!("../control.txt").trim().split("\n");

    while let Some(line) = it.next() {
        match line {
            seeds if seeds.contains("seeds") => {
                println!("{}", line)
            }
            _ => {}
        }
    }
}
