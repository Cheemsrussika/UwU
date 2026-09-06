use std::io::BufReader;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use super::packet_frame::{read_packet_frame, write_packet_frame};
use super::protocol::Packet;

pub fn handle_client(
    stream: TcpStream,
    assigned_id: i32,
    running: Arc<AtomicBool>,
    incoming_packets: Arc<Mutex<Vec<Packet>>>,
    modified_blocks: Arc<Mutex<Vec<(i32, i32, i32, u8)>>>,
) {
    let mut writer = match stream.try_clone() {
        Ok(w) => w,
        Err(_) => return,
    };
    let mut reader = BufReader::new(stream);

    thread::Builder::new()
        .name("LanServerClientHandler".into())
        .spawn(move || {
            while running.load(Ordering::Relaxed) {
                match read_packet_frame(&mut reader) {
                    Ok((packet_id, payload)) => {
                        if let Some(packet) = Packet::decode(packet_id, &payload) {
                            if matches!(packet, Packet::ServerboundHello { .. }) {
                                let login_pkt = Packet::ClientboundLogin { entity_id: assigned_id };
                                let (pid, pld) = login_pkt.encode();
                                let _ = write_packet_frame(&mut writer, pid, &pld);

                                let mods = modified_blocks.lock().unwrap().clone();
                                for (x, y, z, bt) in mods {
                                    let b_pkt = Packet::ClientboundBlockUpdate { x, y, z, block_type: bt };
                                    let (b_pid, b_pld) = b_pkt.encode();
                                    let _ = write_packet_frame(&mut writer, b_pid, &b_pld);
                                }
                            }
                            incoming_packets.lock().unwrap().push(packet);
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut || e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(std::time::Duration::from_millis(5));
                        continue;
                    }
                    Err(_) => break,
                }
            }
        })
        .ok();
}
