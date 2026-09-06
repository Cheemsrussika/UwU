import os

lan_detector = "use std::net::{Ipv4Addr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use super::lan_discovery::{parse_address, parse_motd, DiscoveredServer};

pub struct LanServerDetector {
    servers: Arc<Mutex<Vec<DiscoveredServer>>>,
    running: Arc<AtomicBool>,
}

impl LanServerDetector {
    pub const MULTICAST_IP: Ipv4Addr = Ipv4Addr::new(224, 0, 2, 60);
    pub const PORT: u16 = 4445;

    pub fn new() -> Self {
        let servers = Arc::new(Mutex::new(Vec::new()));
        let running = Arc::new(AtomicBool::new(true));
        let servers_clone = servers.clone();
        let running_clone = running.clone();

        thread::Builder::new()
            .name(LanServerDetector.into())
            .spawn(move || {
                let socket = match UdpSocket::bind((0.0.0.0, Self::PORT)) {
                    Ok(s) => s,
                    Err(e) => {
                        log::warn!(LanServerDetector bind failed: {}, e);
                        return;
                    }
                };
                let _ = socket.join_multicast_v4(&Self::MULTICAST_IP, &Ipv4Addr::UNSPECIFIED);
                let _ = socket.set_read_timeout(Some(Duration::from_millis(1000)));

                let mut buf = [0u8; 1024];
                while running_clone.load(Ordering::Relaxed) {
                    if let Ok((len, src)) = socket.recv_from(&mut buf) {
                        if let Ok(text) = std::str::from_utf8(&buf[..len]) {
                            let motd = parse_motd(text);
                            if let Some(port_str) = parse_address(text) {
                                let addr = format!({}:{}, src.ip(), port_str);
                                let mut list = servers_clone.lock().unwrap();
                                if let Some(srv) = list.iter_mut().find(|s| s.address == addr) {
                                    srv.last_seen = Instant::now();
                                } else {
                                    list.push(DiscoveredServer::new(motd, addr));
                                }
                            }
                        }
                    }
                    let now = Instant::now();
                    let mut list = servers_clone.lock().unwrap();
                    list.retain(|s| now.duration_since(s.last_seen).as_secs() < 5);
                }
            })
            .ok();

        Self { servers, running }
    }

    pub fn get_servers(&self) -> Vec<DiscoveredServer> {
        self.servers.lock().unwrap().clone()
    }
}

impl Drop for LanServerDetector {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
"

with open(src/network/lan_detector.rs, w, encoding=utf-8) as f:
    f.write(lan_detector.strip() + \n)
print(lan_detector.rs created)
