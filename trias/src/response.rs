use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TriasResponse {
    pub service_delivery: Option<ServiceDelivery>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DeliveryPayload {
    LocationInformationResponse(LocationInformationResponse),
    StopEventResponse(Vec<StopEventResponse>),
    TripResponse(TripResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceDelivery {
    pub response_timestamp: ProducerRef,
    pub producer_ref: ProducerRef,
    pub status: ProducerRef,
    pub more_data: String,
    pub language: LanguageEnum,
    pub calc_time: Option<String>,
    pub delivery_payload: DeliveryPayload,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LocationInformationResponse {
    pub location_result: Vec<LocationResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LocationResult {
    pub location: Location,
    pub complete: String,
    pub probability: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    pub stop_point: StopPoint,
    pub location_name: LocationName,
    pub geo_position: Position,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Position {
    pub longitude: String,
    pub latitude: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LocationName {
    pub text: String,
    pub language: LanguageEnum,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StopPoint {
    pub stop_point_ref: String,
    pub stop_point_name: LocationName,
    pub locality_ref: String,
    pub wheelchair_accessible: String,
    pub lighting: String,
    pub covered: String,
}

/*
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StopEventResponse {
    StopEventResult(Vec<StopEventResult>),
    ErrorMessage(ErrorMessage),
}*/

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StopEventResponse {
    pub stop_event_response_context: StopEventResponseContext,
    pub stop_event_result: Option<Vec<StopEventResult>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorMessage {
    code: String,
    text: LocationName,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StopEventResponseContext {
    situations: Situations,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StopEventResult {
    pub result_id: String,
    pub stop_event: StopEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StopEvent {
    pub previous_call: Option<Vec<Call>>,
    pub this_call: Call,
    pub onward_call: Option<Vec<Call>>,
    pub service: StopEventService,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Call {
    pub call_at_stop: CallAtStop,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CallAtStop {
    pub stop_point_ref: String,
    pub stop_point_name: LocationName,
    pub planned_bay: Option<LocationName>,
    pub service_arrival: Option<Service>,
    pub service_departure: Option<Service>,
    pub stop_seq_number: String,
    pub demand_stop: bool,
    pub unplanned_stop: bool,
    pub not_serviced_stop: bool,
    pub no_boarding_at_stop: bool,
    pub no_alighting_at_stop: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Service {
    pub timetabled_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StopEventService {
    pub operating_day_ref: String,
    pub journey_ref: String,
    pub service_section: ServiceSection,
    pub attribute: Option<Attribute>,
    pub origin_stop_point_ref: String,
    pub origin_text: LocationName,
    pub destination_text: LocationName,
    pub unplanned: bool,
    pub cancelled: bool,
    pub deviation: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Attribute {
    pub text: LocationName,
    pub code: String,
    pub mandatory: bool,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceSection {
    pub line_ref: String,
    pub direction_ref: String,
    pub mode: Mode,
    pub published_line_name: LocationName,
    pub operator_ref: String,
    pub route_description: Option<LocationName>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Mode {
    pt_mode: String,
    tram_submode: Option<String>,
    name: LocationName,
    rail_submode: Option<String>,
    bus_submode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TripResponse {
    trip_response_context: TripResponseContext,
    trip_result: Vec<TripResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TripResponseContext {
    situations: Situations,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Situations {
    pt_situation: Option<Vec<PtSituation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PtSituation {
    creation_time: ProducerRef,
    participant_ref: ProducerRef,
    situation_number: ProducerRef,
    version: ProducerRef,
    source: Source,
    progress: ProducerRef,
    validity_period: ValidityPeriod,
    unknown_reason: ProducerRef,
    priority: ProducerRef,
    audience: ProducerRef,
    scope_type: ProducerRef,
    planned: ProducerRef,
    language: LanguageClass,
    summary: Description,
    description: Description,
    detail: Description,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProducerRef {
    #[serde(rename = "_xmlns")]
    xmlns: Option<String>,

    #[serde(rename = "__text")]
    text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Description {
    #[serde(rename = "_xmlns")]
    xmlns: Option<String>,

    #[serde(rename = "_overridden")]
    overridden: Option<String>,

    #[serde(rename = "__text")]
    text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LanguageClass {
    #[serde(rename = "_xmlns")]
    xmlns: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Source {
    source_type: String,

    #[serde(rename = "_xmlns")]
    xmlns: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ValidityPeriod {
    start_time: String,
    end_time: String,

    #[serde(rename = "_xmlns")]
    xmlns: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TripResult {
    result_id: String,
    trip: Trip,
    trip_fares: TripFares,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Trip {
    trip_id: String,
    duration: String,
    start_time: String,
    end_time: String,
    interchanges: String,
    distance: String,
    trip_leg: TripLeg,
    operating_days: OperatingDays,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct OperatingDays {
    from: String,
    to: String,
    pattern: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TripLeg {
    leg_id: String,
    timed_leg: TimedLeg,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TimedLeg {
    leg_board: Leg,
    leg_intermediates: Vec<Leg>,
    leg_alight: Leg,
    service: TimedLegService,
    leg_track: LegTrack,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Leg {
    stop_point_ref: String,
    stop_point_name: LocationName,
    planned_bay: LocationName,
    service_arrival: Option<Service>,
    stop_seq_number: String,
    service_departure: Option<Service>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LegTrack {
    track_section: TrackSection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TrackSection {
    track_start: Track,
    track_end: Track,
    projection: Projection,
    duration: String,
    length: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Projection {
    position: Vec<Position>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Track {
    stop_point_ref: String,
    location_name: LocationName,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TimedLegService {
    operating_day_ref: String,
    journey_ref: String,
    line_ref: String,
    direction_ref: String,
    mode: Mode,
    published_line_name: LocationName,
    operator_ref: String,
    route_description: LocationName,
    origin_text: LocationName,
    destination_text: LocationName,
    situation_full_ref: Option<Vec<SituationFullRef>>,
    attribute: Option<Attribute>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SituationFullRef {
    participant_ref: ProducerRef,
    situation_number: ProducerRef,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TripFares {
    from_trip_leg_id_ref: String,
    to_trip_leg_id_ref: String,
    passed_zones: PassedZones,
    ticket: Vec<Ticket>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PassedZones {
    fares_authority_ref: FaresAuthorityRef,
    fares_authority_text: FaresAuthorityText,
    fare_zone: FareZone,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FareZone {
    fare_zone_ref: String,
    fare_zone_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Ticket {
    ticket_id: String,
    ticket_name: String,
    fares_authority_ref: FaresAuthorityRef,
    fares_authority_text: FaresAuthorityText,
    price: String,
    net_price: String,
    currency: Currency,
    vat_rate: VatRate,
    travel_class: TravelClass,
    valid_for: Option<ValidFor>,
    validity_duration: Option<ValidityDuration>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LanguageEnum {
    #[serde(rename = "de")]
    De,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FaresAuthorityRef {
    #[serde(rename = "kvv")]
    Kvv,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FaresAuthorityText {
    #[serde(rename = "KVV")]
    Kvv,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Currency {
    #[serde(rename = "EUR")]
    Eur,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TravelClass {
    #[serde(rename = "second")]
    Second,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ValidFor {
    Adult,
    Child,
    Senior,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ValidityDuration {
    Pt14H54M,
    Pt15H3M,
    Pt1H30M,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VatRate {
    #[serde(rename = "half")]
    Half,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DestinationText {
    text: String,
    language: LanguageEnum,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThisCall {
    call_at_stop: CallAtStop,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceDeparture {
    timetabled_time: String,
    estimated_time: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DirectionRef {
    Inward,
    Outward,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PtMode {
    Rail,
    Tram,
}

#[derive(Serialize, Deserialize)]
pub enum OperatorRef {
    #[serde(rename = "kvv:01")]
    Kvv01,

    #[serde(rename = "kvv:02")]
    Kvv02,
}
