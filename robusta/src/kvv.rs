use std::collections::HashMap;

mod api;

use chrono::{FixedOffset, Local, Utc};
use futures_util::FutureExt;
use tracing::debug;
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
    /*
    ("Durlacher Tor/KIT-Campus Süd", "de:08212:3"),
    ("Durlacher Tor/KIT-Campus Süd (U)", "de:08212:1001"),
    ("Poststraße", "de:08212:98"),
    ("Otto-Sachs-Straße", "de:08212:508"),
    */
    //
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
    let start = parts.next().unwrap();
    let end = parts.next().unwrap();
    let points = parts
        .map(|point| {
            let mut coords = point.split(',');
            let x = coords.next().unwrap().trim().parse().unwrap();
            let y = coords.next().unwrap().trim().parse().unwrap();
            Point { x, y }
        })
        .collect();
    (start.to_owned(), end.to_owned(), points)
}

fn parse_curves() -> Vec<(String, String, Vec<Point>)> {
    CURVES_STR.lines().map(parse_curve).collect()
}

fn stop_id_by_name(name: &str) -> u32 {
    STOPS.iter().position(|stop| &stop.0 == &name).unwrap() as u32
}

fn intermediate_points(start_id: &str, end_id: &str) -> Vec<Point> {
    let curves = parse_curves();
    let start = STOPS
        .iter()
        .find(|stop| start_id.starts_with(stop.1))
        .unwrap();
    let end = STOPS
        .iter()
        .find(|stop| end_id.starts_with(stop.1))
        .unwrap();
    let mut points = Vec::new();

    if let Some(p) = curves.iter().find(|(s, e, _)| s == start.0 && e == end.0) {
        points = p.2.clone();
    }
    if let Some(p) = curves.iter().find(|(s, e, _)| e == start.0 && s == end.0) {
        points = p.2.clone();
        points.reverse();
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
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Journey {
    stops: HashMap<JourneyRef, chrono::NaiveDateTime>,
    line_name: String,
    destination: String,
}

impl Journey {
    fn new(line_name: String, destination: String) -> Self {
        Self {
            stops: HashMap::new(),
            line_name,
            destination,
        }
    }
}

type JourneyRef = String;
type StopRef = String;
pub type LineDepartures = HashMap<JourneyRef, Journey>;

pub fn parse_time(time: &str) -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M:%SZ").unwrap()
}

pub async fn fetch_departures(stops: &[Stop]) -> LineDepartures {
    let api_endpoint = "https://projekte.kvv-efa.de/koberttrias/trias"; // Replace with your API endpoint
    let access_token = &std::env::var("TRIAS_ACCESS_TOKEN").expect("TRIAS_ACCESS_TOKEN not set");

    let futures: Vec<_> = stops
        .iter()
        .map(|stop| {
            let name = stop.kvv_stop.id.clone();
            let access_token = access_token.clone();
            Box::pin(async move {
                trias::stop_events(name, access_token, 10, api_endpoint)
                    .await
                    .unwrap_or_default()
            })
        })
        .collect();

    let results = futures_util::future::join_all(futures).await;

    let mut jorneys = HashMap::new();

    for stop_events in results
        .iter()
        .flatten()
        .flat_map(|x| x.stop_event_result.as_ref())
    {
        for stop in stop_events {
            let service = &stop.stop_event.service;
            let journey = &service.journey_ref;
            let line_name = service.service_section.published_line_name.text.clone();
            let destination = service.destination_text.text.clone();
            let this_call = &stop.stop_event.this_call;
            let previous_call = &stop.stop_event.previous_call.iter().flatten();
            let next_call = &stop.stop_event.onward_call.iter().flatten();
            let entry = jorneys
                .entry(journey.clone())
                .or_insert(Journey::new(line_name, destination));
            let calls = previous_call
                .clone()
                .chain(std::iter::once(this_call))
                .chain(next_call.clone());

            for call in calls {
                let departure = &call.call_at_stop.service_departure.as_ref();
                let arrival = &call.call_at_stop.service_arrival.as_ref();
                let time = match (departure, arrival) {
                    (Some(departure), _) => parse_time(&departure.timetabled_time),
                    (_, Some(arrival)) => parse_time(&arrival.timetabled_time),
                    _ => {
                        println!("no departure or arrival time");
                        continue;
                    }
                };
                let stop_ref = call.call_at_stop.stop_point_ref.clone();
                let Some(proper_stop_ref) = find_stop_by_kkv_id(&stop_ref, stops) else { continue; };
                let short_ref = proper_stop_ref.kvv_stop.id.clone();
                let current_time = entry.stops.get(&short_ref);
                if let Some(current_time) = current_time {
                    if *current_time < time {
                        continue;
                    }
                }
                let old = entry
                    .stops
                    .insert(proper_stop_ref.kvv_stop.id.clone(), time);
                /*if let Some(old) = old {
                    if old != time {
                        tracing::warn!(
                            "different times for same stop: {} {} {}",
                            journey,
                            stop_ref,
                            time
                        );
                    }
                }*/
            }
        }
    }
    jorneys
}

pub fn find_stop_by_id(id: u32, stops: &[Stop]) -> Option<&Stop> {
    stops.iter().find(|stop| stop.id == id)
}

pub fn find_stop_by_kkv_id<'a>(id: &str, stops: &'a [Stop]) -> Option<&'a Stop> {
    //stops.iter().find(|stop| id.starts_with(&stop.kvv_stop.id))
    let id = format!("{}:", id);
    stops
        .iter()
        .filter(|stop| id.starts_with(&format!("{}:", stop.kvv_stop.id)))
        .max_by_key(|stop| stop.kvv_stop.id.len())
}

