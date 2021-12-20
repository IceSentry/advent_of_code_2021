use serde_scan::scan;

type TargetArea = (i32, i32, i32, i32);

pub fn parse(input: &str) -> TargetArea {
    let input = input.trim();
    scan!("target area: x={}..{}, y={}..{}" <- input).expect("failed to parse")
}

fn simulate(velocity: &mut (i32, i32), target_area: &TargetArea) -> Option<i32> {
    let mut probe = (0, 0);
    let mut max_height = i32::MIN;
    while probe.0 < target_area.1 && probe.1 > target_area.2 {
        probe.0 += velocity.0;
        probe.1 += velocity.1;

        velocity.0 -= velocity.0.signum();
        velocity.1 -= 1;

        if probe.1 > max_height {
            max_height = probe.1;
        }

        if (probe.0 >= target_area.0 && probe.0 <= target_area.1)
            && (probe.1 >= target_area.2 && probe.1 <= target_area.3)
        {
            return Some(max_height);
        }
    }
    None
}

pub fn part_1(target_area: &TargetArea) -> usize {
    let mut max_height = i32::MIN;
    for y in target_area.2..-target_area.2 {
        for x in 1..=target_area.1 {
            if let Some(max_h) = simulate(&mut (x, y), target_area) {
                if max_h > max_height {
                    max_height = max_h;
                }
            }
        }
    }
    max_height as usize
}

pub fn part_2(target_area: &TargetArea) -> usize {
    let mut count = 0;
    for y in target_area.2..-target_area.2 {
        for x in 1..=target_area.1 {
            if simulate(&mut (x, y), target_area).is_some() {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        target area: x=20..30, y=-10..-5
    "};

    #[test]
    pub fn step() {
        let target = super::parse(INPUTS);
        let result = super::simulate(&mut (7, 2), &target);
        assert_eq!(result, Some(3));

        let result = super::simulate(&mut (6, 3), &target);
        assert_eq!(result, Some(6));

        let result = super::simulate(&mut (9, 0), &target);
        assert_eq!(result, Some(0));

        let result = super::simulate(&mut (17, -4), &target);
        assert_eq!(result, None);

        let result = super::simulate(&mut (6, 9), &target);
        assert_eq!(result, Some(45));

        let result = super::simulate(&mut (6, 0), &target);
        assert_eq!(result, Some(0));
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
