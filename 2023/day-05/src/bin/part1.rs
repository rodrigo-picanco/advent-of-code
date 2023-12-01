use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> usize {
    let (seeds, almanac) = parse_input(input);
    seeds.iter().map(|seed| seed.location(&almanac) as usize).min().expect("No locations found")
}

fn parse_input(input: &str) -> (Vec<Seed>, Almanac) {
    let mut lines = input.lines();
    let seeds = lines.next().expect("No seeds found on input string.");
    let almanac = lines.collect::<Vec<_>>().join(" ");
    (parse_seeds(seeds), parse_almanac(&almanac))
}
fn parse_seeds(input: &str) -> Vec<Seed> {
    let seeds_list = input
        .split(":")
        .nth(1)
        .expect("No seeds found on input string.");
    seeds_list
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().expect("Failed to parse seed number."))
        .map(|x| Seed::new(x))
        .collect()
}
// TODO: refac this to a non c like solution
// maybe nom??
fn parse_almanac(input: &str) -> Almanac {
    let words = input.split_ascii_whitespace().collect::<Vec<_>>();
    let mut almanac = Almanac::new();
    let mut x = 0;
    while x < words.len() {
        match words[x] {
            "map:" => {
                let mut mapppings = vec![];
                let name = words[x - 1].to_string();
                x += 1;
                while (x < words.len()) && (words[x].parse::<usize>().is_ok()) {
                    let destination_range_start = words[x]
                        .parse::<usize>()
                        .expect("Failed to parse destination range start.");
                    let source_range_start = words[x + 1].parse::<usize>().expect("Failed to parse source range start.");
                    let range_length = words[x + 2].parse::<usize>().expect("Failed to parse range length.");
                    mapppings.push(Mapping::new(
                        destination_range_start,
                        source_range_start,
                        range_length,
                    ));
                    // consume the numbers 
                    x += 3;
                }
                almanac.insert(name, mapppings);
            }
            _ => {
                x += 1;
            }
        }
    }
    almanac
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Mapping {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}
impl Mapping {
    fn new(destination_range_start: usize, source_range_start: usize, range_length: usize) -> Self {
        Self {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }
    fn process(&self, input: usize) -> Option<usize> {
        let range = self.source_range_start..=(self.source_range_start + self.range_length);
        if range.contains(&input) {
            return Some(self.destination_range_start + input - self.source_range_start);
        }
        None
    }
}

type Almanac = HashMap<String, Vec<Mapping>>;

#[derive(Debug, PartialEq)]
struct Seed {
    number: usize,
}
impl Seed {
    fn new(number: usize) -> Self {
        Self { number }
    }
    fn process(&self, map: &str, almanac: &Almanac, input: usize) -> usize {
        match almanac.get(map) {
            Some(mappings) => mappings
                .iter()
                .find_map(|mapping| mapping.process(input))
                .unwrap_or(input),
            None => panic!("No mapping found for {}", map),
        }
    }
    fn soil(&self, almanac: &Almanac) -> usize {
        self.process("seed-to-soil", almanac, self.number)
    }
    fn fertilizer(&self, almanac: &Almanac) -> usize {
        let soil = self.soil(&almanac);
        self.process("soil-to-fertilizer", &almanac, soil)
    }
    fn water(&self, almanac: &Almanac) -> usize {
        let fertilizer = self.fertilizer(&almanac);
        self.process("fertilizer-to-water", &almanac, fertilizer)
    }
    fn light(&self, almanac: &Almanac) -> usize {
        let water = self.water(&almanac);
        self.process("water-to-light", &almanac, water)
    }
    fn temperature(&self, almanac: &Almanac) -> usize {
        let light = self.light(&almanac);
        self.process("light-to-temperature", &almanac, light)
    }
    fn humidity(&self, almanac: &Almanac) -> usize {
        let temperature = self.temperature(&almanac);
        self.process("temperature-to-humidity", &almanac, temperature)
    }
    fn location(&self, almanac: &Almanac) -> usize {
        let humidity = self.humidity(&almanac);
        self.process("humidity-to-location", &almanac, humidity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15";

        let (seeds, almanac) = parse_input(input);
        let expected_seeds = vec![Seed::new(79), Seed::new(14), Seed::new(55), Seed::new(13)];
        let expected_almanac = Almanac::from([
            (
                "seed-to-soil".to_string(),
                vec![Mapping::new(50, 98, 2), Mapping::new(52, 50, 48)] as Vec<Mapping>,
            ),
            (
                "soil-to-fertilizer".to_string(),
                vec![
                    Mapping::new(0, 15, 37),
                    Mapping::new(37, 52, 2),
                    Mapping::new(39, 0, 15),
                ] as Vec<Mapping>,
            ),
        ]);
        assert_eq!(seeds, expected_seeds);
        assert_eq!(almanac.get("seed-to-soil"), expected_almanac.get("seed-to-soil"));
    }

    #[test]
    fn test_process_seed() {
        let seed = Seed::new(1);
        let almanac = HashMap::from([("seed-to-soil".to_string(), vec![] as Vec<Mapping>)]);
        assert_eq!(seed.soil(&almanac), 1);
        let seed = Seed::new(79);
        let almanac = HashMap::from([(
            "seed-to-soil".to_string(),
            vec![Mapping::new(50, 98, 2), Mapping::new(52, 50, 48)] as Vec<Mapping>,
        )]);
        assert_eq!(seed.soil(&almanac), 81);
    }

    #[test]
    fn test_process_input() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(process_input(input), 35);
    }
}
