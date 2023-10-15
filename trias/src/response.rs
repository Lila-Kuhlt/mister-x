use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TriasResponse {
    #[serde(rename = "ServiceDelivery")]
    pub service_delivery: ServiceDelivery,
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
pub struct LocationInformationResponse {
    #[serde(rename = "LocationResult")]
    pub location_result: Vec<LocationResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationResult {
    #[serde(rename = "Location")]
    pub location: Location,

    #[serde(rename = "Complete")]
    pub complete: String,

    #[serde(rename = "Probability")]
    pub probability: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    #[serde(rename = "StopPoint")]
    pub stop_point: StopPoint,

    #[serde(rename = "LocationName")]
    pub location_name: LocationName,

    #[serde(rename = "GeoPosition")]
    pub geo_position: Position,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    #[serde(rename = "Longitude")]
    pub longitude: String,

    #[serde(rename = "Latitude")]
    pub latitude: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationName {
    #[serde(rename = "Text")]
    pub text: String,

    #[serde(rename = "Language")]
    pub language: LanguageEnum,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StopPoint {
    #[serde(rename = "StopPointRef")]
    pub stop_point_ref: String,

    #[serde(rename = "StopPointName")]
    pub stop_point_name: LocationName,

    #[serde(rename = "LocalityRef")]
    pub locality_ref: String,

    #[serde(rename = "WheelchairAccessible")]
    pub wheelchair_accessible: String,

    #[serde(rename = "Lighting")]
    pub lighting: String,

    #[serde(rename = "Covered")]
    pub covered: String,
}

/*
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StopEventResponse {
    StopEventResult(Vec<StopEventResult>),
    ErrorMessage(ErrorMessage),
}*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StopEventResponse {
    #[serde(rename = "StopEventResponseContext")]
    pub stop_event_response_context: StopEventResponseContext,

    #[serde(rename = "StopEventResult")]
    pub stop_event_result: Vec<StopEventResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorMessage {
    #[serde(rename = "Code")]
    code: String,

    #[serde(rename = "Text")]
    text: LocationName,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StopEventResponseContext {
    #[serde(rename = "Situations")]
    situations: Situations,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StopEventResult {
    #[serde(rename = "ResultId")]
    pub result_id: String,

    #[serde(rename = "StopEvent")]
    pub stop_event: StopEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StopEvent {
    #[serde(rename = "PreviousCall")]
    pub previous_call: Option<Vec<Call>>,

    #[serde(rename = "ThisCall")]
    pub this_call: Call,

    #[serde(rename = "OnwardCall")]
    pub onward_call: Option<Vec<Call>>,

    #[serde(rename = "Service")]
    pub service: StopEventService,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Call {
    #[serde(rename = "CallAtStop")]
    pub call_at_stop: CallAtStop,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CallAtStop {
    #[serde(rename = "StopPointRef")]
    pub stop_point_ref: String,

    #[serde(rename = "StopPointName")]
    pub stop_point_name: LocationName,

    #[serde(rename = "PlannedBay")]
    pub planned_bay: Option<LocationName>,

    #[serde(rename = "ServiceArrival")]
    pub service_arrival: Option<Service>,

    #[serde(rename = "ServiceDeparture")]
    pub service_departure: Option<Service>,

    #[serde(rename = "StopSeqNumber")]
    pub stop_seq_number: String,

    #[serde(rename = "DemandStop")]
    pub demand_stop: String,

    #[serde(rename = "UnplannedStop")]
    pub unplanned_stop: String,

    #[serde(rename = "NotServicedStop")]
    pub not_serviced_stop: String,

    #[serde(rename = "NoBoardingAtStop")]
    pub no_boarding_at_stop: String,

    #[serde(rename = "NoAlightingAtStop")]
    pub no_alighting_at_stop: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Service {
    #[serde(rename = "TimetabledTime")]
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
    pub unplanned: String,
    pub cancelled: String,
    pub deviation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Attribute {
    pub text: LocationName,
    pub code: String,
    pub mandatory: String,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceSection {
    #[serde(rename = "LineRef")]
    pub line_ref: String,

    #[serde(rename = "DirectionRef")]
    pub direction_ref: String,

    #[serde(rename = "Mode")]
    pub mode: Mode,

    #[serde(rename = "PublishedLineName")]
    pub published_line_name: LocationName,

    #[serde(rename = "OperatorRef")]
    pub operator_ref: String,

    #[serde(rename = "RouteDescription")]
    pub route_description: Option<LocationName>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mode {
    #[serde(rename = "PtMode")]
    pt_mode: String,

    #[serde(rename = "TramSubmode")]
    tram_submode: Option<String>,

    #[serde(rename = "Name")]
    name: LocationName,

    #[serde(rename = "RailSubmode")]
    rail_submode: Option<String>,

    #[serde(rename = "BusSubmode")]
    bus_submode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TripResponse {
    #[serde(rename = "TripResponseContext")]
    trip_response_context: TripResponseContext,

    #[serde(rename = "TripResult")]
    trip_result: Vec<TripResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TripResponseContext {
    #[serde(rename = "Situations")]
    situations: Situations,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Situations {
    #[serde(rename = "PtSituation")]
    pt_situation: Option<Vec<PtSituation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PtSituation {
    #[serde(rename = "CreationTime")]
    creation_time: ProducerRef,

    #[serde(rename = "ParticipantRef")]
    participant_ref: ProducerRef,

    #[serde(rename = "SituationNumber")]
    situation_number: ProducerRef,

    #[serde(rename = "Version")]
    version: ProducerRef,

    #[serde(rename = "Source")]
    source: Source,

    #[serde(rename = "Progress")]
    progress: ProducerRef,

    #[serde(rename = "ValidityPeriod")]
    validity_period: ValidityPeriod,

    #[serde(rename = "UnknownReason")]
    unknown_reason: ProducerRef,

    #[serde(rename = "Priority")]
    priority: ProducerRef,

    #[serde(rename = "Audience")]
    audience: ProducerRef,

    #[serde(rename = "ScopeType")]
    scope_type: ProducerRef,

    #[serde(rename = "Planned")]
    planned: ProducerRef,

    #[serde(rename = "Language")]
    language: LanguageClass,

    #[serde(rename = "Summary")]
    summary: Description,

    #[serde(rename = "Description")]
    description: Description,

    #[serde(rename = "Detail")]
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
pub struct Source {
    #[serde(rename = "SourceType")]
    source_type: String,

    #[serde(rename = "_xmlns")]
    xmlns: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidityPeriod {
    #[serde(rename = "StartTime")]
    start_time: String,

    #[serde(rename = "EndTime")]
    end_time: String,

    #[serde(rename = "_xmlns")]
    xmlns: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TripResult {
    #[serde(rename = "ResultId")]
    result_id: String,

    #[serde(rename = "Trip")]
    trip: Trip,

    #[serde(rename = "TripFares")]
    trip_fares: TripFares,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trip {
    #[serde(rename = "TripId")]
    trip_id: String,

    #[serde(rename = "Duration")]
    duration: String,

    #[serde(rename = "StartTime")]
    start_time: String,

    #[serde(rename = "EndTime")]
    end_time: String,

    #[serde(rename = "Interchanges")]
    interchanges: String,

    #[serde(rename = "Distance")]
    distance: String,

    #[serde(rename = "TripLeg")]
    trip_leg: TripLeg,

    #[serde(rename = "OperatingDays")]
    operating_days: OperatingDays,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperatingDays {
    #[serde(rename = "From")]
    from: String,

    #[serde(rename = "To")]
    to: String,

    #[serde(rename = "Pattern")]
    pattern: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TripLeg {
    #[serde(rename = "LegId")]
    leg_id: String,

    #[serde(rename = "TimedLeg")]
    timed_leg: TimedLeg,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimedLeg {
    #[serde(rename = "LegBoard")]
    leg_board: Leg,

    #[serde(rename = "LegIntermediates")]
    leg_intermediates: Vec<Leg>,

    #[serde(rename = "LegAlight")]
    leg_alight: Leg,

    #[serde(rename = "Service")]
    service: TimedLegService,

    #[serde(rename = "LegTrack")]
    leg_track: LegTrack,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Leg {
    #[serde(rename = "StopPointRef")]
    stop_point_ref: String,

    #[serde(rename = "StopPointName")]
    stop_point_name: LocationName,

    #[serde(rename = "PlannedBay")]
    planned_bay: LocationName,

    #[serde(rename = "ServiceArrival")]
    service_arrival: Option<Service>,

    #[serde(rename = "StopSeqNumber")]
    stop_seq_number: String,

    #[serde(rename = "ServiceDeparture")]
    service_departure: Option<Service>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegTrack {
    #[serde(rename = "TrackSection")]
    track_section: TrackSection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackSection {
    #[serde(rename = "TrackStart")]
    track_start: Track,

    #[serde(rename = "TrackEnd")]
    track_end: Track,

    #[serde(rename = "Projection")]
    projection: Projection,

    #[serde(rename = "Duration")]
    duration: String,

    #[serde(rename = "Length")]
    length: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Projection {
    #[serde(rename = "Position")]
    position: Vec<Position>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Track {
    #[serde(rename = "StopPointRef")]
    stop_point_ref: String,

    #[serde(rename = "LocationName")]
    location_name: LocationName,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimedLegService {
    #[serde(rename = "OperatingDayRef")]
    operating_day_ref: String,

    #[serde(rename = "JourneyRef")]
    journey_ref: String,

    #[serde(rename = "LineRef")]
    line_ref: String,

    #[serde(rename = "DirectionRef")]
    direction_ref: String,

    #[serde(rename = "Mode")]
    mode: Mode,

    #[serde(rename = "PublishedLineName")]
    published_line_name: LocationName,

    #[serde(rename = "OperatorRef")]
    operator_ref: String,

    #[serde(rename = "RouteDescription")]
    route_description: LocationName,

    #[serde(rename = "OriginText")]
    origin_text: LocationName,

    #[serde(rename = "DestinationText")]
    destination_text: LocationName,

    #[serde(rename = "SituationFullRef")]
    situation_full_ref: Option<Vec<SituationFullRef>>,

    #[serde(rename = "Attribute")]
    attribute: Option<Attribute>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SituationFullRef {
    #[serde(rename = "ParticipantRef")]
    participant_ref: ProducerRef,

    #[serde(rename = "SituationNumber")]
    situation_number: ProducerRef,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TripFares {
    #[serde(rename = "FromTripLegIdRef")]
    from_trip_leg_id_ref: String,

    #[serde(rename = "ToTripLegIdRef")]
    to_trip_leg_id_ref: String,

    #[serde(rename = "PassedZones")]
    passed_zones: PassedZones,

    #[serde(rename = "Ticket")]
    ticket: Vec<Ticket>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PassedZones {
    #[serde(rename = "FaresAuthorityRef")]
    fares_authority_ref: FaresAuthorityRef,

    #[serde(rename = "FaresAuthorityText")]
    fares_authority_text: FaresAuthorityText,

    #[serde(rename = "FareZone")]
    fare_zone: FareZone,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FareZone {
    #[serde(rename = "FareZoneRef")]
    fare_zone_ref: String,

    #[serde(rename = "FareZoneText")]
    fare_zone_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    #[serde(rename = "TicketId")]
    ticket_id: String,

    #[serde(rename = "TicketName")]
    ticket_name: String,

    #[serde(rename = "FaresAuthorityRef")]
    fares_authority_ref: FaresAuthorityRef,

    #[serde(rename = "FaresAuthorityText")]
    fares_authority_text: FaresAuthorityText,

    #[serde(rename = "Price")]
    price: String,

    #[serde(rename = "NetPrice")]
    net_price: String,

    #[serde(rename = "Currency")]
    currency: Currency,

    #[serde(rename = "VatRate")]
    vat_rate: VatRate,

    #[serde(rename = "TravelClass")]
    travel_class: TravelClass,

    #[serde(rename = "ValidFor")]
    valid_for: Option<ValidFor>,

    #[serde(rename = "ValidityDuration")]
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
    #[serde(rename = "Adult")]
    Adult,

    #[serde(rename = "Child")]
    Child,

    #[serde(rename = "Senior")]
    Senior,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ValidityDuration {
    #[serde(rename = "PT14H54M")]
    Pt14H54M,

    #[serde(rename = "PT15H3M")]
    Pt15H3M,

    #[serde(rename = "PT1H30M")]
    Pt1H30M,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VatRate {
    #[serde(rename = "half")]
    Half,
}

#[derive(Serialize, Deserialize)]
pub struct DestinationText {
    #[serde(rename = "Text")]
    text: String,

    #[serde(rename = "Language")]
    language: LanguageEnum,
}

#[derive(Serialize, Deserialize)]
pub struct ThisCall {
    #[serde(rename = "CallAtStop")]
    call_at_stop: CallAtStop,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceDeparture {
    #[serde(rename = "TimetabledTime")]
    timetabled_time: String,

    #[serde(rename = "EstimatedTime")]
    estimated_time: String,
}

#[derive(Serialize, Deserialize)]
pub enum DirectionRef {
    #[serde(rename = "inward")]
    Inward,

    #[serde(rename = "outward")]
    Outward,
}

#[derive(Serialize, Deserialize)]
pub enum PtMode {
    #[serde(rename = "rail")]
    Rail,

    #[serde(rename = "tram")]
    Tram,
}

#[derive(Serialize, Deserialize)]
pub enum OperatorRef {
    #[serde(rename = "kvv:01")]
    Kvv01,

    #[serde(rename = "kvv:02")]
    Kvv02,
}
