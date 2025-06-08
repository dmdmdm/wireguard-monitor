// Provide friendly peer names the same way this project does
// https://github.com/FlyveHest/wg-friendly-peer-names
use std::process::Command;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use chrono::offset::Utc;
use chrono::DateTime;
use regex::Regex;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::sync::Arc;
use cursive::utils::span::SpannedString;
use cursive::theme::Style;

#[path = "./my_style.rs"]
mod my_style;

fn is_root() -> bool {
    return users::get_current_uid() == 0
}

fn get_wg() -> String {
    if !is_root() {
        return "You must be root to run `wg`".to_string();
    }

    // We tried setting WG_COLORMODE=always btu the colors didn't look nice
    let output = Command::new("wg").arg("show").output().expect("Could not run 'wg show'");
    return String::from_utf8_lossy(&output.stdout).to_string();
}

struct Peers {
    modtime: SystemTime,
    peers_map: HashMap<String,String>
}

lazy_static! {
    static ref LOADED_PEERS: Arc<Mutex<Option<Peers>>> = Arc::new(Mutex::new(None));
}

#[allow(dead_code)]
fn format_systime(st: SystemTime) -> String {
    let datetime: DateTime<Utc> = st.into();
    return datetime.format("%d/%m/%Y %T").to_string();
}

fn get_loaded_peers_modtime() -> SystemTime {
    let peers_opt = LOADED_PEERS.lock().unwrap();
    if peers_opt.is_none() { return UNIX_EPOCH; }
    return peers_opt.as_ref().unwrap().modtime.clone();
}

fn save_loaded_peers(modtime: SystemTime, peers_map: &HashMap<String,String>) {
    let mut peers_opt = LOADED_PEERS.lock().unwrap();
    *peers_opt = Some(Peers { modtime:modtime, peers_map:peers_map.clone() });
}

fn get_loaded_peers_map() -> HashMap<String,String> {
    let peers_opt = LOADED_PEERS.lock().unwrap();
    if peers_opt.is_none() { return HashMap::new(); }
    return peers_opt.as_ref().unwrap().peers_map.clone();
}

fn get_file_modtime(filename: &str) -> SystemTime {
    let metadata = fs::metadata(filename);
    if metadata.is_err() { return UNIX_EPOCH; }
    return metadata.unwrap().modified().unwrap();
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

fn load_peers(filename: &str) -> HashMap<String,String> {
    let mut map: HashMap::<String,String> = HashMap::new();
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

fn merge(wg: &String, peers_map: &HashMap::<String,String>) -> SpannedString<Style> {
    let mut out = SpannedString::<Style>::new();

    let re_interface = Regex::new("^(interface):\\s*(.+)").unwrap();
    let re_peer = Regex::new("^(peer):\\s*(.+)").unwrap();

    for line in wg.lines() {
        let caps_interface = re_interface.captures(&line);
        if caps_interface.is_some() {
            let c = caps_interface.unwrap();
            let label = c.get(1).unwrap().as_str().trim().to_string();
            let interface = c.get(2).unwrap().as_str().trim().to_string();
            let label_style = my_style::bold_green_string(&label);
            let interface_style = my_style::bold_green_string(&interface);
            out.append(label_style);
            out.append(": ");
            out.append(interface_style);
            out.append("\n");
        }
        else {
            out.append(line);
            out.append("\n");
        }
        let caps_peer = re_peer.captures(&line);
        if caps_peer.is_some() {
            let c = caps_peer.unwrap();
            let public_key = c.get(2).unwrap().as_str().trim().to_string();
            let result = peers_map.get(&public_key);
            if result.is_some() {
                let label = "friendly-name";
                let friendly = result.unwrap();
                let label_style = my_style::bold_yellow_string(&label);
                let friendly_style = my_style::bold_yellow_string(&friendly);
                out.append("  ");
                out.append(label_style);
                out.append(": ");
                out.append(friendly_style);
                out.append("\n");
            }
        }
    }

    return out;
}

pub fn get_wgg() -> SpannedString<Style> {
    if !is_root() {
        return my_style::plain_string("You must be root to run `wg`");
    }
    let wg = get_wg();

    let filename = "/etc/wireguard/peers";
    let modtime_file = get_file_modtime(&filename);
    let modtime_loaded = get_loaded_peers_modtime();
    let peers_map: HashMap<String,String>;
    if modtime_file == modtime_loaded {
        peers_map = get_loaded_peers_map();
    }
    else {
        peers_map = load_peers(&filename);
        save_loaded_peers(modtime_file, &peers_map);
    }

    return merge(&wg, &peers_map);
}
