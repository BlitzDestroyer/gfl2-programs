use std::collections::HashSet;

use clap::ValueEnum;
use const_format::formatcp;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;

use crate::debug_println;

const BOARD_SIZE: usize = 16;
const BASE_URL: &str = "https://gf2-h5ump45gacha-us-api.sunborngame.com";
const REFRESH_ENDPOINT: &str = formatcp!("{}/refresh", BASE_URL);
const PLAY_CLICK_ENDPOINT: &str = formatcp!("{}/play_click", BASE_URL);
const INFO_ENDPOINT: &str = formatcp!("{}/info", BASE_URL);
const GACHA_ENDPOINT: &str = formatcp!("{}/gacha", BASE_URL);

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde Json error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Auth token missing")]
    AuthTokenMissing,
    #[error("{0}")]
    Info(String),
}

#[derive(Debug, Deserialize)]
pub struct GflEndpointResponse<T> {
    #[serde(rename = "Code")]
    pub code: i32,
    #[serde(rename = "Message")]
    pub message: String,
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct PlayInfo {
    pub flag: i32,
    pub info: Vec<String>,
    pub max_score: i32,
    pub real_score: i32,
    pub score: i32,
    pub stage: i32,
    pub times: i32,
}

#[derive(Debug, Deserialize)]
pub struct RefreshData {
    pub play_info: PlayInfo,
    pub play_times: i32,
}

#[derive(Debug, Deserialize)]
pub struct PlayClickData {
    pub card_id: String,
    pub gacha_num: i32,
    pub num: i32,
    pub play_info: PlayInfo,
    pub play_times: i32,
}

#[derive(Debug, Deserialize)]
pub struct InfoData {
    pub be_assist_num: i32,
    pub code: String,
    pub day_can_get_score: i32,
    pub gacha_num: i32,
    pub gacha_score: i32,
    pub game_uid: i32,
    pub play_info: PlayInfo,
    pub play_num: i32,
    pub task_info: TaskInfo,
}

#[derive(Debug, Deserialize)]
pub struct TaskInfo {
    pub can_get_assist: i32,
    pub game_login: i32,
    pub login_h5: i32,
    pub share: i32,
}

#[derive(Debug, Deserialize)]
pub struct GachaData {
    pub is_code: i32,
    pub name: String,
    pub pic: String,
    pub record_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct PuzzleConfig {
    pub auth_token: String,
}

#[derive(Debug)]
pub struct LevaPuzzleClient {
    _auth_token: String,
    client: reqwest::Client,
}

impl LevaPuzzleClient {
    pub fn new(auth_token: &str) -> Result<Self, PuzzleError> {
        let auth_token = if auth_token.is_empty() {
            get_auth_token()?
        } 
        else {
            auth_token.to_string()
        };

        if auth_token.is_empty() {
            return Err(PuzzleError::AuthTokenMissing);
        }

        let mut header_map = HeaderMap::new();
        header_map.insert("Authorization", auth_token.parse().unwrap());
        let client = reqwest::Client::builder()
            .default_headers(header_map)
            .build()?;

        Ok(LevaPuzzleClient {
            _auth_token: auth_token,
            client,
        })
    }

    async fn get_info_data_response(&self) -> Result<GflEndpointResponse<InfoData>, PuzzleError> {
        let response = self.client.get(INFO_ENDPOINT)
            .send()
            .await?;

        let status_code = response.status();
        debug_println!("Info response status code: {}", status_code);
        if status_code != 200 {
            let response = response
                .json::<GflEndpointResponse<bool>>()
                .await?;
            let err_msg = format!("Failed to get game info. Reason: {}", response.message);
            eprintln!("{}", err_msg);

            Err(PuzzleError::Info(err_msg))
        }
        else{
            let response = response
                .json::<GflEndpointResponse<InfoData>>()
                .await?;

            Ok(response)
        }
    }

    async fn refresh_game_state(&self) -> Result<GflEndpointResponse<RefreshData>, PuzzleError> {
        let response = self.client.post(REFRESH_ENDPOINT)
            .json(&json!({}))
            .send()
            .await?;

        let status_code = response.status();
        if status_code != 200 {
            let response = response
                .json::<GflEndpointResponse<bool>>()
                .await?;

            let err_msg = format!("Failed to refresh game state. Reason: {}", response.message);
            eprintln!("{}", err_msg);
            Err(PuzzleError::Info(err_msg.to_string()))
        }
        else{
            let response = response
                .json::<GflEndpointResponse<RefreshData>>()
                .await?;

            Ok(response)
        }

        
    }

