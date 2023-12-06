use rayon::prelude::*;

#[derive(Debug)]
struct RangeMap {
    destination_start: i64,
    destination_end: i64,
    source_start: i64,
    source_end: i64,
}

impl RangeMap {
    fn new(destination_start: i64, source_start: i64, length: i64) -> Self {
        Self {
            destination_start,
            destination_end: destination_start + length,
            source_start,
            source_end: source_start + length,
        }
    }

    fn in_destination_range(&self, value: i64) -> bool {
        value >= self.destination_start && value <= self.destination_end
    }

    fn in_source_range(&self, value: i64) -> bool {
        value >= self.source_start && value <= self.source_end
    }

    fn map(&self, value: i64) -> i64 {
        if !self.in_source_range(value) {
            panic!("value {} is not in source range", value);
        }
        let offset = value - self.source_start;
        self.destination_start + offset
    }
}

pub fn process(input: &str) -> i64 {
    let (
        seeds,
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidy_to_location_map,
    ) = parse_input(input);
    // dbg!(&seed_to_soil_map);

    let mut min_location = std::i64::MAX;

    for seed in seeds.iter() {
        let mut seed_to_soil = -1;
        for r in seed_to_soil_map.iter() {
            if r.in_source_range(*seed) {
                seed_to_soil = r.map(*seed);
                break;
            }
        }
        if seed_to_soil == -1 {
            seed_to_soil = *seed;
        }
        let mut soil_to_fertilizer = -1;
        for r in soil_to_fertilizer_map.iter() {
            if r.in_source_range(seed_to_soil) {
                soil_to_fertilizer = r.map(seed_to_soil);
                break;
            }
        }
        if soil_to_fertilizer == -1 {
            soil_to_fertilizer = seed_to_soil;
        }
        let mut fertilizer_to_water = -1;
        for r in fertilizer_to_water_map.iter() {
            if r.in_source_range(soil_to_fertilizer) {
                fertilizer_to_water = r.map(soil_to_fertilizer);
                break;
            }
        }
        if fertilizer_to_water == -1 {
            fertilizer_to_water = soil_to_fertilizer;
        }
        let mut water_to_light = -1;
        for r in water_to_light_map.iter() {
            if r.in_source_range(fertilizer_to_water) {
                water_to_light = r.map(fertilizer_to_water);
                break;
            }
        }
        if water_to_light == -1 {
            water_to_light = fertilizer_to_water;
        }
        let mut light_to_temperature = -1;
        for r in light_to_temperature_map.iter() {
            if r.in_source_range(water_to_light) {
                light_to_temperature = r.map(water_to_light);
                break;
            }
        }
        if light_to_temperature == -1 {
            light_to_temperature = water_to_light;
        }
        let mut temperature_to_humidity = -1;
        for r in temperature_to_humidity_map.iter() {
            if r.in_source_range(light_to_temperature) {
                temperature_to_humidity = r.map(light_to_temperature);
                break;
            }
        }
        if temperature_to_humidity == -1 {
            temperature_to_humidity = light_to_temperature;
        }
        let mut humidy_to_location = -1;
        for r in humidy_to_location_map.iter() {
            if r.in_source_range(temperature_to_humidity) {
                humidy_to_location = r.map(temperature_to_humidity);
                break;
            }
        }
        if humidy_to_location == -1 {
            humidy_to_location = temperature_to_humidity;
        }

        // dbg!(seed_to_soil);
        // dbg!(soil_to_fertilizer);
        // dbg!(fertilizer_to_water);
        // dbg!(water_to_light);
        // dbg!(light_to_temperature);
        // dbg!(temperature_to_humidity);
        // dbg!(humidy_to_location);

        min_location = min_location.min(humidy_to_location);
    }
    min_location
}

