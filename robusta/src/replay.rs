use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{Receiver, Sender};

const MS_PER_FRAME: i64 = 50;

#[derive(specta::Type, Debug, Deserialize)]
pub enum ReplayMessage {
    Pause,
    Goto(f64),
    Speed(f64),
    Disconnected,
}

#[derive(specta::Type, Debug, Serialize)]
pub enum ReplayResponse {
    Frame { time: String, game_state: String },
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

async fn run_replay_loop(state: &[Entry<'_>], mut recv: Receiver<ReplayMessage>, send: Sender<ReplayResponse>) {
    if state.is_empty() {
        return;
    }
    let start_time = state[0].0;
    let end_time = state[state.len() - 1].0;
    let duration = end_time - start_time;
    let duration_in_ms = duration.num_milliseconds() as f64;

    // configuration
    let mut paused = false;
    let mut position = start_time;
    let mut frame_time = chrono::Duration::milliseconds(MS_PER_FRAME);

    let mut interval = tokio::time::interval(std::time::Duration::from_millis(MS_PER_FRAME as u64));
    loop {
        interval.tick().await;
        while let Ok(msg) = recv.try_recv() {
            match msg {
                ReplayMessage::Pause => paused = !paused,
                ReplayMessage::Goto(progress) => position = start_time + chrono::Duration::milliseconds((progress * duration_in_ms) as i64),
                ReplayMessage::Speed(speed) => frame_time = chrono::Duration::milliseconds((MS_PER_FRAME as f64 * speed) as i64),
                ReplayMessage::Disconnected => return,
            }
        }

        if !paused {
            if position > end_time {
                send.send(ReplayResponse::End).await.unwrap();
            } else {
                let entry = find_nearest(state, position);
                send.send(ReplayResponse::Frame {
                    time: position.with_timezone(&chrono_tz::Europe::Berlin).to_string(),
                    game_state: entry.1.to_owned(),
                }).await.unwrap();
                position += frame_time;
            }
        }
    }
}

pub async fn replay<P: AsRef<std::path::Path>>(path: P, recv: Receiver<ReplayMessage>, send: Sender<ReplayResponse>) {
    let data = std::fs::read_to_string(path).unwrap();
    let state = parse_csv(&data);
    run_replay_loop(&state, recv, send).await;
}
