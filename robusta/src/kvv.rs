use chrono::{DateTime, Utc};
use futures_util::future::join_all;
use serde::Serialize;

use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;

// mod api;

use crate::point::{interpolate_segment, Point};
use crate::ws_message::Train;

/// The wait time to use when the arrival or departure time is missing.
const DEFAULT_WAIT_TIME: Duration = Duration::from_secs(30);

#[derive(Debug, Serialize, specta::Type)]
pub struct Stop {
    id: u32,
    kvv_stop: KvvStop,
}

/// Information about a tram station
#[derive(Debug, Serialize, specta::Type, PartialEq)]
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

const STOPS: &[(&str, &str)] = &[
    ("Arbeitsagentur", "de:08212:64"),
    ("Augartenstraße", "de:08212:74"),
    ("Barbarossaplatz", "de:08212:5003"),
    ("Durlacher Tor/KIT-Campus Süd", "de:08212:3"),
    ("Durlacher Tor/KIT-Campus Süd (U)", "de:08212:1001"),
    ("Ebertstraße", "de:08212:91"),
    ("Ettlinger Tor/Staatstheater", "de:08212:71"),
    ("Ettlinger Tor/Staatstheater (U)", "de:08212:1012"),
    ("Karlstor/Bundesgerichtshof", "de:08212:61"),
    ("Kolpingplatz", "de:08212:63"),
    ("Kongresszentrum (U)", "de:08212:1013"),
    ("Kronenplatz", "de:08212:80"),
    ("Kronenplatz (U)", "de:08212:1002"),
    ("Marktplatz (Kaiserstraße U)", "de:08212:1003"),
    ("Marktplatz (Pyramide U)", "de:08212:1011"),
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
    ("Lessingstraße", "de:08212:507"),
    ("Europaplatz/Postgalerie", "de:08212:37"),
    ("Europaplatz/Postgalerie (U)", "de:08212:1004"),
    ("Gottesauer Platz/BGV", "de:08212:6"),
    ("Hauptbahnhof (Vorplatz)", "de:08212:89"),
    ("Kunstakademie/Hochschule", "de:08212:7003"),
];

static CURVES: OnceLock<Vec<(String, String, Vec<Point>)>> = OnceLock::new();

fn parse_curve(line: &str) -> Option<(String, String, Vec<Point>)> {
    let mut parts = line.split(';');
    let start = parts.next()?.trim();
    let end = parts.next()?.trim();
    let points = parts
        .map(|point| {
            let mut coords = point.split(',');
            let latitude = coords.next()?.trim().parse().ok()?;
            let longitude = coords.next()?.trim().parse().ok()?;
            Some(Point {
                latitude,
                longitude,
            })
        })
        .collect::<Option<Vec<_>>>()?;
    Some((start.to_owned(), end.to_owned(), points))
}

fn get_curves() -> &'static [(String, String, Vec<Point>)] {
    const CURVES_STR: &str = include_str!("../data/route_curves.csv");
    CURVES.get_or_init(|| {
        CURVES_STR
            .lines()
            .map(parse_curve)
            .collect::<Option<Vec<_>>>()
            .unwrap_or_else(|| {
                tracing::error!("Error parsing curves");
                Vec::new()
            })
    })
}

fn intermediate_points(start_id: &str, end_id: &str) -> Vec<Point> {
    let curves = get_curves();
    let start = STOPS.iter().find(|stop| start_id == stop.1).unwrap().0;
    let end = STOPS.iter().find(|stop| end_id == stop.1).unwrap().0;
    let mut points = Vec::new();

    if let Some(p) = curves.iter().find(|(s, e, _)| s == start && e == end) {
        points = p.2.clone();
    }
    if let Some(p) = curves.iter().find(|(s, e, _)| e == start && s == end) {
        points = p.2.clone();
        points.reverse();
    }

    points
}

static API_ENDPOINT: OnceLock<String> = OnceLock::new();
static ACCESS_TOKEN: OnceLock<String> = OnceLock::new();

async fn kvv_stops() -> Vec<Stop> {
    let access_token = ACCESS_TOKEN.get().unwrap();
    let api_endpoint = API_ENDPOINT.get().unwrap();
    join_all(STOPS
        .iter()
        .enumerate()
        .map(|(id, stop)| async move {
            let name = stop.1.to_string();
            let stops = trias::search_stops(name, access_token.clone(), api_endpoint, 1).await.unwrap();

            let first_stop = stops.into_iter().next().unwrap();
            let stop_point = first_stop.stop_point;
            let position = first_stop.geo_position;
            let kvv_stop = KvvStop {
                name: stop_point.stop_point_name.text,
                id: stop_point.stop_point_ref,
                lat: position.latitude.parse().unwrap(),
                lon: position.longitude.parse().unwrap(),
            };

            Stop {
                id: id as u32,
                kvv_stop,
            }
        })).await
}

#[derive(Debug, Clone)]
pub struct Times {
    arrival: DateTime<Utc>,
    departure: DateTime<Utc>,
}