fn parse_input(
    input: &str,
) -> (
    Vec<i64>,
    Vec<RangeMap>,
    Vec<RangeMap>,
    Vec<RangeMap>,
    Vec<RangeMap>,
    Vec<RangeMap>,
    Vec<RangeMap>,
    Vec<RangeMap>,
) {
    // seeds: 79 14 55 13
    let mut input = input.split("\n\n");
    let seeds_section = input.next().unwrap().split(": ").last().unwrap();
    let seeds = seeds_section
        .split(' ')
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    // dbg!(seeds);

    let seed_to_soil_map = input
        .next()
        .unwrap()
        .split(':')
        // .inspect(|line| println!("line: {}", line))
        .last()
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        // .inspect(|line| println!("line: {}", line))
        .map(|line| {
            let mut it = line.split(' ');
            let destination_start = it.next().unwrap().parse::<i64>().unwrap();
            let source_start = it.next().unwrap().parse::<i64>().unwrap();
            let length = it.next().unwrap().parse::<i64>().unwrap();
            RangeMap::new(destination_start, source_start, length - 1)
        })
        .collect::<Vec<_>>();
    // dbg!(&seed_to_soil_map);

    let soil_to_fertilizer_map = input
        .next()
        .unwrap()
        .split(':')
        // .inspect(|line| println!("line: {}", line))
        .last()
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        // .inspect(|line| println!("line: {}", line))
        .map(|line| {
            let mut it = line.split(' ');
            let destination_start = it.next().unwrap().parse::<i64>().unwrap();
            let source_start = it.next().unwrap().parse::<i64>().unwrap();
            let length = it.next().unwrap().parse::<i64>().unwrap();
            RangeMap::new(destination_start, source_start, length - 1)
        })
        .collect::<Vec<_>>();
    // dbg!(&soil_to_fertilizer_map);

    let fertilizer_to_water_map = input
        .next()
        .unwrap()
        .split(':')
        // .inspect(|line| println!("line: {}", line))
        .last()
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        // .inspect(|line| println!("line: {}", line))
        .map(|line| {
            let mut it = line.split(' ');
            let destination_start = it.next().unwrap().parse::<i64>().unwrap();
            let source_start = it.next().unwrap().parse::<i64>().unwrap();
            let length = it.next().unwrap().parse::<i64>().unwrap();
            RangeMap::new(destination_start, source_start, length - 1)
        })
        .collect::<Vec<_>>();

    let water_to_light_map = input
        .next()
        .unwrap()
        .split(':')
        // .inspect(|line| println!("line: {}", line))
        .last()
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        // .inspect(|line| println!("line: {}", line))
        .map(|line| {
            let mut it = line.split(' ');
            let destination_start = it.next().unwrap().parse::<i64>().unwrap();
            let source_start = it.next().unwrap().parse::<i64>().unwrap();
            let length = it.next().unwrap().parse::<i64>().unwrap();
            RangeMap::new(destination_start, source_start, length - 1)
        })
        .collect::<Vec<_>>();

    let light_to_temperature_map = input
        .next()
        .unwrap()
        .split(':')
        // .inspect(|line| println!("line: {}", line))
        .last()
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        // .inspect(|line| println!("line: {}", line))
        .map(|line| {
            let mut it = line.split(' ');
            let destination_start = it.next().unwrap().parse::<i64>().unwrap();
            let source_start = it.next().unwrap().parse::<i64>().unwrap();
            let length = it.next().unwrap().parse::<i64>().unwrap();
            RangeMap::new(destination_start, source_start, length - 1)
        })
        .collect::<Vec<_>>();

    let temperature_to_humidity_map = input
        .next()
        .unwrap()
        .split(':')
        // .inspect(|line| println!("line: {}", line))
        .last()
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        // .inspect(|line| println!("line: {}", line))
        .map(|line| {
            let mut it = line.split(' ');
            let destination_start = it.next().unwrap().parse::<i64>().unwrap();
            let source_start = it.next().unwrap().parse::<i64>().unwrap();
            let length = it.next().unwrap().parse::<i64>().unwrap();
            RangeMap::new(destination_start, source_start, length - 1)
        })
        .collect::<Vec<_>>();

    let humidy_to_location_map = input
        .next()
        .unwrap()
        .split(':')
        // .inspect(|line| println!("line: {}", line))
        .last()
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        // .inspect(|line| println!("line: {}", line))
        .map(|line| {
            let mut it = line.split(' ');
            let destination_start = it.next().unwrap().parse::<i64>().unwrap();
            let source_start = it.next().unwrap().parse::<i64>().unwrap();
            let length = it.next().unwrap().parse::<i64>().unwrap();
            RangeMap::new(destination_start, source_start, length - 1)
        })
        .collect::<Vec<_>>();

    (
        seeds,
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidy_to_location_map,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    //     #[test]
    //     fn test_process() {
    //         let input = "seeds: 79 14 55 13

    // seed-to-soil map:
    // 50 98 2
    // 52 50 48

    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15

    // fertilizer-to-water map:
    // 49 53 8
    // 0 11 42
    // 42 0 7
    // 57 7 4

    // water-to-light map:
    // 88 18 7
    // 18 25 70

    // light-to-temperature map:
    // 45 77 23
    // 81 45 19
    // 68 64 13

    // temperature-to-humidity map:
    // 0 69 1
    // 1 0 69

    // humidity-to-location map:
    // 60 56 37
    // 56 93 4";
    //         let expected = 35;
    //         let result = process(input);
    //         assert_eq!(result, expected);
    //     }
}
