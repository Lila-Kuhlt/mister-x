// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Welcome;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Welcome = serde_json::from_str(&json).unwrap();
// }

use chrono::NaiveDate;
use chrono::{DateTime, Duration, Local, NaiveDateTime, NaiveTime, TimeZone};
use serde::{Deserialize, Serialize};

pub async fn fetch_departures(stop: &str) -> Result<kvvliveapi::Departures, reqwest::Error> {
    let response = fetch_stop_events(stop).await?;
    let mut departures = Vec::new();
    let name = response.departure_list.as_ref().unwrap()[0]
        .stop_name
        .clone();
    let request_time = chrono_tz::Europe::Berlin
        .from_local_datetime(&Local::now().naive_local())
        .unwrap();
    for departure in response.departure_list.unwrap() {
        dbg!(&departure);
        let line_name = departure.serving_line.symbol;
        let destination = departure.serving_line.direction;
        let time = departure.date_time;
        //dbg!(departure.countdown);
        let date = NaiveDate::from_ymd_opt(
            time.year.parse().unwrap(),
            time.month.parse().unwrap(),
            time.day.parse().unwrap(),
        )
        .unwrap();
        let time =
            NaiveTime::from_hms_opt(time.hour.parse().unwrap(), time.minute.parse().unwrap(), 0)
                .unwrap();
        let datetime = NaiveDateTime::new(date, time);
        let datetime = chrono_tz::Europe::Berlin
            .from_local_datetime(&datetime)
            .unwrap();
        //dbg!((datetime - request_time).num_seconds());
        let datetime = request_time + Duration::seconds(departure.countdown.parse().unwrap());

        let departure = kvvliveapi::Departure {
            route: line_name,
            direction: destination.clone(),
            destination,
            time: datetime,
            lowfloor: false,
            realtime: false,
            traction: 0,
        };
        departures.push(departure);
    }
    let departures = kvvliveapi::Departures {
        timestamp: request_time,
        stop_name: name,
        departures,
    };
    Ok(departures)
}

pub async fn fetch_stop_id(name: &str) -> Result<String, reqwest::Error> {
    let name = urlencoding::encode(name);
    let request_string = format!("https://www.kvv.de/tunnelEfaDirect.php?action=XSLT_STOPFINDER_REQUEST&coordOutputFormat=WGS84[dd.ddddd]&name_sf=Karlsruhe, {}&language=de&outputFormat=JSON&type_sf=stop", name);
    let response = reqwest::get(&request_string).await?;
    let response = response.text().await?;
    println!("{}", &response);
    let response: StopFinderResponse = serde_json::from_str(&response).unwrap();
    dbg!(&response);
    let id = response.stop_finder.points.first().point_ref.id.clone();
    Ok(id)
}

pub async fn fetch_stop_by_id(id: &str) -> Result<kvvliveapi::Stop, reqwest::Error> {
    let response = fetch_stop_events(id).await?;
    let departure = &response.departure_list.unwrap()[0];
    let pos = (departure.x.parse().unwrap(), departure.y.parse().unwrap());
    let stop = kvvliveapi::Stop {
        id: id.to_owned(),
        name: departure.stop_name.clone(),
        lon: pos.0,
        lat: pos.1,
    };
    Ok(stop)
}

