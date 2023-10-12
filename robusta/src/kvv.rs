use std::collections::HashMap;

mod api;

use crate::ws_message::{Line, Train};

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Stop {
    id: u32,
    kvv_stop: KvvStop,
}

/// Information about a tram station
#[derive(Debug, specta::Type, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct KvvStop {
    /// human readable stop name
    pub name: String,
    /// internal stop id
    pub id: String,
    /// position latitude
    pub lat: f64,
    /// position longitude
    pub lon: f64,
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

/*

 "Arbeitsagentur",
 "Augartenstraße",
 "Barbarossaplatz",
 "Durlacher Tor/KIT-Campus Süd",
 "Durlacher Tor/KIT-Campus Süd (U)",
 "Ebertstraße",
 "Ettlinger Tor/Staatstheater",
 "Ettlinger Tor/Staatstheater (U)",
 "Europaplatz/Postgalerie",
 "Europaplatz/Postgalerie (U)",
    "Hauptbahnhof (Vorplatz)",
    "Holtzstraße (Bus)",
    "Karlstor/Bundesgerichtshof",
    "Kolpingplatz",
    "Kongresszentrum (U)",
    "Kronenplatz",
    "Kronenplatz (U)",

*/
const STOPS: &[&str] = &[
    "Marktplatz (Kaiserstr. U) ",
    "Marktplatz (Pyramide U) ",
    "Mathystraße",
    "Mühlburger Tor",
    "Otto-Sachs-Straße",
    "Poststraße",
    "Rüppurrer Tor",
    "Schillerstraße ",
    "Sophienstraße",
    "St. Vincentius Krankenhaus (Bus)",
    "Südendschule (Bus)",
    "Tivoli",
    "Weinbrennerplatz",
    "Welfenstraße",
    "Werderstraße",
    "ZKM",
    "ZKM (Bus)",
    "Lessingstraße",
];

const CURVES_STR: &str = include_str!("../data/route_curves.csv");

fn parse_curve(line: &str) -> (String, String, Vec<Point>) {
    let mut parts = line.split(';');
    let start = parts.next().unwrap().parse().unwrap();
    let end = parts.next().unwrap().parse().unwrap();
    let points = parts
        .map(|point| {
            let mut coords = point.split(',');
            let x = coords.next().unwrap().trim().parse().unwrap();
            let y = coords.next().unwrap().trim().parse().unwrap();
            Point { x, y }
        })
        .collect();
    (start, end, points)
}

fn parse_curves() -> Vec<(String, String, Vec<Point>)> {
    CURVES_STR.lines().map(parse_curve).collect()
}

fn stop_id_by_name(name: &str) -> u32 {
    STOPS.iter().position(|stop| stop == &name).unwrap() as u32
}

fn intermediate_points(start_id: u32, end_id: u32) -> Vec<Point> {
    let curves = parse_curves();
    let start = STOPS[start_id as usize];
    let end = STOPS[end_id as usize];
    let mut points = Vec::new();

    if let Some(p) = curves.iter().find(|(s, e, _)| s == start && e == end) {
        points = p.2.clone();
    }

    points
}

pub async fn kvv_stops() -> Vec<Stop> {
    let mut stops = Vec::new();
    for (id, stop) in STOPS.iter().enumerate() {
        tracing::trace!("fetching stop id for {}", stop);

        let stop_id = api::fetch_stop_id(stop).await.unwrap();
        let kvv_stop = api::fetch_stop_by_id(&stop_id).await.unwrap();

        stops.push(Stop {
            id: id as u32,
            kvv_stop,
        });
    }
    stops
}
type LineDepartures = HashMap<(String, String), Vec<(u32, Vec<chrono::Duration>)>>;

pub async fn fetch_departures(stops: &[Stop]) -> LineDepartures {
    let mut departures_per_line = HashMap::new();
    for stop in stops {
        let departures = api::fetch_departures(&stop.kvv_stop.id).await.unwrap();
        let response_time = departures.timestamp;
        let mut departures_by_line_and_stop = HashMap::new();

        for departure in departures.departures {
            let line_id = departure.route;
            let delta = departure.time.time() - response_time.time();

            let entry = departures_by_line_and_stop
                .entry((line_id, departure.destination))
                .or_insert_with(Vec::new);
            entry.push(delta);
        }
        for (line_id, delta) in departures_by_line_and_stop {
            let entry = departures_per_line.entry(line_id).or_insert_with(Vec::new);
            entry.push((stop.id, delta));
        }
    }

    departures_per_line
}

