#[macro_export]
macro_rules! response {
    (pub struct $name: ident {$($field: ident: $t: ty $(as $alias: literal)? $(:$venc:vis encoded)? $(; default $vdef:vis)?),* $(,)?}) => {
        #[derive(Clone, Debug, Deserialize)]
        pub struct $name {
            $(
                #[serde(
                    $(rename = $alias,)?
                    $(deserialize_with = "deserialize_number_from_string"$venc,)?
                    $(default $vdef,)?
                )]
                pub $field: $t
            ),*
        }
    };
}

pub mod departure_monitor;
pub mod stop_finder;

pub use departure_monitor::*;
pub use stop_finder::*;

use std::collections::HashMap;
use serde::Deserialize;
use serde_aux::prelude::*;

use crate::{ApiVec, request::types::StationId};

response!(pub struct Parameter {
    name: String,
    value: String,
    typ: Option<String> as "type",
    edit: Option<String>
});

response!(pub struct ResponseData {
    input: HashMap<String, String>,
    points: ApiVec<Point>
});

response!(pub struct Input {
    input: String
});

response!(pub struct Point {
    usage: String,
    typ: String as "type",
    name: String,
    stateless: String,
    any_type: Option<String> as "anyType",
    sort: Option<String>,
    quality: Option<String>,
    best: Option<String>,
    object: Option<String>,
    main_loc: Option<String> as "mainLoc",
    modes: Option<String>,
//    reference: Station as "ref",
});

response!(pub struct Station {
    id: String,
    gid: String,
    omc: u32: encoded,
    place_id: String as "placeID",
    place: String,
    coords: Option<String>
});

response!(pub struct Date {
    day: String,
    month: String,
    year: String,
    weekday: String
});

response!(pub struct DateTime {
    deparr: Option<String>,
    ttp_from: Option<String> as "ttpFrom",
    ttp_to: Option<String> as "ttpTo",
    year: Option<String>,
    month: Option<String>,
    day: Option<String>,
    weekday: Option<String>,
    hour: Option<String>,
    minute: Option<String>, 
});

response!(pub struct ServingLines {
    train_info: String as "trainInfo",
    selected: usize: encoded,
    lines: Vec<ServingLineEntry>
});

response!(pub struct ServingLineEntry {
    mode: LineMode,
    index: String  
});

response!(pub struct LineMode {
    name: String,
    number: String,
    product: String,
    product_id: String as "productId",
    typ: String as "type",
    code: String,
    destination: String,
    destination_id: String as "destID",
    desc: String,
    timetable_period: String as "timetablePeriod",
    diva: Diva
});

response!(pub struct Diva {
    branch: String,
    line: String,
    supplement: String,
    dir: String,
    project: String,
    network: String,
    stateless: String,
    trip_code: String as "tripCode",
    operator: String,
    op_code: String as "opCode",
    v_from: String as "vF",
    v_to: String as "vTo",
    attrs: Vec<Parameter>
});

response!(pub struct Departure {
    stop_id: StationId as "stopID": encoded,
    x: f32: encoded,
    y: f32: encoded,
    map_name: String as "mapName",
    area: String,
    platform: String,
    platform_name: String as "platformName",
    stop_name: String as "stopName",
    name_wo: String as "nameWO",
    point_type: Option<String> as "pointType",
    countdown: String,
    realtime_status: Option<String> as "realtimeStatus",
    realtime_trip_status: Option<String> as "realtimeTripStatus",
    date_time: DateTime as "dateTime",
    real_date_time: Option<DateTime> as "realDateTime",
    serving_line: ServingLine as "servingLine",
    operator: Option<Operator>,
    stop_infos: Option<ApiVec<Info>> as "stopInfos",
    line_infos: Option<ApiVec<Info>> as "lineInfos",
    attrs: Vec<Parameter>; default,
});

response!(pub struct ServingLine {
    key: String,
    code: String,
    number: String,
    symbol: String,
    mot_type: String as "motType",
    mt_subcode: String as "mtSubcode",
    realtime: String,
    direction: String,
    direction_from: String as "directionFrom",
    train_name: Option<String> as "trainName",
    train_number: Option<String> as "trainNum",
    name: String,
    delay: Option<String>,
    dest_id: String as "destID",
    stateless: String
});

response!(pub struct Operator {
    code: String,
    name: String,
    public_code: String as "publicCode"
});

response!(pub struct Info {
    info_link_text: String as "infoLinkText",
    info_link_url: String as "infoLinkURL",
    info_text: InfoText as "infoText",
    param_list: Vec<Parameter> as "paramList",
    additional_links: Vec<AdditionalLink> as "additionalLinks"; default,
});

response!(pub struct InfoText {
    content: String,
    subtitle: String,
    subject: String,
    additional_text: String as "additionalText",
    html_text: String as "htmlText",
    wml_text: String as "wmlText",
    sms_text: String as "smsText",
    speech_text: String  as "speechText"
});

response!(pub struct AdditionalLink {
    id: String as "ID",
    link_url: String as "linkURL",
    link_text: String as "linkText",
    link_text_short: String as "linkTextShort",
    link_target: String as "linkTarget"
});

