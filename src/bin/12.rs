advent_of_code::solution!(12);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    part_one_iter(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut moons = Vec::<Moon>::new();

    for line in input.lines() {
        let mut iter = line.split(',');
        let x: i64 = iter.next().unwrap()[3..].parse().unwrap();
        let y: i64 = iter.next().unwrap()[3..].parse().unwrap();
        let z_str = &iter.next().unwrap()[3..];
        let z: i64 = z_str[..z_str.len() - 1].parse().unwrap();

        let moon = Moon {
            x,
            y,
            z,
            dx: 0,
            dy: 0,
            dz: 0,
        };

        moons.push(moon);
    }

    // The main key thing is - each coordinate updates independently.
    // So we need to find c,y,z cycles and compute LCM.

    let max_iteration = 100000000;

    let c_x = cycle_x(&moons, max_iteration) as u64;
    //dbg!(&c_x);
    let c_y = cycle_y(&moons, max_iteration) as u64;
    //dbg!(&c_y);
    let c_z = cycle_z(&moons, max_iteration) as u64;
    //dbg!(&c_z);

    Some(lcm3(c_x, c_y, c_z))
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
}

fn lcm3(a: u64, b: u64, c: u64) -> u64 {
    lcm(lcm(a, b), c)
}

pub fn part_one_iter(input: &str, iterations: u64) -> Option<u64> {
    let mut moons = Vec::<Moon>::new();

    for line in input.lines() {
        let mut iter = line.split(',');
        let x: i64 = iter.next().unwrap()[3..].parse().unwrap();
        let y: i64 = iter.next().unwrap()[3..].parse().unwrap();
        let z_str = &iter.next().unwrap()[3..];
        let z: i64 = z_str[..z_str.len() - 1].parse().unwrap();

        let moon = Moon {
            x,
            y,
            z,
            dx: 0,
            dy: 0,
            dz: 0,
        };

        moons.push(moon);
    }
    moons = cycle(&moons, iterations);

    //dbg!(&moons);

    let result = moons.iter().fold(0, |acc, m| acc + get_enegry(m));

    Some(result as u64)
}

#[derive(Debug, Clone)]
struct Moon {
    x: i64,
    y: i64,
    z: i64,

    dx: i64,
    dy: i64,
    dz: i64,
}

fn cycle(moons: &[Moon], n: u64) -> Vec<Moon> {
    let mut moons = moons.to_owned();
    for _ in 0..n {
        moons = apply_gravity(&moons);
        moons = apply_velocity(&moons);
    }

    moons
}

fn cycle_x(moons: &[Moon], n: u64) -> i64 {
    let mut moons = moons.to_owned();
    let mut prev_x = Vec::<i64>::new();
    let mut prev_dx = Vec::<i64>::new();
    for moon in &moons {
        prev_x.push(moon.x);
        prev_dx.push(moon.dx);
    }

    for k in 0..n {
        moons = apply_gravity_x(&moons);
        moons = apply_velocity_x(&moons);

        //dbg!(&moons[0]);

        if moons
            .iter()
            .enumerate()
            .all(|(i, moon)| moon.x == prev_x[i] && moon.dx == prev_dx[i])
        {
            return (k + 1) as i64;
        }
    }

    panic!("Waiting too long...");
}

fn cycle_y(moons: &[Moon], n: u64) -> i64 {
    let mut moons = moons.to_owned();
    let mut prev_y = Vec::<i64>::new();
    let mut prev_dy = Vec::<i64>::new();
    for moon in &moons {
        prev_y.push(moon.y);
        prev_dy.push(moon.dy);
    }

    for k in 0..n {
        moons = apply_gravity_y(&moons);
        moons = apply_velocity_y(&moons);

        //dbg!(&moons[0]);

        if moons
            .iter()
            .enumerate()
            .all(|(i, moon)| moon.y == prev_y[i] && moon.dy == prev_dy[i])
        {
            return (k + 1) as i64;
        }
    }

    panic!("Waiting too long...");
}

fn cycle_z(moons: &[Moon], n: u64) -> i64 {
    let mut moons = moons.to_owned();
    let mut prev_z = Vec::<i64>::new();
    let mut prev_dz = Vec::<i64>::new();
    for moon in &moons {
        prev_z.push(moon.z);
        prev_dz.push(moon.dz);
    }

    for k in 0..n {
        moons = apply_gravity_z(&moons);
        moons = apply_velocity_z(&moons);

        //dbg!(&k);
        //dbg!(&moons[0]);

        if moons
            .iter()
            .enumerate()
            .all(|(i, moon)| moon.z == prev_z[i] && moon.dz == prev_dz[i])
        {
            return (k + 1) as i64;
        }
    }

    panic!("Waiting too long...");
}

