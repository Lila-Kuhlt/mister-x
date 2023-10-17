use std::collections::HashMap;

mod api;

use chrono::{Local, Utc};
use futures_util::FutureExt;
use trias::response::{Location, StopEventResponse};

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

const STOPS: &[(&str, &str)] = &[
    ("Arbeitsagentur", "de:08212:64"),
    ("Augartenstraße", "de:08212:74"),
    ("Barbarossaplatz", "de:08212:5003"),
    ("Durlacher Tor/KIT-Campus Süd", "de:08212:3"),
    ("Durlacher Tor/KIT-Campus Süd (U)", "de:08212:1001"),
    ("Ebertstraße", "de:08212:91"),
    ("Ettlinger Tor/Staatstheater", "de:08212:71"),
    ("Ettlinger Tor/Staatstheater (U)", "de:08212:1012"),
    ("Holtzstraße (Bus)", "de:08212:5509"),
    ("Karlstor/Bundesgerichtshof", "de:08212:61"),
    ("Kolpingplatz", "de:08212:63"),
    ("Kongresszentrum", "de:08212:72"),
    ("Kongresszentrum (U)", "de:08212:1013"),
    ("Kronenplatz", "de:08212:80"),
    ("Kronenplatz (U)", "de:08212:1002"),
    ("Marktplatz (Kaiserstr. U)", "de:08212:1003"),
    ("Marktplatz (Pyramide U) ", "de:08212:1011"),
    ("Mathystraße", "de:08212:62"),
    ("Mühlburger Tor", "de:08212:39"),
    ("Otto-Sachs-Straße", "de:08212:508"),
    ("Poststraße", "de:08212:98"),
    ("Rüppurrer Tor", "de:08212:85"),
    ("Schillerstraße", "de:08212:40"),
    ("Sophienstraße", "de:08212:602"),
    ("St. Vincentius Krankenhaus", "de:08212:5508"),
    ("Südendschule", "de:08212:5504"),
    ("Tivoli", "de:08212:84"),
    ("Weinbrennerplatz", "de:08212:603"),
    ("Welfenstraße", "de:08212:6218"),
    ("Werderstraße", "de:08212:83"),
    ("ZKM", "de:08212:65"),
    //("ZKM/Städtische Galerie", "de:08212:29"),
    ("Lessingstraße", "de:08212:507"),
    ("Europaplatz/Postgalerie", "de:08212:37"),
    ("Europaplatz/Postgalerie (U)", "de:08212:1004"),
    ("Marktplatz (Pyramide U)", "de:08212:1011"),
    ("Marktplatz (Kaiserstraße U)", "de:08212:1003"),
    ("Gebhardstraße", "de:08212:5004"),
    ("Gottesauer Platz/BGV", "de:08212:6"),
    ("Hauptbahnhof (Vorplatz)", "de:08212:89"),
    ("Hauptbahnhof Süd", "de:08212:88"),
    ("Hübschstraße", "de:08212:505"),
    ("Kongresszentrum", "de:08212:72"),
    ("Kongresszentrum (U)", "de:08212:1013"),
    ("Kronenplatz", "de:08212:80"),
    ("Kronenplatz (U)", "de:08212:1002"),
    ("Kunstakademie/Hochschule", "de:08212:7003"),
    ("Landesbausparkasse", "de:08212:604"),
    ("Yorckstraße", "de:08212:41"),
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
    STOPS.iter().position(|stop| &stop.0 == &name).unwrap() as u32
}

fn intermediate_points(start_id: u32, end_id: u32) -> Vec<Point> {
    let curves = parse_curves();
    let start = STOPS[start_id as usize];
    let end = STOPS[end_id as usize];
    let mut points = Vec::new();

    if let Some(p) = curves.iter().find(|(s, e, _)| s == start.0 && e == end.0) {
        points = p.2.clone();
    }

    points
}

