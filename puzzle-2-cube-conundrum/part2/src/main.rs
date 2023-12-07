fn main() {
    println!(
        "{:?}",
        include_str!("../control.txt")
            .trim()
            .split("\n")
            .map(|line| line.split(":").nth(1).unwrap().split(";").fold(
                (0, 0, 0),
                |(mut red, mut green, mut blue), permutation| {
                    permutation.split(",").for_each(|selection| {
                        let (number, color) = selection.trim().split_once(" ").unwrap();
                        let n = number.parse::<usize>().unwrap();
                        match color {
                            "red" => {
                                if n > red {
                                    red = n;
                                }
                            }
                            "green" => {
                                if n > green {
                                    green = n;
                                }
                            }
                            "blue" => {
                                if n > blue {
                                    blue = n;
                                }
                            }
                            _ => unreachable!(),
                        }
                    });
                    (red, green, blue)
                }
            ))
            .map(|(red, green, blue)| red * green * blue)
            .sum::<usize>()
    );
}