fn apply_gravity(moons: &[Moon]) -> Vec<Moon> {
    let mut next_moons = moons.to_owned();
    for ((ai, a), (bi, b)) in moons.iter().enumerate().tuple_combinations() {
        if a.x < b.x {
            next_moons[ai].dx += 1;
            next_moons[bi].dx -= 1;
        }
        if a.x > b.x {
            next_moons[ai].dx -= 1;
            next_moons[bi].dx += 1;
        }

        if a.y < b.y {
            next_moons[ai].dy += 1;
            next_moons[bi].dy -= 1;
        }
        if a.y > b.y {
            next_moons[ai].dy -= 1;
            next_moons[bi].dy += 1;
        }

        if a.z < b.z {
            next_moons[ai].dz += 1;
            next_moons[bi].dz -= 1;
        }
        if a.z > b.z {
            next_moons[ai].dz -= 1;
            next_moons[bi].dz += 1;
        }
    }
    next_moons
}

fn apply_gravity_x(moons: &[Moon]) -> Vec<Moon> {
    let mut next_moons = moons.to_owned();
    for ((ai, a), (bi, b)) in moons.iter().enumerate().tuple_combinations() {
        if a.x < b.x {
            next_moons[ai].dx += 1;
            next_moons[bi].dx -= 1;
        }
        if a.x > b.x {
            next_moons[ai].dx -= 1;
            next_moons[bi].dx += 1;
        }
    }
    next_moons
}

fn apply_gravity_y(moons: &[Moon]) -> Vec<Moon> {
    let mut next_moons = moons.to_owned();
    for ((ai, a), (bi, b)) in moons.iter().enumerate().tuple_combinations() {
        if a.y < b.y {
            next_moons[ai].dy += 1;
            next_moons[bi].dy -= 1;
        }
        if a.y > b.y {
            next_moons[ai].dy -= 1;
            next_moons[bi].dy += 1;
        }
    }
    next_moons
}

fn apply_gravity_z(moons: &[Moon]) -> Vec<Moon> {
    let mut next_moons = moons.to_owned();
    for ((ai, a), (bi, b)) in moons.iter().enumerate().tuple_combinations() {
        if a.z < b.z {
            next_moons[ai].dz += 1;
            next_moons[bi].dz -= 1;
        }
        if a.z > b.z {
            next_moons[ai].dz -= 1;
            next_moons[bi].dz += 1;
        }
    }
    next_moons
}

fn apply_velocity(moons: &[Moon]) -> Vec<Moon> {
    let mut next_moons = moons.to_owned();
    for (i, moon) in moons.iter().enumerate() {
        next_moons[i].x += moon.dx;
        next_moons[i].y += moon.dy;
        next_moons[i].z += moon.dz;
    }
    next_moons
}

fn apply_velocity_x(moons: &[Moon]) -> Vec<Moon> {
    let mut next_moons = moons.to_owned();
    for (i, moon) in moons.iter().enumerate() {
        next_moons[i].x += moon.dx;
    }
    next_moons
}

fn apply_velocity_y(moons: &[Moon]) -> Vec<Moon> {
    let mut next_moons = moons.to_owned();
    for (i, moon) in moons.iter().enumerate() {
        next_moons[i].y += moon.dy;
    }
    next_moons
}

fn apply_velocity_z(moons: &[Moon]) -> Vec<Moon> {
    let mut next_moons = moons.to_owned();
    for (i, moon) in moons.iter().enumerate() {
        next_moons[i].z += moon.dz;
    }
    next_moons
}

fn get_enegry(moon: &Moon) -> i64 {
    let potential_energy = moon.x.abs() + moon.y.abs() + moon.z.abs();
    let kinetic_energy = moon.dx.abs() + moon.dy.abs() + moon.dz.abs();

    potential_energy * kinetic_energy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one_iter(
            &advent_of_code::template::read_file_part("examples", DAY, 1),
            10,
        );
        assert_eq!(result, Some(179));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one_iter(
            &advent_of_code::template::read_file_part("examples", DAY, 2),
            100,
        );
        assert_eq!(result, Some(1940));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2772));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4686774924));
    }
}
