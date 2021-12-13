use hashbrown::HashMap;

type Cave = String;
type Graph = HashMap<Cave, Vec<Cave>>;

pub fn parse(input: &str) -> Graph {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (l, r) = line.split_once('-').expect("");
        let l = l.to_string();
        let r = r.to_string();
        map.entry(l.clone()).or_insert(vec![]).push(r.clone());
        map.entry(r.clone()).or_insert(vec![]).push(l.clone());
    }

    map
}

fn find_paths(graph: &Graph, path: &mut Vec<Cave>, cave: &str, mut repeat_found: bool) -> usize {
    if cave == "end" {
        return 1;
    }

    if path.contains(&cave.to_string()) && cave.chars().all(|c| c.is_lowercase()) {
        if repeat_found || cave == "start" {
            return 0;
        } else {
            repeat_found = true;
        }
    }

    path.push(cave.into());
    let count = graph[cave]
        .iter()
        .map(|cave| find_paths(graph, path, cave, repeat_found))
        .sum();
    path.pop();
    count
}

pub fn part_1(input: &Graph) -> usize {
    find_paths(input, &mut Vec::new(), "start", true)
}

pub fn part_2(input: &Graph) -> usize {
    find_paths(input, &mut Vec::new(), "start", false)
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