    async fn play_click(&self, index: usize) -> Result<GflEndpointResponse<PlayClickData>, PuzzleError> {
        let response = self.client.post(PLAY_CLICK_ENDPOINT)
            .json(&json!({ "index": index }))
            .send()
            .await?;

        let status_code = response.status();
        debug_println!("Play click response status code: {}", status_code);

        if status_code != 200 {
            let response = response
                .json::<GflEndpointResponse<bool>>()
                .await?;
            let err_msg = format!("Failed to play click. Reason: {}", response.message);
            eprintln!("{}", err_msg);

            Err(PuzzleError::Info(err_msg))
        }
        else{
            let response = response
                .json::<GflEndpointResponse<PlayClickData>>()
                .await?;

            Ok(response)
        }
    }

    async fn roll_gacha(&self) -> Result<GflEndpointResponse<GachaData>, PuzzleError> {
        let response = self.client.post(GACHA_ENDPOINT)
            .send()
            .await?;

        let status_code = response.status();
        debug_println!("Gacha response status code: {}", status_code);

        if status_code != 200 {
            let response = response
                .json::<GflEndpointResponse<bool>>()
                .await?;

            let err_msg = format!("Failed to roll gacha. Reason: {}", response.message);
            eprintln!("{}", err_msg);
            
            Err(PuzzleError::Info(err_msg))
        }
        else{
            let response = response
                .json::<GflEndpointResponse<GachaData>>()
                .await?;

            Ok(response)
        }
    }
}

#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum Attempts {
    None,
    One,
    All
}

impl std::fmt::Display for Attempts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attempts::None => write!(f, "none"),
            Attempts::One => write!(f, "one"),
            Attempts::All => write!(f, "all"),
        }
    }
}

pub async fn solve_puzzle(client: &LevaPuzzleClient, attempts: Attempts) -> Result<(), PuzzleError> {
    if matches!(attempts, Attempts::None) {
        debug_println!("No puzzle attempts specified. Skipping puzzle solving.");
        return Ok(());
    }

    let mut response = client.get_info_data_response().await?;
    let info_data = &response.data;
    if info_data.play_num < 1 {
        let err_msg = "No available plays left for today.";
        eprintln!("{}", err_msg);
        return Err(PuzzleError::Info(err_msg.to_string()));
    }

    match attempts {
        Attempts::None => unreachable!(),
        Attempts::One => {
            solve_puzzle_helper(client, &info_data).await?;
        },
        Attempts::All => {
            for _ in 0..info_data.play_num {
                let info_data = &response.data;
                solve_puzzle_helper(client, &info_data).await?;
                tokio::time::sleep(std::time::Duration::from_millis(500)).await; // Small delay to avoid spamming the server
                response = client.get_info_data_response().await?;
            }
        },
    }

    Ok(())
}

async fn solve_puzzle_helper(client: &LevaPuzzleClient, info_data: &InfoData) -> Result<(), PuzzleError> {
    let mut seen = vec![String::new(); BOARD_SIZE];
    let mut solved_indices = HashSet::new();
    let (ongoing, mut last_seen, mut last_seen_index) = determine_current_game_state(info_data, &mut seen, &mut solved_indices);
    if !ongoing {
        println!("No ongoing game found. Starting a new game...");
        seen.iter_mut().for_each(|s| *s = String::new());
        solved_indices.clear();

        client.refresh_game_state().await?;
    }
    else{
        println!("Resuming ongoing game...");
        debug_println!("Seen cards: {:?}", seen);
        debug_println!("Solved indices: {:?}", solved_indices);
    }

    println!("Starting Auto Play Leva's Memory Puzzle...");
    while solved_indices.len() < BOARD_SIZE {
        let index = get_index_to_click(&seen, &solved_indices, &last_seen, last_seen_index);
        debug_println!("Clicking index: {}", index);
        if index == usize::MAX {
            let err_msg = "No valid index to click found. This should not happen.";
            eprintln!("{}", err_msg);
            return Err(PuzzleError::Info(err_msg.to_string()));
        }

        let response = client.play_click(index).await?;

        if response.message != "OK" {
            let err_msg = format!("Error from server: {}", response.message);
            eprintln!("{}", err_msg);
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            continue;
        }

        let data_obj = response.data;
        let card_id = data_obj.card_id;
        if card_id.is_empty() {
            let err_msg = "No card ID received from server.";
            eprintln!("{}", err_msg);
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            continue;
        }

        seen[index] = card_id.clone();
        if !last_seen.is_empty() {
            if last_seen == card_id {
                solved_indices.insert(index);
                solved_indices.insert(last_seen_index as usize);
            }
            
            last_seen.clear();
            last_seen_index = -1;
        }
        else {
            last_seen = card_id;
            last_seen_index = index as isize;
        }

        debug_println!("Current seen cards: {:?}", seen);
        debug_println!("Solved indices: {:?}", solved_indices);
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;  // Small delay to avoid spamming the server
    }

    println!("Puzzle solved!");

    Ok(())
}

