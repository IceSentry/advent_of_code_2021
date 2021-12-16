use std::collections::BinaryHeap;

type Data = Vec<Vec<i32>>;

type Point = (usize, usize);

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| c.to_digit(10))
                .map(|x| x as i32)
                .collect()
        })
        .collect()
}

fn shortest_path(data: &Data, start: Point, goal: Point) -> i32 {
    let mut dist = vec![vec![i32::MAX; data[0].len()]; data.len()];

    let mut queue = BinaryHeap::new();
    queue.push((0, start.0, start.1));

    // WARN
    // The cost needs to be the first element of the tuple
    // Rust will check the elements in order and compare them until one is not equal to the other
    while let Some((cost, x, y)) = queue.pop() {
        if (x, y) == goal {
            return -cost;
        }
        if -cost > dist[y][x] {
            continue;
        }
        for (n_x, n_y) in [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ] {
            if let Some(n_cost) = data.get(n_y).and_then(|row| row.get(n_x)) {
                let n_cost = -cost + n_cost;
                if n_cost < dist[n_y][n_x] {
                    queue.push((-n_cost, n_x, n_y));
                    dist[n_y][n_x] = n_cost;
                }
            }
        }
    }

    dist[goal.1][goal.0]
}

fn enlarge_map(input: &Data) -> Data {
    let height = input.len();
    let width = input[0].len();
    let mut larger_map = vec![vec![0; width * 5]; height * 5];
    for y in 0..larger_map.len() {
        for x in 0..larger_map[0].len() {
            larger_map[y][x] = {
                let cost = input[y % height][x % width] + (x / width) as i32 + (y / height) as i32;
                if cost <= 9 {
                    cost
                } else {
                    cost - 9
                }
            };
        }
    }
    larger_map
}

pub fn part_1(input: &Data) -> i32 {
    shortest_path(input, (0, 0), (input.len() - 1, input[0].len() - 1))
}

pub fn part_2(input: &Data) -> i32 {
    let large_map = enlarge_map(input);
    shortest_path(
        &large_map,
        (0, 0),
        (large_map.len() - 1, large_map[0].len() - 1),
    )
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "};

    const LARGE_MAP: &str = indoc! {"
        11637517422274862853338597396444961841755517295286
        13813736722492484783351359589446246169155735727126
        21365113283247622439435873354154698446526571955763
        36949315694715142671582625378269373648937148475914
        74634171118574528222968563933317967414442817852555
        13191281372421239248353234135946434524615754563572
        13599124212461123532357223464346833457545794456865
        31254216394236532741534764385264587549637569865174
        12931385212314249632342535174345364628545647573965
        23119445813422155692453326671356443778246755488935
        22748628533385973964449618417555172952866628316397
        24924847833513595894462461691557357271266846838237
        32476224394358733541546984465265719557637682166874
        47151426715826253782693736489371484759148259586125
        85745282229685639333179674144428178525553928963666
        24212392483532341359464345246157545635726865674683
        24611235323572234643468334575457944568656815567976
        42365327415347643852645875496375698651748671976285
        23142496323425351743453646285456475739656758684176
        34221556924533266713564437782467554889357866599146
        33859739644496184175551729528666283163977739427418
        35135958944624616915573572712668468382377957949348
        43587335415469844652657195576376821668748793277985
        58262537826937364893714847591482595861259361697236
        96856393331796741444281785255539289636664139174777
        35323413594643452461575456357268656746837976785794
        35722346434683345754579445686568155679767926678187
        53476438526458754963756986517486719762859782187396
        34253517434536462854564757396567586841767869795287
        45332667135644377824675548893578665991468977611257
        44961841755517295286662831639777394274188841538529
        46246169155735727126684683823779579493488168151459
        54698446526571955763768216687487932779859814388196
        69373648937148475914825958612593616972361472718347
        17967414442817852555392896366641391747775241285888
        46434524615754563572686567468379767857948187896815
        46833457545794456865681556797679266781878137789298
        64587549637569865174867197628597821873961893298417
        45364628545647573965675868417678697952878971816398
        56443778246755488935786659914689776112579188722368
        55172952866628316397773942741888415385299952649631
        57357271266846838237795794934881681514599279262561
        65719557637682166874879327798598143881961925499217
        71484759148259586125936169723614727183472583829458
        28178525553928963666413917477752412858886352396999
        57545635726865674683797678579481878968159298917926
        57944568656815567976792667818781377892989248891319
        75698651748671976285978218739618932984172914319528
        56475739656758684176786979528789718163989182927419
        67554889357866599146897761125791887223681299833479
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 40);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 315);
    }

    #[test]
    pub fn large_map() {
        let input = super::parse("8");
        let result = super::enlarge_map(&input);
        let expected = super::parse(indoc! {"
            8 9 1 2 3
            9 1 2 3 4
            1 2 3 4 5
            2 3 4 5 6
            3 4 5 6 7
        "});
        assert_eq!(result, expected);

        let input = super::parse(INPUTS);
        let result = super::enlarge_map(&input);
        let expected = super::parse(LARGE_MAP);

        for y in 0..50 {
            assert_eq!(result[y], expected[y]);
        }
    }
}