use rayon::prelude::*;

#[derive(Debug)]
struct Game {
    time: u32,
    winning_distance: u32,
}

#[derive(Debug)]
struct SimulationResult {
    winning_distance: u32,
    distances: Vec<u32>,
}

impl SimulationResult {
    fn get_winning_distances(&self) -> u32 {
        self.distances
            .clone()
            .into_par_iter()
            .filter(|x| *x > self.winning_distance)
            .count() as u32
    }
}

pub fn process(input: &str) -> u32 {
    let games = parse_input(input);
    // dbg!(&games);
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

    winning_games.iter().product::<u32>()
}

fn simulate_game(time: u32) -> Vec<u32> {
    let mut distances = vec![0; time as usize];
    distances.par_iter_mut().enumerate().for_each(|(i, x)| {
        let speed = i as u32;
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
        .filter(|s| !s.is_empty());
    let mut line2 = input
        .lines()
        .nth(1)
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty());

    // dbg!(&line1);
    line1
        .map(move |time| {
            let time = time.parse::<u32>().unwrap();
            let winning_distance = line2.next().unwrap().parse::<u32>().unwrap();
            Game {
                time,
                winning_distance,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let expected = 288;
        let result = process(input);
        assert_eq!(result, expected);
    }
}
