use std::fs;
use std::io;
use std::collections::HashMap;
use std::path::Path;
use std::io::prelude::*;
use std::str::FromStr;
use std::result::Result;


fn main(){

fn average_distance(node_id: &str) -> f64 {
    let edges_file = format!("{}.edges", node_id);
    let circles_file = format!("{}.circles", node_id);
    let feat_file = format!("{}.feat", node_id);
    let egofeat_file = format!("{}.egofeat", node_id);
    let featnames_file = format!("{}.featnames", node_id);

    let edges = read_edges(&edges_file);
    let circles = read_circles(&circles_file);
    let features = read_features(&feat_file);
    let ego_features = read_ego_features(&egofeat_file);
    let feature_names = read_feature_names(&featnames_file);

    let mut total_distance = 0.0; 
    let mut count = 0;

    for friend in edges.unwrap_or_else(|err| {
        return Vec::new();
    }) {
        if let Some(friend_features) = features.unwrap_or_else(|err| {

            return HashMap::new();
        }).get(&friend) {
            let distance = compute_distance
            (
                &ego_features,
                &friend_features,
                &feature_names,
            );
            total_distance += distance;
            count += 1;
        }
    }

    total_distance / count as f64
}


fn read_edges(file_path: &str) -> Result<Vec<String>, io::Error> {
    let mut edges = Vec::new();

    let file = fs::File::open(file_path)?;
    let mut reader = io::BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let edge: Vec<String> = line.trim().split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        edges.push(edge);
        line.clear();
    }

    Ok(edges)
}

fn read_circles(file_path: &str) -> Result<Vec<Vec<String>>, io::Error> {
    let mut circles = Vec::new();

    let file = fs::File::open(file_path)?;
    let mut reader = io::BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let circle: Vec<String> = line
            .trim()
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        circles.push(circle);
        line.clear();
    }

    Ok(circles)
}


fn read_features(file_path: &str) -> Result<HashMap<String, Vec<u8>>, io::Error> {
    let mut features = HashMap::new();

    let file = fs::File::open(file_path)?;
    let mut reader = io::BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let parts: Vec<&str> = line.trim().split(" ").collect();
        let node_id = parts[0].to_string();
        let node_features = parts[1..]
            .iter()
            .map(|s| u8::from_str(s).unwrap())
            .collect();
        features.insert(node_id, node_features);
        line.clear();
    }

    Ok(features)
}

fn read_ego_features(file_path: &str) -> Result<Vec<u8>, io::Error> {
    let mut ego_features = Vec::new();

    let file = fs::File::open(file_path)?;
    let mut reader = io::BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let ego_features: Vec<u8> = line
    }

    Ok(ego_features)
}

fn read_feature_names(file_path: &str) -> Result<Vec<String>, io::Error> {
    let mut feature_names = Vec::new();

    let file = fs::File::open(file_path)?;
    let mut reader = io::BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let feature_names: Vec<String> = line
        feature_names.append(&mut feature_names);
        line.clear();
    }

    Ok(feature_names)
}


fn compute_distance(
    ego_features: &Vec<u8>,
    friend_features: &Vec<u8>,
    feature_names: &Vec<String>,
) -> f64 {
    let mut distance: f32 = 0.0;

    distance += 1.0;
}
}