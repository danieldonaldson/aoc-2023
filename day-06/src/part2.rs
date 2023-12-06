use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

#[derive(Debug)]
struct Game {
    time: u64,
    winning_distance: u64,
}

#[derive(Debug)]
struct SimulationResult {
    winning_distance: u64,
    distances: Vec<u64>,
}

impl SimulationResult {
    fn get_winning_distances(&self) -> u64 {
        self.distances
            .clone()
            .into_par_iter()
            .filter(|x| *x > self.winning_distance)
            .count() as u64
    }
}

pub fn process(input: &str) -> u64 {
    let games = parse_input(input);
    dbg!(&games);
    // panic!("not implemented yet");
    let simulations = games
        .into_par_iter()
        .map(|game| SimulationResult {
            distances: simulate_game(game.time),
            winning_distance: game.winning_distance,
        })
        .collect::<Vec<_>>();
    // dbg!(&simulations);
    let winning_games = simulations
        .into_par_iter()
        .map(|result| result.get_winning_distances())
        .collect::<Vec<_>>();
    // dbg!(winning_games);

    winning_games.iter().product::<u64>()
}

fn simulate_game(time: u64) -> Vec<u64> {
    let mut distances = vec![0; time as usize];
    distances.par_iter_mut().enumerate().for_each(|(i, x)| {
        let speed = i as u64;
        let time_rest = time - speed;
        let distance = speed * time_rest;
        *x = distance;
    });
    distances
}

fn parse_input(input: &str) -> Vec<Game> {
    // dbg!(input);
    let line1 = input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .flat_map(|s| s.chars())
        .collect::<String>();
    let line2 = input
        .lines()
        .nth(1)
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .flat_map(|s| s.chars())
        .collect::<String>();

    // dbg!(&line1);
    let mut ret = Vec::new();
    let game = Game {
        time: line1.parse::<u64>().unwrap(),
        winning_distance: line2.parse::<u64>().unwrap(),
    };
    ret.push(game);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      71530
Distance:  940200";
        let expected = 288;
        let result = process(input);
        assert_eq!(result, expected);
    }
}
