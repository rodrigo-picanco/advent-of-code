use std::collections::BTreeMap;

use nom::IResult;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
    // 342030 too high
}

fn process_input(input: &str) -> u32 {
    input
        .trim_end_matches('\n')
        .split(',')
        .map(parse_entry)
        .fold(
            BTreeMap::new() as BTreeMap<u32, Vec<(&str, u32)>>,
            |mut boxes, (case, lens, new_value)| {
                match (boxes.get_mut(&case), new_value) {
                    (Some(case), Some(new_value)) => {
                        if let Some(index) =
                            case.iter().position(|(item_lens, _)| *item_lens == lens)
                        {
                            case[index] = (lens, new_value);
                        } else {
                            case.push((lens, new_value));
                        }
                    }
                    (Some(case), None) => {
                        if let Some(idx) = case.iter().position(|(item_lens, _)| *item_lens == lens) {
                            case.remove(idx);
                        }
                    }
                    (None, Some(new_value)) => {
                        boxes.insert(case, vec![(lens, new_value)]);
                    }
                    (None, None) => {}
                };
                boxes
                // map.entry(case)
                //     .and_modify(|e| {
                //         *e = match e.iter().find(|(l, _)| *l == lens) {
                //             Some(_) => {
                //             },
                //             None => e
                //                 .iter()
                //                 .chain(std::iter::once(&(lens, new_value)))
                //                 .cloned()
                //                 .collect(),
                //         }
                //     })
                //     .or_insert_with(||
                //                     match new_value {
                //                         Some(v) => vec![(lens, v)],
                //                         None => vec![],
                //                     });
                // map
            },
        )
        .iter()
        .fold(0, |acc, (case_number, case)| {
            acc + case
                .iter()
                .enumerate()
                .fold(0, |acc, (slot_number, (_, focal_length))| {
                    acc + ((case_number + 1) * (slot_number + 1) as u32 * focal_length)
                })
        })
}

fn hash(input: &str) -> u32 {
    input
        .bytes()
        .fold(0, |hash, ascii| (hash + ascii as u32) * 17 % 256)
}

fn parse(input: &str) -> IResult<&str, (&str, char, &str)> {
    nom::sequence::tuple((
        nom::character::complete::alpha1,
        nom::character::complete::one_of("-="),
        nom::character::complete::digit0,
    ))(input)
}

fn parse_entry(input: &str) -> (u32, &str, Option<u32>) {
    let (_, (key, sign, value)) = parse(input).unwrap();
    let case = hash(key);
    (
        case,
        key,
        match sign {
            '-' => None,
            '=' => Some(value.parse::<u32>().expect("no value found on entry")),
            _ => panic!("invalid sign found on entry {sign}"),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("cm-", (0, "cm", None))]
    #[case("cm=2", (0, "cm", Some(2)))]
    #[case("dcf=3", (173, "dcf", Some(3)))]
    fn test_parse_entry(#[case] input: &str, #[case] expected: (u32, &str, Option<u32>)) {
        assert_eq!(parse_entry(input), expected)
    }

    #[rstest]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn hash_test(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, hash(input))
    }

    #[test]
    fn test_process() {
        assert_eq!(
            process_input("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145
        );
    }
}
