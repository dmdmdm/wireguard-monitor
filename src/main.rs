use std::process::{Command, Stdio};
use std::cmp::min;
use std::io::Read;
use cursive::align::HAlign;
use cursive::views::{ResizedView, Dialog, LinearLayout, TextContent, TextView, Panel};
use cursive::view::{SizeConstraint, ScrollStrategy};
use cursive::traits::*;

#[link(name = "c")]
unsafe extern "C" {
    fn geteuid() -> u32;
}

fn is_root() -> bool {
    unsafe {
        return geteuid() == 0;
    }
}

fn get_wg() -> String {
    if !is_root() {
        return "You must be root to run `wg`".to_string();
    }

    let output = Command::new("wg").arg("show").output().expect("Could not run 'wg show'");
    return String::from_utf8_lossy(&output.stdout).to_string();
}

fn get_ifconfig() -> String {
    let output = Command::new("ifconfig").arg("wg0").output().expect("Could not run 'ifconfig wg0'");
    return String::from_utf8_lossy(&output.stdout).to_string();
}

fn background_top(content_wg: TextContent, content_ifconfig: TextContent) {
    let half_sec = std::time::Duration::from_millis(500);
    loop {
         content_wg.set_content(get_wg());
         std::thread::sleep(half_sec);
         content_ifconfig.set_content(get_ifconfig());
         std::thread::sleep(half_sec);
    }
}

fn vec_to_text(vec: &Vec<String>) -> String {
    let mut txt: String = "".to_owned();
    for line in vec.iter() {
        let bline = &line;
        txt.push_str(bline);
        txt.push_str("\n");
    }
    return txt;
}

fn background_tcpdump(content_tcpdump: TextContent) {
    if !is_root() {
        content_tcpdump.set_content("You must be root to run `tcpdump`");
        return;
    }

    let child = Command::new("tcpdump")
        .arg("-iwg0")
        .arg("-l")          // Line buffering (avoids delay)
        .arg("-n")          // No DNS lookups
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn();
    if child.is_err() {
        content_tcpdump.set_content("Could not run tcpdump - permission?");
        return;
    }
    let stdout_result = child.unwrap().stdout;

    if stdout_result.is_none() {
        content_tcpdump.set_content("Could not get output from tcpdump");
        return;
    }
    let stdout = stdout_result.unwrap();

    content_tcpdump.set_content("tcpdump started, waiting for wg0 traffic");

    let mut vec = Vec::<String>::new();

    let mut byte_line = Vec::new();
    for byte_result in stdout.bytes() {
        let byte = byte_result.unwrap();
        if byte == 0x0A {
            let line = String::from_utf8_lossy(&byte_line).to_string();
            byte_line.clear();
            vec.push(line);

            if vec.len() > 100 {
                vec.remove(0);
            }

            content_tcpdump.set_content(vec_to_text(&vec));
        }
        else {
            byte_line.push(byte);
        }
    }
}

fn main() {
    let mut siv = cursive::default();
    let size = siv.screen_size();
    let height = size.y;
    let box_height = height / 3;

    let content_wg = TextContent::new("wg...");
    let tv1 = TextView::new_with_content(content_wg.clone())
        .no_wrap()
        .with_name("tv1")
        .scrollable();
    let swidth = SizeConstraint::Full;
    let sheight = SizeConstraint::AtMost(box_height);
    let box1 = ResizedView::new(swidth, sheight, tv1);
    let pan1 = Panel::new(box1).title("wg show");
    
    let content_ifconfig = TextContent::new("ifconfig...");
    let tv2 = TextView::new_with_content(content_ifconfig.clone())
        .no_wrap()
        .with_name("tv2")
        .scrollable();
    let box2 = ResizedView::with_max_height(min(7, box_height), tv2);
    let pan2 = Panel::new(box2).title("ifconfig");

    let content_tcpdump = TextContent::new("tcpdump...");
    let mut tv3 = TextView::new_with_content(content_tcpdump.clone())
        .no_wrap()
        .with_name("tv3")
        .scrollable();
    tv3.set_scroll_strategy(ScrollStrategy::StickToBottom);
    let box3 = ResizedView::with_max_height(box_height, tv3);
    let pan3 = Panel::new(box3).title("tcpdump");

    let tv4 = TextView::new("Press 'q' to quit");

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(pan1)
                .child(pan2)
                .child(pan3)
                .child(tv4)
        )
        .title("wireguard monitor")
        .h_align(HAlign::Center),
    );

    siv.add_global_callback('q', |s| s.quit());

    std::thread::spawn(move || { background_top(content_wg, content_ifconfig) });
    std::thread::spawn(move || { background_tcpdump(content_tcpdump) });

    siv.set_fps(1);

    siv.run();
}
