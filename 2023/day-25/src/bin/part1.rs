use petgraph::graphmap::GraphMap;
use rustworkx_core::connectivity::stoer_wagner_min_cut;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> usize {
    let adj = parse(input);
    let graph: GraphMap<&str, &str, petgraph::Undirected> = adj.iter().collect();
    let cost = |_| Ok::<i32, &str>(1);
    let min = stoer_wagner_min_cut(&graph, cost);
    let (_, partition) = min.unwrap().unwrap();
    let nodes = graph.node_count();

    (nodes - partition.len()) * partition.len()


}

fn parse_line(line: &str) -> Vec<(&str, &str)> {
    let mut parts = line.split(": ");
    let source = parts.next().expect("should have a source");
    let destinations: Vec<&str> = parts
        .next()
        .expect("should have destinations")
        .split(" ")
        .collect();
    destinations
        .into_iter()
        .map(|dest| (source, dest))
        .collect()
}

fn parse(input: &str) -> Vec<(&str, &str)> {
    input.lines().flat_map(parse_line).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        assert_eq!(process_input(input), 54);
    }
}
