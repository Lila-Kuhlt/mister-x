use chrono::{DateTime, Utc};
use futures_util::future::join_all;
use lazy_static::lazy_static;
use serde::Serialize;

use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;

// mod api;

use crate::point::{interpolate_segment, Point};
use crate::ws_message::Train;

/// The wait time to use when the arrival or departure time is missing.
const DEFAULT_WAIT_TIME: Duration = Duration::from_secs(30);

/// Information about a tram station.
#[derive(Debug, Serialize, specta::Type, PartialEq)]
pub struct Stop {
    /// human readable stop name
    pub name: String,
    /// internal stop id
    pub id: String,
    /// position latitude
    pub lat: f64,
    /// position longitude
    pub lon: f64,
}

lazy_static! {
    /// The included stops and their IDs.
    static ref STOPS: HashMap<&'static str, &'static str> = HashMap::from([
        ("de:08212:64", "Arbeitsagentur"),
        ("de:08212:74", "Augartenstraße"),
        ("de:08212:5003", "Barbarossaplatz"),
        ("de:08212:3", "Durlacher Tor/KIT-Campus Süd"),
        ("de:08212:1001", "Durlacher Tor/KIT-Campus Süd (U)"),
        ("de:08212:91", "Ebertstraße"),
        ("de:08212:71", "Ettlinger Tor/Staatstheater"),
        ("de:08212:1012", "Ettlinger Tor/Staatstheater (U)"),
        ("de:08212:61", "Karlstor/Bundesgerichtshof"),
        ("de:08212:63", "Kolpingplatz"),
        ("de:08212:1013", "Kongresszentrum (U)"),
        ("de:08212:80", "Kronenplatz"),
        ("de:08212:1002", "Kronenplatz (U)"),
        ("de:08212:1003", "Marktplatz (Kaiserstraße U)"),
        ("de:08212:1011", "Marktplatz (Pyramide U)"),
        ("de:08212:62", "Mathystraße"),
        ("de:08212:39", "Mühlburger Tor"),
        ("de:08212:508", "Otto-Sachs-Straße"),
        ("de:08212:98", "Poststraße"),
        ("de:08212:85", "Rüppurrer Tor"),
        ("de:08212:40", "Schillerstraße"),
        ("de:08212:602", "Sophienstraße"),
        ("de:08212:5508", "St. Vincentius Krankenhaus"),
        ("de:08212:5504", "Südendschule"),
        ("de:08212:84", "Tivoli"),
        ("de:08212:603", "Weinbrennerplatz"),
        ("de:08212:6218", "Welfenstraße"),
        ("de:08212:83", "Werderstraße"),
        ("de:08212:65", "ZKM"),
        ("de:08212:507", "Lessingstraße"),
        ("de:08212:37", "Europaplatz/Postgalerie"),
        ("de:08212:1004", "Europaplatz/Postgalerie (U)"),
        ("de:08212:6", "Gottesauer Platz/BGV"),
        ("de:08212:89", "Hauptbahnhof (Vorplatz)"),
        ("de:08212:7003", "Kunstakademie/Hochschule"),
    ]);

    static ref CURVES: HashMap<(&'static str, &'static str), Vec<Point>> = {
        const CURVES_STR: &str = include_str!("../data/route_curves.csv");
        CURVES_STR
            .lines()
            .map(parse_curve)
            .collect::<Option<HashMap<_, _>>>()
            .unwrap_or_else(|| {
                tracing::error!("Error parsing curves");
                HashMap::new()
            })
    };
}

fn parse_curve(line: &str) -> Option<((&str, &str), Vec<Point>)> {
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
    Some(((start, end), points))
}

fn intermediate_points(start_id: &str, end_id: &str) -> Vec<Point> {
    let start = STOPS[start_id];
    let end = STOPS[end_id];

    if let Some(points) = CURVES.get(&(start, end)) {
        points.clone()
    } else if let Some(points) = CURVES.get(&(end, start)) {
        let mut points = points.clone();
        points.reverse();
        points
    } else {
        Vec::new()
    }
}

static API_ENDPOINT: OnceLock<String> = OnceLock::new();
static ACCESS_TOKEN: OnceLock<String> = OnceLock::new();

async fn kvv_stops() -> Vec<Stop> {
    let access_token = ACCESS_TOKEN.get().unwrap();
    let api_endpoint = API_ENDPOINT.get().unwrap();
    join_all(STOPS.keys().map(|&stop_id| async move {
        let stops = trias::search_stops(stop_id.to_owned(), access_token.clone(), api_endpoint, 1).await.unwrap();

        let first_stop = stops.into_iter().next().unwrap();
        let stop_point = first_stop.stop_point;
        let position = first_stop.geo_position;
        Stop {
            name: stop_point.stop_point_name.text,
            id: stop_point.stop_point_ref,
            lat: position.latitude,
            lon: position.longitude,
        }
    })).await
}

