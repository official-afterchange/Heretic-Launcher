use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

#[derive(Serialize)]
pub struct ServerStatus {
    pub online: bool,
    pub players_online: u32,
    pub players_max: u32,
    pub ping_ms: u64,
}

#[derive(Deserialize)]
pub struct StatusJson {
    players: PlayersJson,
}

#[derive(Deserialize)]
struct PlayersJson {
    online: u32,
    max: u32
}

fn write_varint(buf: &mut Vec<u8>, mut v: u32) {
    loop {
        let mut b = (v & 0x7F) as u8;
        v >>= 7;
        if v != 0 {
            b |= 0x80;
        }
        buf.push(b);
        if v == 0 {
            break;
        }
    }
}

async fn read_varint(s: &mut TcpStream) -> Result<u32, String> {
    let mut result = 0u32;
    for i in 0..5 {
        let b = s.read_u8().await.map_err(|e| e.to_string())?;
        result |= ((b & 0x7F) as u32) << (7 * i);
        if b & 0x80 == 0 {
            return Ok(result);
        }
    }
    Err("VarInt trop long".into())
}

async fn send_packet(s: &mut TcpStream, payload: &[u8]) -> Result<(), String> {
    let mut out = Vec::new();
    write_varint(&mut out, payload.len() as u32);
    out.extend_from_slice(payload);
    s.write_all(&out).await.map_err(|e| e.to_string())
}

async fn query(host: &str, port: u16) -> Result<ServerStatus, String> {
    let mut stream = TcpStream::connect((host, port))
        .await
        .map_err(|e| e.to_string())?;

    // 1. Handshake (id 0x00, état suivant = 1 : status)
    let mut hs = Vec::new();
    write_varint(&mut hs, 0x00);
    write_varint(&mut hs, 0); // version de protocole (0 accepté pour le status)
    write_varint(&mut hs, host.len() as u32);
    hs.extend_from_slice(host.as_bytes());
    hs.extend_from_slice(&port.to_be_bytes());
    write_varint(&mut hs, 1);
    send_packet(&mut stream, &hs).await?;

    // 2. Status request (id 0x00, sans données)
    send_packet(&mut stream, &[0x00]).await?;

    // 3. Réponse : longueur, id, longueur du JSON, JSON
    let _packet_len = read_varint(&mut stream).await?;
    let _packet_id = read_varint(&mut stream).await?;
    let json_len = read_varint(&mut stream).await? as usize;
    let mut json = vec![0u8; json_len];
    stream.read_exact(&mut json).await.map_err(|e| e.to_string())?;
    let status: StatusJson = serde_json::from_slice(&json).map_err(|e| e.to_string())?;

    // 4. Ping (id 0x01 + un i64 arbitraire), on mesure l'aller-retour
    let mut ping = Vec::new();
    write_varint(&mut ping, 0x01);
    ping.extend_from_slice(&0i64.to_be_bytes());
    let start = Instant::now();
    send_packet(&mut stream, &ping).await?;
    let _len = read_varint(&mut stream).await?;
    let mut pong = vec![0u8; 9]; // id (1 octet) + i64
    stream.read_exact(&mut pong).await.map_err(|e| e.to_string())?;
    let ping_ms = start.elapsed().as_millis() as u64;

    Ok(ServerStatus {
        online: true,
        players_online: status.players.online,
        players_max: status.players.max,
        ping_ms,
    })
}

#[tauri::command]
pub async fn get_server_status(address: String) -> Result<ServerStatus, String> {
    let (host, port) = match address.rsplit_once(':') {
        Some((h, p)) => (h.to_string(), p.parse::<u16>().map_err(|_| "Port invalide")?),
        None => (address.clone(), 25565),
    };

    // Timeout de 5 s, et un serveur injoignable = "offline" plutôt qu'une erreur
    match timeout(Duration::from_secs(5), query(&host, port)).await {
        Ok(Ok(status)) => Ok(status),
        _ => Ok(ServerStatus {
            online: false,
            players_online: 0,
            players_max: 0,
            ping_ms: 0,
        }),
    }
}