pub fn find_stop_by_id(id: u32, stops: &[Stop]) -> Option<&Stop> {
    stops.iter().find(|stop| stop.id == id)
}

pub fn points_on_route(start_stop_id: u32, end_stop_id: u32, stops: &[Stop]) -> Vec<Point> {
    let start_stop = find_stop_by_id(start_stop_id, stops).unwrap();
    let end_stop = find_stop_by_id(end_stop_id, stops).unwrap();

    let start = Point {
        x: start_stop.kvv_stop.lat as f32,
        y: start_stop.kvv_stop.lon as f32,
    };
    let end = Point {
        x: end_stop.kvv_stop.lat as f32,
        y: end_stop.kvv_stop.lon as f32,
    };
    let mut points = vec![start];
    points.extend(intermediate_points(start_stop_id, end_stop_id));
    points.push(end);

    points
}

struct TrainPos {
    stop_id: u32,
    next_stop_id: u32,
    progress: f32,
}

pub fn interpolate_segment(points: &[Point], progress: f32) -> Point {
    let total_length = points
        .windows(2)
        .map(|slice| {
            let [start, end] = slice else {
                 panic!("slice has wrong length");
             };
            let dx = end.x - start.x;
            let dy = end.y - start.y;
            (dx * dx + dy * dy).sqrt()
        })
        .sum::<f32>();
    let length = progress * total_length;

    let mut current_length = 0.0;
    for slice in points.windows(2) {
        let [start, end] = slice else {
             panic!("slice has wrong length");
         };
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let segment_length = (dx * dx + dy * dy).sqrt();
        if current_length + segment_length > length {
            let progress = (length - current_length) / segment_length;
            return Point {
                x: start.x + progress * dx,
                y: start.y + progress * dy,
            };
        }
        current_length += segment_length;
    }
    let end = points.last().unwrap();
    Point { x: end.x, y: end.y }
}

pub fn train_positions_per_route(
    departures_per_line: LineDepartures,
    line_id: &str,
    destination: &str,
    stops: &[Stop],
) -> Vec<Train> {
    let mut trains = Vec::new();
    let departures = departures_per_line
        .get(&(line_id.to_owned(), destination.to_owned()))
        .unwrap();
    let mut train_offsets = Vec::new();
    for slice in departures.windows(2) {
        if let [last, current] = slice {
            if last.1 > current.1 {
                // TODO: handle panics
                let segment_duration = last.1[0] - current.1[0];
                train_offsets.push(TrainPos {
                    stop_id: last.0,
                    next_stop_id: current.0,
                    progress: current.1[0].num_seconds() as f32
                        / segment_duration.num_seconds() as f32,
                });
            }
        }
    }
    for train_offset in train_offsets {
        let points = points_on_route(train_offset.stop_id, train_offset.next_stop_id, &stops);
        let position = interpolate_segment(&points, train_offset.progress);

        trains.push(Train {
            id: 0,
            long: position.x,
            lat: position.y,
            line_id: line_id.to_owned(),
            direction: destination.to_owned(),
        });
    }
    trains
}

pub static KVV_STOPS: std::sync::OnceLock<Vec<Stop>> = std::sync::OnceLock::new();

pub async fn init() {
    let stops = kvv_stops().await;
    KVV_STOPS.set(stops).expect("failed to set KVV_STOPS");
}

pub async fn train_positions() -> Vec<Train> {
    let stops = KVV_STOPS.get().expect("KVV_STOPS not initialized");
    let departures_per_line = fetch_departures(&stops).await;

    let mut trains = Vec::new();
    for (line_id, destination) in departures_per_line.keys() {
        let positions =
            train_positions_per_route(departures_per_line.clone(), line_id, destination, &stops);
        trains.extend(positions);
    }
    trains
}
