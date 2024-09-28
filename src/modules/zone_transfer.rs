use std::collections::HashSet;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::tcp::TcpClientConnection;
use trust_dns_client::rr::{Name, RData, RecordType};
use trust_dns_resolver::TokioAsyncResolver;
use log::{info, warn};

async fn get_authoritative_dns(domain: &str) -> Vec<Ipv4Addr> {
    let resolver = TokioAsyncResolver::tokio_from_system_conf().expect("Failed to create DNS resolver");
    let ns_response = resolver.lookup(domain, RecordType::NS).await;
    let mut ips = Vec::new();

    match ns_response {
        Ok(ns_records) => {
            for ns_record in ns_records {
                if let RData::NS(name) = ns_record {
                    let ns_ip_response = resolver.lookup_ip(name.to_utf8()).await;
                    if let Ok(ns_ips) = ns_ip_response {
                        for ip in ns_ips.iter() {
                            if let std::net::IpAddr::V4(ipv4) = ip {
                                ips.push(ipv4);
                            }
                        }
                    }
                }
            }
        }
        Err(_) => {
            warn!("Failed to fetch NS records for the domain.");
        }
    }

    ips
}

fn zone_transfer(domain: &str, ns_ip: Ipv4Addr) -> HashSet<String> {
    let mut domains = HashSet::new();

    let socket_addr = SocketAddr::V4(SocketAddrV4::new(ns_ip, 53));
    let conn = match TcpClientConnection::new(socket_addr) {
        Ok(conn) => conn,
        Err(err) => {
            warn!("Failed to create TCP connection: {}", err);
            return domains;
        }
    };

    let client = SyncClient::new(conn);
    let name = match Name::from_ascii(domain) {
        Ok(name) => name,
        Err(err) => {
            warn!("Invalid domain name: {}", err);
            return domains;
        }
    };

    let transfer_result = client.zone_transfer(&name, None);
    match transfer_result {
        Ok(mut response) => {
            let mut success = false;

            while let Some(dns_response) = response.next() {
                match dns_response {
                    Ok(dns_response) => {
                        for record in dns_response.answers() {
                            let mut record_name = record.name().to_utf8();
                            if record_name.ends_with('.') {
                                record_name = record_name.trim_end_matches('.').to_string();
                            }
                            domains.insert(record_name);
                            success = true;
                        }
                    }
                    Err(err) => {
                        warn!("Error processing record: {}", err);
                    }
                }
            }

            if success {
                info!("{} -> {} | Zone transfer successful!", domain, ns_ip);
            } else {
                info!("No records found in zone transfer.");
            }
        }
        Err(err) => {
            warn!("Zone transfer failed: {}", err);
        }
    }
    domains
}

pub async fn call_zone_transfer(domain: &str) -> HashSet<String> {
    let ns_ips = get_authoritative_dns(domain).await;
    let domains = Arc::new(Mutex::new(HashSet::new()));
    let mut join_set = JoinSet::new();

    for ns_ip in ns_ips {
        let domain_clone = domain.to_string();
        let domains_arc = Arc::clone(&domains);

        join_set.spawn_blocking(move || {
            let subdomains = zone_transfer(&domain_clone, ns_ip);
            subdomains
        });

        if join_set.len() >= 12 {
            if let Some(result) = join_set.join_next().await {
                if let Ok(subdomains) = result {
                    let mut domains_locked = domains_arc.lock().await;
                    for subdomain in subdomains {
                        domains_locked.insert(subdomain);
                    }
                }
            }
        }
    }

    while let Some(result) = join_set.join_next().await {
        if let Ok(subdomains) = result {
            let mut domains_locked = domains.lock().await;
            for subdomain in subdomains {
                domains_locked.insert(subdomain);
            }
        }
    }

    let domains_locked = domains.lock().await;
    domains_locked.clone()
}