#[derive(Debug, Default, Clone)]
pub struct Journey {
    stops: HashMap<StopRef, Times>,
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

pub fn parse_times(call: &trias::response::Call) -> Option<Times> {
    let arrival = call
        .call_at_stop
        .service_arrival
        .as_ref()
        .map(|service| service.estimated_time.as_ref().unwrap_or(&service.timetabled_time).parse().unwrap());
    let departure = call
        .call_at_stop
        .service_departure
        .as_ref()
        .map(|service| service.estimated_time.as_ref().unwrap_or(&service.timetabled_time).parse().unwrap());
    match (arrival, departure) {
        (Some(arrival), Some(departure)) => Some(Times { arrival, departure }),
        (Some(arrival), None) => Some(Times { arrival, departure: arrival + DEFAULT_WAIT_TIME }),
        (None, Some(departure)) => Some(Times { arrival: departure - DEFAULT_WAIT_TIME, departure }),
        (None, None) => {
            tracing::warn!("no departure or arrival time");
            None
        }
    }
}

pub async fn fetch_departures(stops: &[Stop]) -> LineDepartures {
    let access_token = ACCESS_TOKEN.get().unwrap();
    let api_endpoint = API_ENDPOINT.get().unwrap();

    let results = join_all(stops
        .iter()
        .map(|stop| {
            let name = stop.kvv_stop.id.clone();
            let access_token = access_token.clone();
            async move {
                trias::stop_events(name, access_token, 10, api_endpoint)
                    .await
                    .unwrap_or_default()
            }
        })
    ).await;

    let mut jorneys = HashMap::new();

    for stop_events in results
        .iter()
        .flatten()
        .flat_map(|x| x.stop_event_result.as_ref())
    {
        for stop in stop_events {
            let service = &stop.stop_event.service;
            if service.cancelled {
                continue;
            }
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
                let Some(times) = parse_times(call) else { continue; };
                let stop_ref = &call.call_at_stop.stop_point_ref;
                let Some(proper_stop_ref) = find_stop_by_kvv_id(stop_ref, stops) else {
                    continue;
                };
                let short_ref = &proper_stop_ref.kvv_stop.id;
                let current_times = entry.stops.get(short_ref);
                if let Some(current_times) = current_times {
                    if current_times.departure < times.departure {
                        continue;
                    }
                }
                entry.stops.insert(short_ref.clone(), times);
            }
            if entry.stops.len() < 2 {
                jorneys.remove(journey);
            }
        }
    }
    jorneys
}

pub fn find_stop_by_kvv_id<'a>(id: &str, stops: &'a [Stop]) -> Option<&'a Stop> {
    // stop ids can have extra information at the end, e.g. "de:08212:3:01" which is not present in
    // the base id "de:08212:3". We want to match the base id.
    let id = format!("{}:", id);
    stops
        .iter()
        .find(|stop| id.starts_with(&format!("{}:", stop.kvv_stop.id)))
}

pub fn points_on_route(start_stop_id: &str, end_stop_id: &str, stops: &[Stop]) -> Vec<Point> {
    let Some(start_stop) = find_stop_by_kvv_id(start_stop_id, stops) else {
        return Vec::new();
    };
    let Some(end_stop) = find_stop_by_kvv_id(end_stop_id, stops) else {
        return Vec::new();
    };

    let start = Point {
        latitude: start_stop.kvv_stop.lat as f32,
        longitude: start_stop.kvv_stop.lon as f32,
    };
    let end = Point {
        latitude: end_stop.kvv_stop.lat as f32,
        longitude: end_stop.kvv_stop.lon as f32,
    };
    let mut points = vec![start];
    points.extend(intermediate_points(start_stop_id, end_stop_id));
    points.push(end);

    points
}

pub fn train_position_per_route(
    time: DateTime<Utc>,
    line_id: &str,
    departures: &Journey,
    stops: &[Stop],
) -> Option<Train> {
    let mut line_stops: Vec<_> = departures.stops.iter().collect();
    line_stops.sort_by_key(|x| x.1.departure);

    if line_stops.is_empty() {
        tracing::warn!("no departures for line {}", line_id);
        return None;
    }

    let line_name = departures.line_name.clone();
    let destination = departures.destination.clone();

    let pos_offset = line_stops
        .iter()
        .position(|x| x.1.departure > time)
        .unwrap_or_default();
    let slice = &line_stops[(pos_offset.max(1) - 1)..=pos_offset];
    if let [last, next] = slice {
        let current_duration = time - last.1.departure;
        let segment_duration = next.1.arrival - last.1.departure;
        let stop_id = last.0;
        let next_stop_id = next.0;
        let progress = (current_duration.num_seconds() as f32 / segment_duration.num_seconds() as f32).clamp(0., 1.);
        let points = points_on_route(stop_id, next_stop_id, stops);
        if let Some(position) = interpolate_segment(&points, progress) {
            return Some(Train {
                id: 0,
                lat: position.latitude,
                long: position.longitude,
                line_id: line_id.to_owned(),
                line_name,
                direction: destination,
            });
        }
    }
    None
}

pub static KVV_STOPS: OnceLock<Vec<Stop>> = OnceLock::new();

pub async fn init() {
    let api_endpoint = dotenv::var("TRIAS_API_ENDPOINT").expect("TRIAS_API_ENDPOINT not set");
    API_ENDPOINT.set(api_endpoint).expect("failed to set API_ENDPOINT");
    let access_token = dotenv::var("TRIAS_ACCESS_TOKEN").expect("TRIAS_ACCESS_TOKEN not set");
    ACCESS_TOKEN.set(access_token).expect("failed to set ACCESS_TOKEN");
    let stops = kvv_stops().await;
    KVV_STOPS.set(stops).expect("failed to set KVV_STOPS");
}

pub async fn fetch_departures_for_region() -> LineDepartures {
    let stops = KVV_STOPS.get().expect("KVV_STOPS not initialized");
    fetch_departures(stops).await
}

pub fn train_positions(
    departures_per_line: &LineDepartures,
    render_time: DateTime<Utc>,
) -> Vec<Train> {
    let stops = KVV_STOPS.get().expect("KVV_STOPS not initialized");
    departures_per_line
        .iter()
        .flat_map(|(line_id, departures)| train_position_per_route(
            render_time,
            line_id,
            departures,
            stops,
        ))
        .collect()
}