#[derive(Debug, Clone, PartialEq)]
pub struct Times {
    arrival: DateTime<Utc>,
    departure: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Journey {
    stops: Vec<(StopRef, Times)>,
    line_name: String,
    destination: String,
}

impl Journey {
    fn new(line_name: String, destination: String) -> Self {
        Self {
            stops: Vec::new(),
            line_name,
            destination,
        }
    }
}

type JourneyRef = String;
type StopRef = String;
pub type LineDepartures = HashMap<JourneyRef, Journey>;

pub fn get_times(call: &trias::response::CallAtStop) -> Option<Times> {
    let arrival = call
        .service_arrival
        .as_ref()
        .map(|service| service.estimated_time.unwrap_or(service.timetabled_time));
    let departure = call
        .service_departure
        .as_ref()
        .map(|service| service.estimated_time.unwrap_or(service.timetabled_time));
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

    let stop_results = join_all(stops.iter().map(|stop| {
        let name = stop.id.clone();
        let access_token = access_token.clone();
        async move {
            trias::stop_events(name, access_token, 10, api_endpoint)
                .await
                .map_err(|err| err.to_string())
        }
    })).await;

    let mut journeys = HashMap::new();

    for stop_event in stop_results
        .into_iter()
        .flat_map(|res| match res {
            Ok(x) => x.stop_event_result,
            Err(err) => {
                tracing::error!("{}", err);
                Vec::new()
            }
        })
        .map(|stop_event_result| stop_event_result.stop_event)
    {
        let service = stop_event.service;
        if service.cancelled {
            continue;
        }
        let journey_ref = service.journey_ref;
        let line_name = service.service_section.published_line_name.text;
        let destination = service.destination_text.text;
        if journeys.contains_key(&journey_ref) {
            continue;
        }
        let mut journey = Journey::new(line_name, destination);
        let previous_calls = stop_event.previous_call.into_iter();
        let this_call = stop_event.this_call;
        let next_calls = stop_event.onward_call.into_iter();
        let calls = previous_calls
            .chain(std::iter::once(this_call))
            .chain(next_calls)
            .map(|call| call.call_at_stop);

        for call in calls {
            let stop_ref = &call.stop_point_ref;
            let Some(stop) = find_stop_by_kvv_id(stop_ref, stops) else {
                continue;
            };
            let Some(times) = get_times(&call) else {
                continue;
            };
            journey.stops.push((stop.id.clone(), times));
        }
        journeys.insert(journey_ref, journey);
    }
    journeys
}

pub fn find_stop_by_kvv_id<'a>(id: &str, stops: &'a [Stop]) -> Option<&'a Stop> {
    // stop ids can have extra information at the end, e.g. "de:08212:3:01" which is not present in
    // the base id "de:08212:3". We want to match the base id.
    let id = format!("{}:", id);
    stops
        .iter()
        .find(|stop| id.starts_with(&format!("{}:", stop.id)))
}

pub fn points_on_route(start_stop_id: &str, end_stop_id: &str, stops: &[Stop]) -> Vec<Point> {
    let Some(start_stop) = find_stop_by_kvv_id(start_stop_id, stops) else {
        return Vec::new();
    };
    let Some(end_stop) = find_stop_by_kvv_id(end_stop_id, stops) else {
        return Vec::new();
    };

    let start = Point {
        latitude: start_stop.lat as f32,
        longitude: start_stop.lon as f32,
    };
    let end = Point {
        latitude: end_stop.lat as f32,
        longitude: end_stop.lon as f32,
    };
    let mut points = vec![start];
    points.extend(intermediate_points(start_stop_id, end_stop_id));
    points.push(end);

    points
}

pub fn train_position_per_route(
    time: DateTime<Utc>,
    journey_ref: &str,
    departures: &Journey,
    stops: &[Stop],
) -> Option<Train> {
    if departures.stops.is_empty() {
        tracing::warn!("no departures for journey {}", journey_ref);
        return None;
    }

    let line_name = departures.line_name.clone();
    let destination = departures.destination.clone();

    let pos_offset = departures
        .stops
        .binary_search_by_key(&time, |(_, times)| times.departure)
        .unwrap_or_else(|i| i);
    if let [last, next] = &departures.stops[(pos_offset.max(1) - 1)..=pos_offset.min(departures.stops.len() - 1)] {
        let current_duration = time - last.1.departure;
        let segment_duration = next.1.arrival - last.1.departure;
        let stop_id = &last.0;
        let next_stop_id = &next.0;
        let progress = (current_duration.num_seconds() as f32 / segment_duration.num_seconds() as f32).clamp(0., 1.);
        let points = points_on_route(stop_id, next_stop_id, stops);
        if let Some(position) = interpolate_segment(&points, progress) {
            return Some(Train {
                id: 0,
                lat: position.latitude,
                long: position.longitude,
                line_id: journey_ref.to_owned(),
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
        .flat_map(|(journey_ref, departures)| {
            train_position_per_route(render_time, journey_ref, departures, stops)
        })
        .collect()
}