pub fn points_on_route(start_stop_id: &str, end_stop_id: &str, stops: &[Stop]) -> Vec<Point> {
    let Some(start_stop) = find_stop_by_kkv_id(start_stop_id, stops) else {
        return Vec::new();
    };
    let Some(end_stop) = find_stop_by_kkv_id(end_stop_id, stops) else {
        return Vec::new();
    };

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

#[derive(Debug, Clone)]
struct TrainPos {
    stop_id: String,
    next_stop_id: String,
    progress: f32,
}

pub fn interpolate_segment(points: &[Point], progress: f32) -> Option<Point> {
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
            return Some(Point {
                x: start.x + progress * dx,
                y: start.y + progress * dy,
            });
        }
        current_length += segment_length;
    }
    points.last().map(|end| Point { x: end.x, y: end.y })
}

pub fn train_positions_per_route(
    departures_per_line: LineDepartures,
    time: chrono::NaiveDateTime,
    line_id: &str,
    stops: &[Stop],
) -> Vec<Train> {
    let mut trains = Vec::new();
    let departures = departures_per_line.get(line_id);
    let mut departures: Vec<_> = departures
        .map(|x| x.stops.iter().collect())
        .unwrap_or_default();
    departures.sort_by_key(|x| x.1);

    let line_name = departures_per_line
        .get(line_id)
        .map(|x| x.line_name.clone())
        .unwrap_or_default();
    let destination = departures_per_line
        .get(line_id)
        .map(|x| x.destination.clone())
        .unwrap_or_default();
    //dbg!(&departures);
    /*println!("departures for line {}", line_id);
    for departure in departures.iter() {
        if let Some(stop) = find_stop_by_kkv_id(departure.0, stops) {
            println!("stop: {} {}", stop.kvv_stop.name, departure.1.time());
        }
    }*/

    let pos_offset = departures
        .iter()
        .position(|x| x.1 > &time)
        .unwrap_or_default();
    let mut train_offsets = Vec::new();
    let slice = &departures[(pos_offset.max(1) - 1)..=pos_offset];
    if let [last, next] = slice {
        // TODO: handle panics
        let last_time = *last.1 - time;
        let next_time = *next.1 - time;
        let segment_duration = next_time - last_time - chrono::Duration::seconds(30);
        train_offsets.push(TrainPos {
            stop_id: last.0.clone(),
            next_stop_id: next.0.clone(),
            progress: 1.
                - (next_time.num_seconds() as f32 / segment_duration.num_seconds() as f32)
                    .clamp(0., 1.),
        });
    }
    for train_offset in train_offsets {
        let points = points_on_route(&train_offset.stop_id, &train_offset.next_stop_id, stops);
        if let Some(position) = interpolate_segment(&points, train_offset.progress) {
            trains.push(Train {
                id: 0,
                long: position.x,
                lat: position.y,
                line_id: line_id.to_owned(),
                line_name: line_name.to_owned(),
                direction: destination.to_owned(),
            });
        }
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
    fetch_departures(stops).await
}

pub async fn train_positions(
    departures_per_line: &LineDepartures,
    render_time: chrono::NaiveDateTime,
) -> Vec<Train> {
    let stops = KVV_STOPS.get().expect("KVV_STOPS not initialized");
    let mut trains = Vec::new();
    let mut journeys: Vec<_> = departures_per_line.keys().collect();
    journeys.sort();
    for line_id in journeys {
        let positions =
            train_positions_per_route(departures_per_line.clone(), render_time, line_id, stops);
        trains.extend(positions);
    }
    trains
}
