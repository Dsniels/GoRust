fn get_input() -> &'static str {
    return "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
}

fn main() {
    print!(
        "{}",
        get_input()
            .lines() // this is the iterator :)
            .enumerate() // grab a value and attach an index on it
            .flat_map(|(idx, line)| {
                //if the thing is none flat map will ignore it
                return line.chars().nth(idx * 3 % line.len());
            })
            .filter(|&x| x == '#') //filter needs a reference
            .count()
    );
}