pub async fn kvv_stops() -> Vec<Stop> {
    let mut stops = Vec::new();
    let mut futures = Vec::new();
    for stop in STOPS.iter() {
        tracing::trace!("fetching stop id for {}", stop.0);
        dotenv::dotenv().ok().unwrap();
        let api_endpoint = "https://projekte.kvv-efa.de/koberttrias/trias"; // Replace with your API endpoint
        let access_token = std::env::var("TRIAS_ACCESS_TOKEN").expect("TRIAS_ACCESS_TOKEN not set");
        let name = format!("{}", stop.1);

        let foo = trias::search_stops(name.clone(), access_token.clone(), api_endpoint, 2).await;
        /*
        for stop in foo.as_ref().unwrap() {
            println!(
                "(\"{}\", \"{}\"),",
                stop.stop_point.stop_point_name.text, stop.stop_point.stop_point_ref
            );
        }*/
        /*
        if stop != &foo.as_ref().unwrap()[0].stop_point.stop_point_name.text {
            println!(
                "stop name mismatch: {} != {}",
                stop,
                foo.unwrap()[0].stop_point.stop_point_name.text
            );
        }*/

        let stop = trias::search_stops(name, access_token, api_endpoint, 1);
        futures.push(stop);
    }
    let results = futures_util::future::join_all(futures).await;

    for (id, stop) in results.iter().enumerate() {
        let stop = stop.as_ref().unwrap().clone();
        let stop_point = stop[0].stop_point.clone();
        let position = stop[0].geo_position.clone();
        let kvv_stop = KvvStop {
            name: stop_point.stop_point_name.text,
            id: stop_point.stop_point_ref,
            lat: position.latitude.parse().unwrap(),
            lon: position.longitude.parse().unwrap(),
        };

        stops.push(Stop {
            id: id as u32,
            kvv_stop,
        });
    }
    stops
}

pub type LineDepartures = HashMap<String, Vec<(u32, chrono::Duration)>>;

pub async fn fetch_departures(stops: &[Stop]) -> LineDepartures {
    let mut departures_per_line = HashMap::new();
    let api_endpoint = "https://projekte.kvv-efa.de/koberttrias/trias"; // Replace with your API endpoint
    let access_token = &std::env::var("TRIAS_ACCESS_TOKEN").expect("TRIAS_ACCESS_TOKEN not set");

    let futures: Vec<_> = stops
        .iter()
        .map(|stop| {
            let name = stop.kvv_stop.id.clone();
            let access_token = access_token.clone();
            Box::pin(async move {
                (
                    stop.id,
                    trias::stop_events(name, access_token, 10, api_endpoint)
                        .await
                        .unwrap(),
                )
            })
        })
        .collect();

    let results = futures_util::future::join_all(futures).await;

    for (id, stops) in results.iter() {
        let response_time = Local::now().with_timezone(&chrono_tz::Europe::Berlin);

        for stop in stops {
            let mut departures_by_line_and_stop = HashMap::new();
            println!("stop id: {}", id);
            let departures = stop
                .stop_event_result
                .as_ref()
                .unwrap()
                .iter()
                .map(|x| &x.stop_event);
            for departure in departures {
                let line_id = &departure.service.journey_ref.clone();
                let time = &departure
                    .this_call
                    .call_at_stop
                    .service_departure
                    .as_ref()
                    .unwrap()
                    .timetabled_time;

                let time =
                    chrono::NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M:%SZ").unwrap();
                let delta = time.time() - response_time.time();

                let entry = departures_by_line_and_stop
                    .entry(line_id.clone())
                    .or_insert_with(Vec::new);
                entry.push((*id, delta));
                //dbg!(delta.num_minutes());

                let Some(previous_call) = &departure
                    .previous_call
                    .as_ref() else { println!("no previous call");continue; };
                let last_stop = &previous_call.last().unwrap().call_at_stop;

                let last_stop_time = &last_stop
                    .service_departure
                    .as_ref()
                    .unwrap()
                    .timetabled_time;
                let Some(last_stop_id) =
                    find_stop_by_kkv_id(&last_stop.stop_point_ref, KVV_STOPS.get().unwrap()) else {
                        println!("no last stop in KVV_STOPS {:?}", last_stop.stop_point_name.text);
                        continue;
                    };

                let last_stop_time =
                    chrono::NaiveDateTime::parse_from_str(last_stop_time, "%Y-%m-%dT%H:%M:%SZ")
                        .unwrap();

                // TODO: This only works during daytime
                let last_stop_delta = last_stop_time.time() - response_time.time();

                entry.push((last_stop_id.id, last_stop_delta));
            }
            for (line_id, mut deltas) in departures_by_line_and_stop {
                let entry = departures_per_line
                    .entry(line_id.clone())
                    .or_insert_with(Vec::new);
                entry.append(&mut deltas);
                entry.sort_by_key(|x| x.1);
                entry.dedup_by_key(|x| x.1.num_seconds());
                entry.dedup_by_key(|x| x.0);
            }
        }
    }

    departures_per_line
}

