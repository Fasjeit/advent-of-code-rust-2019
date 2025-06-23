extern crate bmp;
use bmp::Image;
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let (data, size_x_y) = (lines.next().unwrap(), lines.next().unwrap_or("25,6"));

    let splitted_size: Vec<u32> = size_x_y
        .split(",")
        .map(|i| i.parse().expect("Expected u64 list"))
        .collect();

    let (image_size_x, image_size_y) = (splitted_size[0] as usize, splitted_size[1] as usize);
    let layer_size = image_size_y * image_size_x;

    let data_int: Vec<u32> = data.chars().filter_map(|c| c.to_digit(10)).collect();

    let mut layer_zeroes = HashMap::<usize, u32>::new();
    for layer in 0..data_int.len() / layer_size {
        layer_zeroes.entry(layer).or_insert(0);
    }

    for (index, digit) in data_int.iter().enumerate() {
        let layer = index / layer_size;
        if *digit == 0 {
            layer_zeroes.entry(layer).and_modify(|e| *e += 1);
        }
    }

    // find the number of 1 digits multiplied by the number of 2 digits
    // on layer containing smallest number of zeroes
    let (min_layer, _) = layer_zeroes.iter().min_by_key(|&(_, v)| v).unwrap();
    //dbg!(&layer_zeroes);
    //println!("Min layer is [{min_layer}]");

    let mut ones = 0;
    let mut twos = 0;
    for element in data_int
        .iter()
        .skip(layer_size * min_layer)
        .take(layer_size)
    {
        if *element == 1 {
            ones += 1;
        } else if *element == 2 {
            twos += 1;
        }
    }

    Some(ones * twos)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let (data, size_x_y) = (lines.next().unwrap(), lines.next().unwrap_or("25,6"));

    let splitted_size: Vec<u32> = size_x_y
        .split(",")
        .map(|i| i.parse().expect("Expected u64 list"))
        .collect();

    let (image_size_x, image_size_y) = (splitted_size[0] as usize, splitted_size[1] as usize);
    let layer_size = image_size_y * image_size_x;

    let data_int: Vec<u32> = data.chars().filter_map(|c| c.to_digit(10)).collect();

    //let mut data_out: Vec<u32> = Vec::with_capacity(data_int.len());
    let mut img = Image::new(image_size_x as u32, image_size_y as u32);

    for y in 0..image_size_y {
        for x in 0..image_size_x {
            let index = y * image_size_x + x;

            let mut layer = 0;

            loop {
                let layer_index = index + layer * layer_size;
                //println!("pos [y:{y}-x:{x}] l [{layer}] li [{layer_index}]");
                if data_int[layer_index] == 0 {
                    img.set_pixel(x as u32, y as u32, bmp::consts::BLACK);
                    //data_out.push(0);
                    //println!("layer [{layer}] - 0");
                    break;
                } else if data_int[layer_index] == 1 {
                    img.set_pixel(x as u32, y as u32, bmp::consts::RED);
                    //data_out.push(1);
                    //println!("layer [{layer}] - 1");
                    break;
                } else {
                    // transparent - check other layers.
                    layer += 1;
                    continue;
                }
            }
            //let value = data_out.last().unwrap();
            //let symbol = if *value == 0 { "⬛" } else { "⬜" };
            //print!("{}", symbol);
        }
        //println!();
    }

    let _ = img.save("decoded_image_day8_part2.bmp");

    None
}

// fn decode_pixel(data: Vec<u32>, x: usize, y: usize) {
//     for layer in
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        // see image file output
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        // see image file output
        assert_eq!(result, None);
    }
}
