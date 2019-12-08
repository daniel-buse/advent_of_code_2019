const BLACK: u32 = 0;
const WHITE: u32 = 1;
const TRANSPARENT: u32 = 2;
const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;
const PIXELS_PER_LAYER: usize = IMAGE_WIDTH * IMAGE_HEIGHT;

fn part1(pixels: &[u32]) {
    assert_eq!(pixels.len() % PIXELS_PER_LAYER, 0);
    let layers = pixels.len() / PIXELS_PER_LAYER;
    let mut fewest_zero_layer_index = 0;
    let mut min_zeros = usize::max_value();
    for layer in 0..layers {
        let start = layer * PIXELS_PER_LAYER;
        let zeros_count = pixels[start..start + PIXELS_PER_LAYER]
            .iter()
            .filter(|p| **p == 0)
            .count();
        if zeros_count < min_zeros {
            min_zeros = zeros_count;
            fewest_zero_layer_index = layer;
        }
    }
    let start = fewest_zero_layer_index * PIXELS_PER_LAYER;
    let layer_pixels = &pixels[start..start + PIXELS_PER_LAYER];
    let ones_count = layer_pixels.iter().filter(|p| **p == 1).count();
    let twos_count = layer_pixels.iter().filter(|p| **p == 2).count();
    let result = ones_count * twos_count;
    println!("Part1: {}", result);
}

fn part2(pixels: &[u32]) {
    let mut result_pixels = vec![TRANSPARENT; PIXELS_PER_LAYER];
    for pixel in 0..PIXELS_PER_LAYER {
        for layer in pixels.chunks(PIXELS_PER_LAYER) {
            let layer_pixel = layer[pixel];
            if layer_pixel != TRANSPARENT {
                result_pixels[pixel] = layer_pixel;
                break;
            }
        }
    }
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let pixel = result_pixels[x + y * IMAGE_WIDTH];
            match pixel {
                BLACK => print!(" "),
                WHITE => print!("#"),
                _ => unreachable!(),
            }
        }
        println!();
    }
}

fn main() {
    let input_str = include_str!("input.txt");
    let input = input_str
        .trim()
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<u32>>>()
        .unwrap();
    part1(&input);
    part2(&input);
}
