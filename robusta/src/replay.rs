use std::path::Path;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{Receiver, Sender};

pub const PATH: &str = "replays";
const MS_PER_FRAME: i64 = 50;

#[derive(specta::Type, Debug, Deserialize)]
pub enum ReplayMessage {
    /// Play a game from a file.
    Play(String),
    /// Pause/unpause the replay.
    Pause,
    /// Go to a specific position. The value is between 0 and 1.
    Goto(f64),
    /// Set the playback speed. The value is positive.
    Speed(f64),
    /// The client disconnected.
    Disconnected,
}

#[derive(specta::Type, Debug, Serialize)]
pub enum ReplayResponse {
    Start,
    Frame { time: String, progress: f64, game_state: String },
    Files(Vec<String>),
    End,
}

type Entry<'a> = (DateTime<FixedOffset>, &'a str);

fn parse_csv(data: &str) -> Vec<Entry> {
    data
        .lines()
        .map(|line| {
            let (time, game_state) = line.split_once(", ").unwrap();
            // don't parse the game state, as we need to serialize it for sending anyway
            (DateTime::parse_from_rfc3339(time).unwrap(), game_state)
        })
        .collect()
}

/// Find the entry whose time is the closest to `time`.
///
/// # Panics
///
/// Panics if `time` is out of range, i.e. before the first time or after the last time.
fn find_nearest<'a>(state: &[Entry<'a>], time: DateTime<FixedOffset>) -> Entry<'a> {
    match state.binary_search_by_key(&time, |(time, _)| *time) {
        Ok(i) => state[i],
        Err(i) => std::cmp::min_by_key(state[i - 1], state[i], |entry| (entry.0 - time).abs()),
    }
}

/// Replay a game.
///
/// # Panics
///
/// Panics if `state` is empty.
///
/// # Returns
///
/// * `Ok(file)`: play another file
/// * `Err(msg)`: client disconnected
async fn run_replay_loop(state: &[Entry<'_>], recv: &mut Receiver<ReplayMessage>, send: &Sender<ReplayResponse>) -> Result<String, ()> {
    assert!(!state.is_empty());

    let start_time = state[0].0;
    let end_time = state[state.len() - 1].0;
    let duration = end_time - start_time;
    let duration_in_ms = duration.num_milliseconds() as f64;

    // configuration
    let mut paused = true;
    let mut position = start_time;
    let mut frame_time = chrono::Duration::milliseconds(MS_PER_FRAME);

    macro_rules! send_frame {
        () => {
            let entry = find_nearest(state, position);
            let datetime = position.with_timezone(&chrono_tz::Europe::Berlin);
            send.send(ReplayResponse::Frame {
                time: format!("{} {}", datetime.naive_local().format("%F %T"), datetime.offset()),
                progress: ((position - start_time).num_milliseconds() as f64 / duration_in_ms).clamp(0.0, 1.0),
                game_state: entry.1.to_owned(),
            }).await.unwrap();
        };
    }

    let mut interval = tokio::time::interval(std::time::Duration::from_millis(MS_PER_FRAME as u64));
    send.send(ReplayResponse::Start).await.unwrap();
    send_frame!();
    loop {
        interval.tick().await;
        while let Ok(msg) = recv.try_recv() {
            match msg {
                ReplayMessage::Play(file) => return Ok(file),
                ReplayMessage::Pause => paused = !paused,
                ReplayMessage::Goto(progress) => {
                    position = start_time + chrono::Duration::milliseconds((progress * duration_in_ms) as i64);
                    send_frame!();
                }
                ReplayMessage::Speed(speed) => {
                    frame_time = chrono::Duration::milliseconds((MS_PER_FRAME as f64 * speed).clamp(0.0, duration_in_ms) as i64);
                }
                ReplayMessage::Disconnected => return Err(()),
            }
        }

        if !paused {
            if position >= end_time {
                position = end_time;
                send_frame!();
                send.send(ReplayResponse::End).await.unwrap();
                paused = true;
            } else {
                send_frame!();
                position += frame_time;
            }
        }
    }
}

pub async fn replay(mut recv: Receiver<ReplayMessage>, send: Sender<ReplayResponse>) {
    while let Some(msg) = recv.recv().await {
        match msg {
            ReplayMessage::Play(file) => {
                let mut file = file;
                while !file.is_empty() {
                    let data = std::fs::read_to_string(Path::new(PATH).join(&file)).unwrap();
                    let state = parse_csv(&data);
                    if state.is_empty() {
                        break;
                    }
                    match run_replay_loop(&state, &mut recv, &send).await {
                        Ok(new_file) => file = new_file,
                        Err(_) => return,
                    }
                }
            }
            ReplayMessage::Pause => {
                send.send(ReplayResponse::End).await.unwrap();
            }
            ReplayMessage::Goto(_) => {}
            ReplayMessage::Speed(_) => {}
            ReplayMessage::Disconnected => return,
        }
    }
}
