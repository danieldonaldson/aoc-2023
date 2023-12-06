use std::sync::atomic::AtomicI64;

use rayon::prelude::*;

#[derive(Debug)]
struct RangeMap {
    destination_start: i64,
    source_start: i64,
    source_end: i64,
}

impl RangeMap {
    fn new(destination_start: i64, source_start: i64, length: i64) -> Self {
        Self {
            destination_start,
            source_start,
            source_end: source_start + length,
        }
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

    let min_location = AtomicI64::new(std::i64::MAX);

    seeds.par_iter().for_each(|seed| {
        let seed_to_soil = seed_to_soil_map
            .par_iter()
            .find_any(|r| r.in_source_range(*seed))
            .map_or(*seed, |r| r.map(*seed));
        let soil_to_fertilizer = soil_to_fertilizer_map
            .par_iter()
            .find_any(|r| r.in_source_range(seed_to_soil))
            .map_or(seed_to_soil, |r| r.map(seed_to_soil));
        let fertilizer_to_water = fertilizer_to_water_map
            .par_iter()
            .find_any(|r| r.in_source_range(soil_to_fertilizer))
            .map_or(soil_to_fertilizer, |r| r.map(soil_to_fertilizer));
        let water_to_light = water_to_light_map
            .par_iter()
            .find_any(|r| r.in_source_range(fertilizer_to_water))
            .map_or(fertilizer_to_water, |r| r.map(fertilizer_to_water));
        let light_to_temperature = light_to_temperature_map
            .par_iter()
            .find_any(|r| r.in_source_range(water_to_light))
            .map_or(water_to_light, |r| r.map(water_to_light));
        let temperature_to_humidity = temperature_to_humidity_map
            .par_iter()
            .find_any(|r| r.in_source_range(light_to_temperature))
            .map_or(light_to_temperature, |r| r.map(light_to_temperature));
        let humidy_to_location = humidy_to_location_map
            .par_iter()
            .find_any(|r| r.in_source_range(temperature_to_humidity))
            .map_or(temperature_to_humidity, |r| r.map(temperature_to_humidity));

        min_location.fetch_min(humidy_to_location, std::sync::atomic::Ordering::Relaxed);
    });
    min_location.load(std::sync::atomic::Ordering::Relaxed)
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