fn determine_current_game_state(data: &InfoData, seen: &mut Vec<String>, solved_indices: &mut HashSet<usize>) -> (bool, String, isize) {
    let play_info = &data.play_info;
    let flag = play_info.flag;
    let info = &play_info.info;
    if info.len() != BOARD_SIZE {
        return (false, String::new(), -1);
    }

    let mut not_matched = "";
    let mut not_matched_index: isize = -1;
    let mut matches = 0;
    for i in 0..BOARD_SIZE {
        debug_println!("Card at index {} is {}", i, info[i]);
        seen[i] = info[i].clone();
        if info[i].is_empty() {
            continue;
        }

        let mut match_found = false;
        for j in (i + 1)..BOARD_SIZE {
            if info[j] == info[i] {
                debug_println!("Found match for card {} at indices {} and {}", info[i], i, j);
                solved_indices.insert(i);
                solved_indices.insert(j);
                match_found = true;
                matches += 1;
                break;
            }
        }

        if !match_found {
            not_matched = &info[i];
            not_matched_index = i as isize;
        }
    }

    if flag != 2 {
        not_matched = "";
        not_matched_index = -1;
    }

    debug_println!("Current matches: {}", matches);

    (matches < BOARD_SIZE / 2, not_matched.to_string(), not_matched_index)
}

fn get_index_to_click(seen: &Vec<String>, solved_indices: &HashSet<usize>, last_seen: &String, last_seen_index: isize) -> usize {
    if !last_seen.is_empty() {
        debug_println!("Looking for match for revealed card: {}", last_seen);
        for i in 0..BOARD_SIZE {
            if seen[i] == *last_seen && !solved_indices.contains(&i) && i as isize != last_seen_index {
                return i;
            }
        }
    }

    debug_println!("No revealed card or no match found. Looking for existing match.");
    for i in 0..BOARD_SIZE {
        if solved_indices.contains(&i) || seen[i].is_empty() {
            continue;
        }

        for j in (i + 1)..BOARD_SIZE {
            if solved_indices.contains(&j) || seen[j].is_empty() {
                continue;
            }

            if seen[i] == seen[j] {
                debug_println!("Found existing match for card {} at indices {} and {}", seen[i], i, j);
                return i;
            }
        }
    }

    debug_println!("No existing match found. Looking for first unseen card.");
    for i in 0..BOARD_SIZE {
        if !solved_indices.contains(&i) && seen[i].is_empty() {
            return i;
        }
    }

    usize::MAX // Should not happen
}

fn get_auth_token() -> Result<String, PuzzleError> {
    let result = std::fs::read_to_string("leva_puzzle_config.json");
    match result {
        Ok(config_str) => {
            let config: PuzzleConfig = serde_json::from_str(&config_str)?;
            Ok(config.auth_token)
        },
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                let mut buf = String::new();
                print!("Enter your authentication token: ");
                std::io::stdin().read_line(&mut buf)?;
                let auth_token = buf.trim().to_string();
                Ok(auth_token)
            },
            _ => Err(PuzzleError::Io(err)),
        },
    }
}

pub async fn roll_gacha(client: &LevaPuzzleClient, attempts: Attempts) -> Result<(), PuzzleError> {
    if matches!(attempts, Attempts::None) {
        debug_println!("No gacha attempts specified. Skipping gacha roll.");
        return Ok(());
    }

    let info_response = client.get_info_data_response().await?;
    let gacha_num = info_response.data.gacha_num;
    if gacha_num < 1 {
        let err_msg = "No gacha attempts left.";
        eprintln!("{}", err_msg);
        return Err(PuzzleError::Info(err_msg.to_string()));
    }

    match attempts {
        Attempts::None => unreachable!(),
        Attempts::One => {
            roll_gacha_helper(client).await?;
        },
        Attempts::All => {
            for _ in 0..gacha_num {
                roll_gacha_helper(client).await?;
                tokio::time::sleep(std::time::Duration::from_millis(500)).await; // Small delay to avoid spamming the server
            }
        }
    }

    Ok(())
}

async fn roll_gacha_helper(client: &LevaPuzzleClient) -> Result<(), PuzzleError> {
    println!("Rolling gacha...");
    let gacha_response = client.roll_gacha().await?;
    if gacha_response.message != "OK" {
        let err_msg = format!("Error from server: {}", gacha_response.message);
        eprintln!("{}", err_msg);
        return Err(PuzzleError::Info(err_msg));
    }

    let gacha_data = gacha_response.data;
    println!("Gacha Result: {}", gacha_data.name);

    Ok(())
}