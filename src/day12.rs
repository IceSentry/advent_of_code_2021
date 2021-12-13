use hashbrown::{HashMap, HashSet};
use serde_scan::scan;

type Data = HashMap<String, Vec<String>>;

pub fn parse(input: &str) -> Data {
    let input: Vec<(String, String)> = input
        .lines()
        .map(|line| scan!("{}-{}" <- line).expect("Failed to parse input"))
        .collect();

    let mut map = HashMap::new();
    for (l, r) in input {
        map.entry(l.clone()).or_insert(vec![]).push(r.clone());
        map.entry(r.clone()).or_insert(vec![]).push(l.clone());
    }

    map
}

fn find_paths(
    graph: &HashMap<String, Vec<String>>,
    path: &mut Vec<String>,
    cave: &str,
    valid_path: fn(&[String], &str) -> bool,
) -> usize {
    if cave == "end" {
        return 1;
    }

    if valid_path(path, cave) {
        return 0;
    }

    path.push(cave.into());
    let count = graph[cave]
        .iter()
        .map(|cave| find_paths(graph, path, cave, valid_path))
        .sum();
    path.pop();
    count
}

pub fn part_1(input: &Data) -> usize {
    find_paths(input, &mut Vec::new(), "start", |path, cave| {
        path.contains(&cave.to_string()) && cave.chars().all(|c| c.is_lowercase())
    })
}

pub fn part_2(input: &Data) -> usize {
    find_paths(input, &mut Vec::new(), "start", |path, cave| {
        if cave == "start" && path.len() > 1 {
            return true;
        }
        let mut visited = HashSet::new();
        let mut repeat = false;
        for c in path {
            if c.chars().all(|c| c.is_lowercase()) {
                if visited.contains(c) {
                    repeat = true;
                } else {
                    visited.insert(c);
                }
            }
        }
        visited.contains(&cave.to_string()) && repeat
    })
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    "};

    const INPUTS_LARGER: &str = indoc! {"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    "};

    const INPUTS_EVEN_LARGER: &str = indoc! {"
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 10);

        let input = super::parse(INPUTS_LARGER);
        let result = super::part_1(&input);
        assert_eq!(result, 19);

        let input = super::parse(INPUTS_EVEN_LARGER);
        let result = super::part_1(&input);
        assert_eq!(result, 226);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 36);

        let input = super::parse(INPUTS_LARGER);
        let result = super::part_2(&input);
        assert_eq!(result, 103);

        let input = super::parse(INPUTS_EVEN_LARGER);
        let result = super::part_2(&input);
        assert_eq!(result, 3509);
    }
}
