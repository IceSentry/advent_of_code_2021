use serde_scan::scan;

type TargetArea = (i32, i32, i32, i32);

pub fn parse(input: &str) -> TargetArea {
    let input = input.trim();
    scan!("target area: x={}..{}, y={}..{}" <- input).expect("failed to parse")
}

fn simulate(start_velocity: (i32, i32), target_area: &TargetArea) -> Option<i32> {
    let (mut x, mut y) = (0, 0);
    let mut velocity = start_velocity;
    let mut max_height = i32::MIN;
    while x < target_area.1 && y > target_area.2 {
        x += velocity.0;
        y += velocity.1;

        velocity.0 -= velocity.0.signum();
        velocity.1 -= 1;

        if y > max_height {
            max_height = y;
        }

        if (x >= target_area.0 && x <= target_area.1) && (y >= target_area.2 && y <= target_area.3)
        {
            return Some(max_height);
        }
    }
    None
}

fn find(target_area: &TargetArea) -> (usize, usize) {
    let mut max_height = i32::MIN;
    let mut count = 0;
    for y in target_area.2..-target_area.2 {
        for x in 1..=target_area.1 {
            if let Some(max_h) = simulate((x, y), target_area) {
                if max_h > max_height {
                    max_height = max_h;
                }
                count += 1;
            }
        }
    }
    (max_height as usize, count)
}

pub fn part_1(target_area: &TargetArea) -> usize {
    let (max_height, _) = find(target_area);
    max_height
}

pub fn part_2(target_area: &TargetArea) -> usize {
    let (_, count) = find(target_area);
    count
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        target area: x=20..30, y=-10..-5
    "};

    #[test]
    pub fn simulate() {
        use super::simulate;
        let target = super::parse(INPUTS);

        assert_eq!(simulate((7, 2), &target), Some(3));
        assert_eq!(simulate((6, 3), &target), Some(6));
        assert_eq!(simulate((9, 0), &target), Some(0));
        assert_eq!(simulate((17, -4), &target), None);
        assert_eq!(simulate((6, 9), &target), Some(45));
        assert_eq!(simulate((6, 0), &target), Some(0));
    }

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 45);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 112);
    }
}
