fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .trim()
            .split("\n")
            .enumerate()
            .filter_map(|(id, line)| {
                line.split(":")
                    .nth(1)
                    .unwrap()
                    .split(";")
                    .all(|random_selection| {
                        random_selection.split(",").all(|selection| {
                            let (number, color) = selection.trim().split_once(" ").unwrap();
                            match (number.parse::<usize>().unwrap(), color) {
                                (0..=12, "red") => true,
                                (0..=13, "green") => true,
                                (0..=14, "blue") => true,
                                (_, _) => false,
                            }
                        })
                    })
                    .then_some(id + 1)
            })
            .sum::<usize>()
    );
}
