use std::collections::HashMap;

use anyhow::{anyhow, Context};
use serde::Deserialize;

pub fn read(f: &str) -> anyhow::Result<Problem> {
    let status: Status = read_json_file(f, "status.json")?;
    let train_info: TrainInfos = read_json_file(f, "TrainInfo.json")?;
    let line_movements: LineMovements = read_json_file(f, "LineMovements.json")?;
    let station_movements: StationMovements = read_json_file(f, "StationMovements.json")?;

    Ok(Problem {
        status,
        train_info,
        line_movements,
        station_movements,
    })
}

fn read_json_file<'a, T: serde::de::DeserializeOwned>(f: &str, x: &str) -> anyhow::Result<T> {
    let status_file = glob::glob(&format!("{}/*{}", f, x))?
        .nth(0)
        .ok_or_else(|| anyhow!("No {} file in the directory.", x))??;
    let status_str = std::fs::read_to_string(&status_file)?;
    let status = serde_json::from_str::<T>(&status_str)
        .with_context(|| format!("reading json file {}", status_file.display()))?;
    Ok(status)
}

#[derive(Deserialize, Debug)]
pub struct Problem {
    pub status: Status,
    pub train_info: TrainInfos,
    pub line_movements: LineMovements,
    pub station_movements: StationMovements,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Status {
    pub now: i64,
    pub trains: Vec<Train>,
    pub lined_routes: Vec<LinedRoute>,
    pub blocks: Vec<Block>,
    pub slowdowns: Vec<Slowdown>,
    pub dispatcher_solved_conflicts: Vec<()>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Train {
    pub id: String,
    pub train_positions: Vec<TrainPosition>,
    pub train_mode: String,
    pub train_hold_main: bool,
    pub current_length: i64,
    pub train_category: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CurrentPosition {
    Track,
    StationStoppingPoint,
    StationRoute,
    Predicted,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrainPosition {
    pub current_position: CurrentPosition,
    pub time_in: i64,
    pub station_position: Option<StationPosition>,
    pub track_circuit_position: Option<TrackCircuitPosition>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct StationPosition {
    pub station_id: String,
    pub stopping_point_id: Option<String>,
    pub route_id: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrackCircuitPosition {
    pub track_id: String,
    pub track_circuit_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LinedRoute {
    pub station_id: String,
    pub route_id: String,
    pub train_id: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockType {
    Rolling,
    Countdown,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Block {
    pub block_id: String,
    pub start_time: i64,
    pub duration: i64,
    pub block_type: BlockType,
    pub resources: Vec<BlockResource>,
    pub long_term: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BlockResource {
    pub station_resource: Option<StationBlockResource>,
    pub track_resource: Option<TrackBlockResource>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct StationBlockResource {
    pub station_id: String,
    pub route_id: Option<String>,
    pub stopping_point_id: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrackBlockResource {
    pub track_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Slowdown {
    pub slowdown_id: String,
    pub start_time: i64,
    pub duration: i64,
    pub speed: i64,
    pub resources: Option<Vec<BlockResource>>,
    pub description: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrainInfos {
    pub train_infos: std::collections::HashMap<String, TrainInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrainInfo {
    pub category: String,
    pub priority: i64,
    pub default_length: i64,
    pub speed: i64,
    pub line_point_headway: i64,
    pub followers: Option<Vec<String>>,
    pub crossings: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LineMovements {
    pub line_movements: std::collections::HashMap<String, TrainLineMovements>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrainLineMovements {
    pub track_movements: Option<HashMap<String, TrackMovement>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrackMovement {
    pub station_id: String,
    pub track_circuit_infos: Option<Vec<TrackCircuitInfo>>,
    pub reachable_track_ids: Option<Vec<String>>,
    pub track_runtime_infos: Vec<TrackRuntimeInfo>,
    pub min_cumulative_runtime: i64,
    pub min_cumulative_runtimes: Option<HashMap<String, i64>>,
    pub best_out_track_id: String,
    pub correct_path_id: Option<String>,
    pub min_reverse_switches: i64,
    pub min_reverse_switches_by_track: Option<HashMap<String, i64>>,
    pub preferred_out_track_id: Option<String>,
    pub min_non_preferred: i64,
    pub min_non_preferred_by_track: Option<HashMap<String, i64>>,
    pub distance_from_mandatory_non_preferred: Option<i64>,
    pub available_mask: i64,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrackRuntimeInfo {
    pub track_running_time: i64,
    pub line_headway: i64,
    pub track_circuit_running_times: Vec<TrackCircuitRunningTime>,
    pub reverse_track_running_time: Option<i64>,
    pub reverse_track_circuit_running_times: Option<Vec<TrackCircuitRunningTime>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrackCircuitRunningTime {
    pub track_circuit_id: String,
    pub running_time: i64,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrackCircuitInfo {
    pub track_circuit_id: String,
    pub dwell_time: Option<i64>,
    pub activities: Option<Vec<String>>,
    pub new_projected_length: Option<i64>,
    pub dwell_type: Option<DwellType>,
    #[serde(rename = "activityIds")]
    pub activity_ids: Option<Vec<usize>>,
    pub penalty: Option<i64>,
    pub end_of_graph: Option<bool>,
    pub earliest_departure_time: Option<i64>,
    pub relative_edt: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DwellType {
    Activity,
    Pseudo,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct StationMovements {
    pub train_movements: std::collections::HashMap<String, TrainStationMovements>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrainStationMovements {
    pub station_movements: Option<HashMap<String, TrainStationMovement>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrainStationMovement {
    pub station_nodes: HashMap<String, StationNode>,
    pub entry_track_ids: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct StationNode {
    pub id: String,
    pub node_type: NodeType,
    pub default_min_cumulative_runtime: i64,
    pub min_cumulative_runtime: Option<HashMap<String, i64>>,
    pub next_edges: Option<Vec<String>>,
    pub min_clearance: Option<i64>,
    pub available_mask: i64,
    pub correct_path: Option<bool>,
    pub dwell_time: Option<i64>,
    pub activities: Option<Vec<String>>,
    pub runtime_info_set: Option<HashMap<String, RuntimeInfoSet>>,
    pub prev_edges: Option<Vec<String>>,
    #[serde(rename = "isPreferred")]
    pub is_preferred: Option<bool>,
    #[serde(rename = "activityIds")]
    pub activity_ids: Option<Vec<usize>>,
    pub reachable_stopping_points: Option<Vec<String>>,
    pub reachable_tracks: Option<Vec<String>>,
    pub new_projected_length: Option<i64>,
    pub dwell_type: Option<DwellType>,
    pub relative_edt: Option<bool>,
    pub earliest_departure_time: Option<i64>,
    pub end_of_graph: Option<bool>,
    pub penalty: Option<i64>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeInfoSet {
    pub runtime_infos: Vec<RuntimeInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeInfo {
    pub running_time: i64,
    pub clearance: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeType {
    Track,
    StationRoute,
}