pub fn find_stop_by_id(id: u32, stops: &[Stop]) -> Option<&Stop> {
    stops.iter().find(|stop| stop.id == id)
}

pub fn find_stop_by_kkv_id<'a>(id: &str, stops: &'a [Stop]) -> Option<&'a Stop> {
    stops.iter().find(|stop| id.starts_with(&stop.kvv_stop.id))
}

pub fn points_on_route(start_stop_id: u32, end_stop_id: u32, stops: &[Stop]) -> Vec<Point> {
    let start_stop = find_stop_by_id(start_stop_id, stops).unwrap();
    let end_stop = find_stop_by_id(end_stop_id, stops).unwrap();

    let start = Point {
        x: start_stop.kvv_stop.lon as f32,
        y: start_stop.kvv_stop.lat as f32,
    };
    let end = Point {
        x: end_stop.kvv_stop.lon as f32,
        y: end_stop.kvv_stop.lat as f32,
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
    time_offset: chrono::Duration,
    line_id: &str,
    destination: &str,
    stops: &[Stop],
) -> Vec<Train> {
    let mut trains = Vec::new();
    let departures = departures_per_line.get(line_id).unwrap();
    let mut train_offsets = Vec::new();
    /*println!("departures for line {}", line_id);
    for stop_id in departures {
        println!(
            "{} {}",
            find_stop_by_id(stop_id.0, stops).unwrap().kvv_stop.name,
            stop_id.1.num_seconds()
        )
    }*/
    if let [last, current] = departures[..] {
        // TODO: handle panics
        let last_time = last.1 - time_offset;
        let current_time = current.1 - time_offset;
        let segment_duration = last_time - current_time - chrono::Duration::seconds(40);
        train_offsets.push(TrainPos {
            stop_id: last.0,
            next_stop_id: current.0,
            progress: 1.
                - (current_time.num_seconds() as f32 / segment_duration.num_seconds() as f32)
                    .clamp(0., 1.),
        });
    }
    for train_offset in train_offsets {
        let points = points_on_route(train_offset.stop_id, train_offset.next_stop_id, stops);
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
pub async fn fetch_departures_for_region() -> LineDepartures {
    let stops = KVV_STOPS.get().expect("KVV_STOPS not initialized");
    fetch_departures(&stops).await
}

pub async fn train_positions(
    departures_per_line: &LineDepartures,
    offset: chrono::Duration,
) -> Vec<Train> {
    let stops = KVV_STOPS.get().expect("KVV_STOPS not initialized");
    let mut trains = Vec::new();
    for line_id in departures_per_line.keys() {
        let positions =
            train_positions_per_route(departures_per_line.clone(), offset, line_id, line_id, stops);
        trains.extend(positions);
    }
    trains
}
