

struct Rect {
    xlow: i64,
    xhigh: i64,
    ylow: i64,
    yhigh: i64,
}

impl Rect {
    fn contains(&self, point: (i64, i64)) -> bool {
        point.0 >= self.xlow
            && point.0 <= self.xhigh
            && point.1 >= self.ylow
            && point.1 <= self.yhigh
    }
    fn passed(&self, point: (i64, i64), xvelocity: i64) -> bool {
        point.1 < self.ylow && (point.0 > self.xhigh || xvelocity == 0)
    }
}

fn search(target: &Rect, start_velocity: (i64, i64)) -> Option<i64> {
    let mut max_height = 0;
    let mut position = (0, 0);
    let mut velocity = start_velocity;
    while !target.contains(position) {
        if target.passed(position, velocity.0) {
            return None;
        }
        position = (position.0 + velocity.0, position.1 + velocity.1);
        velocity.0 -= num::signum(velocity.0);
        velocity.1 -= 1;
        max_height = std::cmp::max(max_height, position.1);
    }
    return Some(max_height);
}

#[test]
fn part1() {
    let target = Rect {
        xlow: 206,
        xhigh: 250,
        ylow: -105,
        yhigh: -57,
    };
    let mut max_height = 0;
    for i in 0..256 {
        for j in 0..500 {
            let result = search(&target, (i, j));
            if let Some(height) = result {
                if height > max_height {
                    println!("Found new max! {:?} {}", (i, j), height);
                    max_height = height;
                }
            }
        }
    }
}


#[test]
fn part2() {
    let target = Rect {
        xlow: 206,
        xhigh: 250,
        ylow: -105,
        yhigh: -57,
    };
    let mut num_hits = 0;
    for i in 0..256 {
        for j in -105..500 {
            let result = search(&target, (i, j));
            if let Some(_) = result {
                num_hits +=1
            }
        }
    }
    println!("total hits {}", num_hits);
}