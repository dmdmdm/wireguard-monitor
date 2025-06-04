// Provide friendly peer names the same way this project does
// https://github.com/FlyveHest/wg-friendly-peer-names
use std::process::Command;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

fn is_root() -> bool {
    return users::get_current_uid() == 0
}

fn get_wg() -> String {
    if !is_root() {
        return "You must be root to run `wg`".to_string();
    }

    let output = Command::new("wg").arg("show").output().expect("Could not run 'wg show'");
    return String::from_utf8_lossy(&output.stdout).to_string();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(dead_code)]
fn dump_peers(map: &HashMap<String,String>) -> String {
    let mut out = String::from("");
    for (key,value) in map {
        out += &format!("{} is {}\n", key, value);
    }
    return out;
}

// Possible Improvement: cache this and only read when it changes
fn load_peers() -> HashMap<String,String> {
    let mut map: HashMap::<String,String> = HashMap::new();
    let filename = "/etc/wireguard/peers";
    if !Path::new(filename).is_file() { return map; }
    let result = read_lines(filename);
    if result.is_err() { return map; }
    let lines = result.unwrap();
    for line in lines.into_iter().map_while(Result::ok) {
        let split: Vec<&str> = line.split(':').collect();
        if split.len() != 2 { continue; }
        let key = split[0];
        let name = split[1];
        map.insert(key.to_owned(), name.to_owned());
    }
    return map;
}

fn merge(wg: &String, peers: &HashMap::<String,String>) -> String {
    let mut out = String::from("");

    let re_peer = Regex::new("^peer:\\s*(.+)").unwrap();

    for line in wg.lines() {
        out.push_str(line);
        out.push_str("\n");
        let caps_peer = re_peer.captures(&line);
        if caps_peer.is_some() {
            let c = caps_peer.unwrap();
            let public_key = c.get(1).unwrap().as_str().trim().to_string();
            let result = peers.get(&public_key);
            if result.is_some() {
                out.push_str(&format!("  friendly-name: {}\n", result.unwrap()));
            }
        }
    }

    return out;
}

pub fn get_wgg() -> String {
    if !is_root() {
        return "You must be root to run `wg`".to_string();
    }
    let wg = get_wg();
    let peers = load_peers();
    let merged = merge(&wg, &peers);
    return merged
}
