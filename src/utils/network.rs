use std::net::IpAddr;

pub const PORT: u16 = 5000;

pub fn is_loopback_host(host: &str) -> bool {
    let h = host.split(':').next().unwrap_or(host).trim().to_lowercase();
    h == "localhost" || h == "127.0.0.1" || h == "::1" || h.starts_with("[::1]")
}

fn is_private_ipv4(o: [u8; 4]) -> bool {
    o[0] == 10
        || (o[0] == 172 && (16..=31).contains(&o[1]))
        || (o[0] == 192 && o[1] == 168)
}

fn ip_rank(ip: &str) -> u8 {
    if ip.starts_with("192.168.") {
        0
    } else if ip.starts_with("10.") {
        1
    } else {
        2
    }
}

pub fn list_lan_ips() -> Vec<String> {
    let mut ips: Vec<String> = Vec::new();

    if let Ok(interfaces) = local_ip_address::list_afinet_netifas() {
        for (_name, addr) in interfaces {
            if let IpAddr::V4(v4) = addr {
                let octets = v4.octets();
                if is_private_ipv4(octets) {
                    let s = v4.to_string();
                    if !ips.contains(&s) {
                        ips.push(s);
                    }
                }
            }
        }
    }

    if ips.is_empty() {
        if let Ok(IpAddr::V4(v4)) = local_ip_address::local_ip() {
            ips.push(v4.to_string());
        }
    }

    ips.sort_by_key(|ip| ip_rank(ip));
    ips
}

pub fn lan_urls() -> Vec<String> {
    list_lan_ips()
        .into_iter()
        .map(|ip| format!("http://{}:{}", ip, PORT))
        .collect()
}

/// Pick the URL other devices should use to join this share.
pub fn resolve_share_url(request_host: Option<&str>) -> (String, Vec<String>, bool) {
    let urls = lan_urls();

    if let Some(host) = request_host {
        let hostname = host.split(':').next().unwrap_or(host);
        if !is_loopback_host(hostname) {
            let share = if host.contains(':') {
                format!("http://{}", host)
            } else {
                format!("http://{}:{}", host, PORT)
            };
            return (share, urls, false);
        }
    }

    let share = urls
        .first()
        .cloned()
        .unwrap_or_else(|| format!("http://localhost:{}", PORT));

    let host_is_local = request_host
        .map(is_loopback_host)
        .unwrap_or(true);

    (share, urls, host_is_local)
}
