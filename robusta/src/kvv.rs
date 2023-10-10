use std::collections::HashMap;

use kvvliveapi::Stop as KvvStop;

use crate::ws_message::{Line, Train};

pub struct Stop {
    id: u32,
    kvv_stop: KvvStop,
}

#[derive(Clone, Default)]
pub struct Route {
    pub stops: Vec<Segment>,
    pub line_id: u32,
    pub destination: String,
}

#[derive(Clone, Default, Debug)]
pub struct Segment {
    pub start_id: u32,
    pub end_id: u32,
    pub positions: Vec<Point>,
}

#[derive(Clone, Default, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

const STOPS: &[&str] = &["Marktplatz"];

pub async fn kvv_stops() -> Vec<Stop> {
    let mut stops = Vec::new();
    for (id, stop) in STOPS.iter().enumerate() {
        let kvv_stop = kvvliveapi::search_by_name(stop)
            .unwrap()
            .drain(..)
            .next()
            .unwrap();

        stops.push(Stop {
            id: id as u32,
            kvv_stop,
        });
    }
    stops
}
type LineDepartures = HashMap<String, Vec<(u32, chrono::Duration)>>;

pub async fn fetch_departures(stops: &[Stop]) -> LineDepartures {
    let mut departures_per_line = HashMap::new();
    for stop in stops {
        let departures = kvvliveapi::departures_by_stop(&stop.kvv_stop.id).unwrap();
        let response_time = departures.timestamp;
        let mut departures_by_line_and_stop = HashMap::new();

        for departure in departures.departures {
            let line_id = departure.route;
            let delta = departure.time.time() - response_time.time();
            match departures_by_line_and_stop.get(&line_id) {
                Some(entry) if entry < &delta => (),
                _ => {
                    departures_by_line_and_stop.insert(line_id, delta);
                }
            }
        }
        for (line_id, delta) in departures_by_line_and_stop {
            let entry = departures_per_line.entry(line_id).or_insert_with(Vec::new);
            entry.push((stop.id, delta));
        }
    }

    departures_per_line
}

pub async fn train_positions_per_route(
    departures_per_line: LineDepartures,
    routes: &[Route],
) -> Vec<Train> {
    todo!()
}