async fn fetch_stop_events(id: &str) -> Result<Response, reqwest::Error> {
    let id = urlencoding::encode(id);
    let request_string = format!("https://projekte.kvv-efa.de/sl3-alone/XSLT_DM_REQUEST?outputFormat=JSON&coordOutputFormat=WGS84[dd.ddddd]&depType=stopEvents&locationServerActive=1&mode=direct&name_dm={}&type_dm=stop&useOnlyStops=1&useRealtime=1", id);
    let response = reqwest::get(&request_string).await?;
    let response = response.text().await?;
    let response: Response = serde_json::from_str(&response)
        .expect(&format!("failed to parse response, got {}", response));
    Ok(response)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    parameters: Vec<Parameter>,
    dm: Arr,
    arr: Arr,
    date_time: WelcomeDateTime,
    date_range: Vec<DateRange>,
    option: KOption,
    serving_lines: ServingLines,
    departure_list: Option<Vec<DepartureList>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arr {
    input: Input,
    points: Option<PointsUnion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    usage: String,
    #[serde(rename = "type")]
    point_type: Option<String>,
    name: String,
    stateless: String,
    #[serde(rename = "ref")]
    point_ref: Ref,
    infos: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ref {
    id: String,
    gid: String,
    omc: String,
    #[serde(rename = "placeID")]
    place_id: String,
    place: String,
    coords: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    day: String,
    month: String,
    year: String,
    weekday: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeDateTime {
    deparr: String,
    ttp_from: String,
    ttp_to: String,
    year: String,
    month: String,
    day: String,
    hour: String,
    minute: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartureList {
    #[serde(rename = "stopID")]
    stop_id: String,
    x: String,
    y: String,
    map_name: String,
    area: String,
    platform: String,
    platform_name: String,
    stop_name: String,
    #[serde(rename = "nameWO")]
    name_wo: String,
    point_type: Option<PointType>,
    countdown: String,
    date_time: DepartureListDateTime,
    serving_line: ServingLine,
    operator: Option<Operator>,
    stop_infos: Option<serde_json::Value>,
    line_infos: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LineInfosUnion {
    LineInfoArray(Vec<LineInfo>),
    LineInfosClass(LineInfosClass),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineInfosClass {
    line_info: LineInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartureListDateTime {
    year: String,
    month: String,
    day: String,
    weekday: String,
    hour: String,
    minute: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineInfo {
    info_link_text: String,
    #[serde(rename = "infoLinkURL")]
    info_link_url: String,
    info_text: InfoText,
    param_list: Vec<ParamList>,
    additional_links: Vec<AdditionalLink>,
    attachments: Option<Vec<Attachment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalLink {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "linkURL")]
    link_url: String,
    link_text: String,
    link_text_short: String,
    link_target: LinkTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkTarget {
    #[serde(rename = "_blank")]
    Blank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    #[serde(rename = "ID")]
    id: String,
    path: String,
    file_name: String,
    virt_path: String,
    link_text: String,
    link_target: LinkTarget,
    size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoText {
    content: String,
    subtitle: String,
    subject: String,
    additional_text: String,
    html_text: String,
    wml_text: String,
    sms_text: String,
    speech_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamList {
    #[serde(rename = "type")]
    param_list_type: Type,
    name: String,
    value: String,
    edit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    #[serde(rename = "addInfoParam")]
    AddInfoParam,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator {
    code: String,
    name: String,
    public_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PointType {
    Gleis,
    Bstg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServingLine {
    key: String,
    code: String,
    number: String,
    symbol: String,
    mot_type: String,
    mt_subcode: String,
    realtime: String,
    direction: String,
    direction_from: String,
    train_name: Option<String>,
    train_num: Option<String>,
    name: String,
    li_erg_ri_proj: LiErgRiProj,
    #[serde(rename = "destID")]
    dest_id: String,
    stateless: String,
    hints: Option<Vec<Hint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hint {
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiErgRiProj {
    line: String,
    project: String,
    direction: Dir,
    supplement: String,
    network: Network,
    gid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Dir {
    H,
    R,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Network {
    Kvv,
    Vrs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KOption {
    pt_option: PtOption,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtOption {
    active: String,
    max_changes: String,
    max_time: String,
    max_wait: String,
    route_type: String,
    change_speed: String,
    line_restriction: String,
    use_prox_foot_search: String,
    use_prox_foot_search_origin: String,
    use_prox_foot_search_destination: String,
    bike: String,
    plane: String,
    no_crowded: String,
    no_solid_stairs: String,
    no_escalators: String,
    no_elevators: String,
    low_platform_vhcl: String,
    wheelchair: String,
    need_elevated_plt: String,
    assistance: String,
    #[serde(rename = "SOSAvail")]
    sos_avail: String,
    no_lonely_transfer: String,
    illum_transfer: String,
    overground_transfer: String,
    no_insecure_places: String,
    private_transport: String,
    excluded_means: Vec<ExcludedMean>,
    active_imp: String,
    active_com: String,
    active_sec: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExcludedMean {
    means: String,
    value: String,
    selected: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    name: String,
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServingLines {
    lines: Option<Vec<Line>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    mode: Mode,
    index: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mode {
    name: String,
    number: String,
    product: String,
    product_id: String,
    #[serde(rename = "type")]
    mode_type: String,
    code: String,
    destination: String,
    #[serde(rename = "destID")]
    dest_id: String,
    desc: String,
    timetable_period: String,
    diva: Diva,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diva {
    branch: String,
    line: String,
    supplement: String,
    dir: Dir,
    project: String,
    network: Network,
    stateless: String,
    trip_code: String,
    operator: String,
    op_code: String,
    v_f: String,
    v_to: String,
    attrs: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopFinderResponse {
    parameters: Vec<Parameter>,
    stop_finder: StopFinder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopFinder {
    input: Input,
    points: PointsUnion,
    message: Option<Vec<Parameter>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointsUnion {
    PointElementArray(Vec<Point>),
    PointsClass(PointsClass),
}

impl PointsUnion {
    fn first(&self) -> &Point {
        match self {
            PointsUnion::PointElementArray(vec) => &vec[0],
            PointsUnion::PointsClass(point) => &point.point,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsClass {
    point: Point,
}
