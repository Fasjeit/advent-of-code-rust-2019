advent_of_code::solution!(12);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    part_one_iter(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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

fn cycle(moons: &Vec<Moon>, n: u64) -> Vec<Moon> {
    let mut moons = moons.clone();
    for _ in 0..n {
        moons = apply_gravity(&moons);
        moons = apply_velocity(&moons);
    }

    moons
}

fn apply_gravity(moons: &Vec<Moon>) -> Vec<Moon> {
    let mut next_moons = moons.clone();
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

fn apply_velocity(moons: &Vec<Moon>) -> Vec<Moon> {
    let mut next_moons = moons.clone();
    for (i, moon) in moons.iter().enumerate() {
        next_moons[i].x += moon.dx;
        next_moons[i].y += moon.dy;
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
    fn test_part_one() {
        let result = part_one_iter(
            &advent_of_code::template::read_file_part("examples", DAY, 1),
            100,
        );
        assert_eq!(result, Some(1940));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
