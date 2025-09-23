use std::process::Stdio;
use std::str::FromStr;

use color_eyre::{
    Result,
    eyre::{bail, eyre},
};
use serde::{Deserialize, Serialize};

const NUM_TRIALS: u8 = 25;
const THRESHOLD_FOR_CRACK: u8 = 15;

fn find_php_seed(results: &[u8]) -> Result<u32> {
    let child = std::process::Command::new("php-mt-seed")
        .args(results.iter().flat_map(|v| {
            [
                format!("{v}"),
                format!("{v}"),
                String::from("1"),
                String::from("5"),
            ]
        }))
        .stdout(Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;

    let s = String::from_utf8_lossy(&output.stdout);
    let seeds: Vec<_> = s
        .lines()
        .filter_map(|line| {
            if line.starts_with("seed = 0x") && line.ends_with(" (PHP 7.1.0+)") {
                let seed = line
                    .split_whitespace()
                    .nth(4)
                    .ok_or_else(|| eyre!("seed not found: {line}"))
                    .unwrap();
                Some(u32::from_str(seed).unwrap())
            } else {
                None
            }
        })
        .collect();

    match seeds.len() {
        1 => Ok(seeds[0]),
        0 => bail!("no seeds found"),
        n => bail!("multiple seeds found ({n})"),
    }
}

fn cards_from_seed(seed: u32) -> Result<Vec<u8>> {
    let child = std::process::Command::new("php")
        .arg("-r")
        .arg(format!(
            "srand({seed}); for ($i = 0; $i < {NUM_TRIALS}; $i++) {{ echo rand(1, 5) . \"\n\"; }}"
        ))
        .stdout(Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;
    let s = String::from_utf8_lossy(&output.stdout);

    Ok(s.lines().map(|s| u8::from_str(s).unwrap()).collect())
}

struct Api {
    client: reqwest::Client,
}

#[derive(Serialize)]
struct SubmitGuessRequest {
    game_id: u32,
    trial_num: u8,
    choice: u8,
}

impl Api {
    fn new() -> Result<Self> {
        Ok(Self {
            client: reqwest::Client::builder().cookie_store(true).build()?,
        })
    }

    async fn start_game(&self, num_trials: u8) -> Result<u32> {
        #[derive(Serialize)]
        struct StartGameRequest {
            num_trials: u8,
        }

        #[derive(Deserialize)]
        struct StartGameResponse {
            game_id: u32,
        }

        let response: StartGameResponse = self
            .client
            .post("https://zenertest.bensparks.co.uk/scripts/startGame.php")
            .json(&StartGameRequest { num_trials })
            .send()
            .await?
            .json()
            .await?;

        Ok(response.game_id)
    }

    async fn submit_guess(&self, request: SubmitGuessRequest) -> Result<(u8, bool)> {
        #[derive(Deserialize)]
        struct SubmitGuessResponse {
            correct_card: u8,
        }

        let response: SubmitGuessResponse = self
            .client
            .post("https://zenertest.bensparks.co.uk/scripts/submitGuess.php")
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok((
            response.correct_card,
            response.correct_card == request.choice,
        ))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let api = Api::new()?;

    let game_id = api.start_game(NUM_TRIALS).await?;
    println!("Game id: {game_id}");

    let mut score = 0;
    let mut cards = [0; THRESHOLD_FOR_CRACK as usize];
    for (i, card) in cards.iter_mut().enumerate() {
        let (correct_card, was_correct) = api
            .submit_guess(SubmitGuessRequest {
                game_id,
                trial_num: i as u8 + 1,
                choice: 1,
            })
            .await?;
        *card = correct_card;
        score += if was_correct { 1 } else { 0 };
    }

    println!("Got {score}/{THRESHOLD_FOR_CRACK} by guessing.");
    println!("Cracking seed...");

    let seed = find_php_seed(&cards)?;
    println!("Found seed: {seed}");
    let all_cards = cards_from_seed(seed)?;
    assert_eq!(all_cards[..THRESHOLD_FOR_CRACK as usize], cards);


    for (i, &choice) in all_cards[THRESHOLD_FOR_CRACK as usize..].iter().enumerate() {
        let (correct_card, was_correct) = api
            .submit_guess(SubmitGuessRequest {
                game_id,
                trial_num: THRESHOLD_FOR_CRACK + i as u8 + 1,
                choice,
            })
            .await?;
        if !was_correct {
            println!("WARNING: Incorrect guess: guessed {choice}, but was {correct_card}");
        }

        score += if was_correct { 1 } else { 0 };
    }

    println!("Final score: {score}/{NUM_TRIALS}");


    /*
    let response: StartGameResponse = client
        .post("https://zenertest.bensparks.co.uk/scripts/startGame.php")
        .json(&StartGameRequest { num_trials: NUM_TRIALS })
        .send()
        .await?
        .json()
        .await?;

    let game_id = response.game_id;
    println!("game id: {game_id}");

    println!("Result: {res:?}");

    print!("php-mt-seed ");
    for r in res {
        print!("{r} {r} 1 5 ");
    }
    println!();
    */

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_SEED: u32 = 0x545a6f95;
    const EXAMPLE_CARDS: &[u8] = &[
        2, 3, 1, 4, 4, 4, 3, 3, 3, 3, 1, 2, 5, 3, 5, 4, 1, 4, 2, 3, 3, 1, 1, 3, 2,
    ];

    #[test]
    #[ignore = "too slow"]
    fn find_seed() {
        let seed = find_php_seed(EXAMPLE_CARDS).unwrap();
        assert_eq!(seed, EXAMPLE_SEED);
    }

    #[test]
    fn generate_cards() {
        assert_eq!(cards_from_seed(EXAMPLE_SEED).unwrap(), EXAMPLE_CARDS);
    }
}
