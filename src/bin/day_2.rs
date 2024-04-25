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
}

fn main() {
    // https://adventofcode.com/2023/day/2
    // let colors = (9, 16, 5);
    let colors = (12, 14, 13);
    let sum: u32 = fs::read_to_string("assets/game_puzzle_input")
        .expect("file should exists")
        .lines()
        .map(|l| Game::from(l))
        .filter(|g| g.contains_set_colors(colors))
        .map(|g| g.id)
        .sum();

    println!("Sum is: {}", sum);
    assert_eq!(2204, sum);
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
}
