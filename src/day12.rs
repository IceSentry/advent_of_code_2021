use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
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

pub fn part_1(input: &Data) -> usize {
    let paths = find_paths(input, &[], &HashSet::new(), "start".into());
    // println!("{} paths found:", paths.len());
    // for p in &paths {
    //     println!("{:?}", p);
    // }
    paths.len()
}

fn find_paths(
    graph: &HashMap<String, Vec<String>>,
    path: &[String],
    visited: &HashSet<String>,
    cave: String,
) -> Vec<Vec<String>> {
    if visited.contains(&cave) {
        return vec![];
    }

    let mut path = path.to_owned();
    path.extend_from_slice(&[cave.clone()]);

    if cave == "end" {
        return vec![path];
    }

    let mut visited = visited.clone();
    if cave.chars().all(|c| c.is_lowercase()) {
        visited.insert(cave.clone());
    }

    let mut paths = vec![];
    for c in &graph[&cave] {
        paths.extend(find_paths(graph, &path, &visited, c.clone()));
    }
    paths
}

pub fn part_2(input: &Data) -> usize {
    let paths = find_paths_2(input, &[], "start");
    // println!("{} paths found:", paths.len());
    // let paths_out = paths.iter().map(|p| p.join(",")).sorted();
    // for p in paths_out {
    //     println!("{}", p);
    // }
    paths.len()
}

fn path_contains(path: &[String], cave: &str) -> bool {
    if cave == "start" && path.len() > 1 {
        return true;
    }
    let mut visited = HashMap::new();
    for c in path {
        if c.chars().all(|c| c.is_lowercase()) {
            *visited.entry(c).or_insert(0) += 1;
        }
    }
    let key = cave.to_string();
    if visited.contains_key(&key) {
        if visited.values().filter(|v| **v == 2).count() == 1 {
            visited[&key] == 1 || visited[&key] == 2
        } else {
            false
        }
    } else {
        false
    }
}

fn find_paths_2(
    graph: &HashMap<String, Vec<String>>,
    path: &[String],
    cave: &str,
) -> Vec<Vec<String>> {
    if path_contains(path, cave) {
        return vec![];
    }

    let mut path = path.to_owned();
    path.extend_from_slice(&[cave.into()]);

    if cave == "end" {
        return vec![path];
    }

    let mut paths = vec![];
    for c in &graph[cave] {
        paths.extend(find_paths_2(graph, &path, c).into_iter());
    }
    paths
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
