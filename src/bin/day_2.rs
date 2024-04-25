use std::fs;

#[derive(Debug)]
struct Game {
    id: u32,
    configuration: Vec<KubeSetColors>,
}

#[derive(Debug)]
struct KubeSetColors {
    red: Option<u32>,
    blue: Option<u32>,
    green: Option<u32>,
}

impl KubeSetColors {
    fn new() -> Self {
        KubeSetColors {
            red: None,
            blue: None,
            green: None,
        }
    }

    fn parse(line: &str) -> Self {
        let color_set = line.split(", ");
        let mut kube_sets = KubeSetColors::new();
        for col in color_set {
            if col.contains("red") {
                let kube_num = col.replace("red", "").trim().parse::<u32>();
                match kube_num {
                    Ok(val) => kube_sets.red = Some(val),
                    _ => {}
                }
            } else if col.contains("blue") {
                let kube_num = col.replace("blue", "").trim().parse::<u32>();
                match kube_num {
                    Ok(val) => kube_sets.blue = Some(val),
                    _ => {}
                }
            } else if col.contains("green") {
                let kube_num = col.replace("green", "").trim().parse::<u32>();
                match kube_num {
                    Ok(val) => kube_sets.green = Some(val),
                    _ => {}
                }
            }
        }

        kube_sets
    }

    fn can_contain_set(&self, (red, blue, green): (u32, u32, u32)) -> bool {
        self.red.unwrap_or_default() <= red
            && self.green.unwrap_or_default() <= green
            && self.blue.unwrap_or_default() <= blue
    }

    fn power(&self) -> u32 {
        self.red.unwrap_or(1) * self.green.unwrap_or(1) * self.blue.unwrap_or(1)
    }
}

impl Game {
    fn from(line: &str) -> Self {
        let mut game_conf = line.split(":");
        let id = game_conf
            .nth(0)
            .expect("It should exists")
            .replace("Game ", "")
            .parse::<u32>()
            .expect("Game must be present");

        let mut config: Vec<KubeSetColors> = vec![];
        let config_v = game_conf.nth(0);

        match config_v {
            Some(v) => {
                config = v
                    .split(";")
                    .map(|conf| KubeSetColors::parse(conf))
                    .collect()
            }
            _ => {}
        }

        Game {
            id,
            configuration: config,
        }
    }

    fn contains_set_colors(&self, (red, blue, green): (u32, u32, u32)) -> bool {
        for ele in &self.configuration {
            if !ele.can_contain_set((red, blue, green)) {
                return false;
            }
        }
        true
    }

    fn min_kubes_to_play_game(&self) -> KubeSetColors {
        let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);

        self.configuration
            .iter()
            .map(|c| {
                (
                    c.red.unwrap_or_default(),
                    c.green.unwrap_or_default(),
                    c.blue.unwrap_or_default(),
                )
            })
            .for_each(|(red, green, blue)| {
                if min_red < red {
                    min_red = red;
                }

                if min_blue < blue {
                    min_blue = blue;
                }

                if min_green < green {
                    min_green = green;
                }
            });

        KubeSetColors {
            red: Some(min_red),
            blue: Some(min_blue),
            green: Some(min_green),
        }
    }
}

fn part_two() {
    let sum: u32 = fs::read_to_string("assets/game_puzzle_input")
        .expect("file should exists")
        .lines()
        .map(|l| Game::from(l).min_kubes_to_play_game().power())
        .sum();

    println!("Part 2 :: Sum : {}", sum);
}

fn part_one() {
    // let colors = (9, 16, 5);
    let colors = (12, 14, 13);
    let sum: u32 = fs::read_to_string("assets/game_puzzle_input")
        .expect("file should exists")
        .lines()
        .map(|l| Game::from(l))
        .filter(|g| g.contains_set_colors(colors))
        .map(|g| g.id)
        .sum();

    println!("Part 1 :: Sum: {}", sum);
    assert_eq!(2204, sum);
}

fn main() {
    // https://adventofcode.com/2023/day/2
    part_one();
    part_two();
}

#[cfg(test)]
mod test {
    use crate::Game;

    #[test]
    fn main_boom() {
        let game = Game::from("Game 2: 16 blue, 9 red, 5 green; 8 red; 8 blue, 5 green, 12 red; 11 blue, 8 green, 17 red");

        assert_eq!(2, game.id);
        assert_eq!(4, game.configuration.len());
    }

    #[test]
    fn contains_colors() {
        let game = Game::from("Game 2: 16 blue, 9 red, 5 green; 8 red; 8 blue, 5 green, 12 red; 11 blue, 8 green, 17 red");

        assert!(game.contains_set_colors((17, 16, 8)));
    }

    #[test]
    fn does_not_contain_colors() {
        let game = Game::from("Game 2: 16 blue, 9 red, 5 green; 8 red; 8 blue, 5 green, 12 red; 11 blue, 8 green, 17 red");

        assert!(!game.contains_set_colors((29, 3, 12)));
    }

    #[test]
    fn min_collection_of_kubes_to_play_game() {
        let game =
            Game::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");

        let set_color = game.min_kubes_to_play_game();

        assert_eq!(Some(20), set_color.red);
        assert_eq!(Some(13), set_color.green);
        assert_eq!(Some(6), set_color.blue);
    }
}
