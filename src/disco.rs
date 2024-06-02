use crate::{HueError, HueError::DiscoveryError};
use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Record, RecordKind};
use std::{net::IpAddr, time::Duration};

// As Per instructions at
// https://developers.meethue.com/develop/application-design-guidance/hue-bridge-discovery/
pub async fn discover_hue_bridge() -> Result<IpAddr, HueError> {
    let bridge = discover_hue_bridge_m_dns().await;
    match bridge {
        Ok(bridge_ip) => Ok(bridge_ip),
        Err(_) => {
            // let n_upnp_result = discover_hue_bridge_n_upnp();
            // if n_upnp_result.is_err() {
                Err(DiscoveryError {
                    msg: "Could not discover bridge".into(),
                })?
            // } else {
            //     n_upnp_result
            // }
        }
    }
}

// Define the service name for hue bridge
const SERVICE_NAME: &str = "_hue._tcp.local";

// Define a function that discovers a hue bridge using mDNS
pub async fn discover_hue_bridge_m_dns() -> Result<IpAddr, HueError> {
    // Iterate through responses from each hue bridge device, asking for new devices every 15s
    let stream_disc = mdns::discover::all(SERVICE_NAME, Duration::from_secs(1));
    let stream = match stream_disc {
        Ok(s) => s.listen(),
        Err(_e) => {
            return Err(DiscoveryError {
                msg: _e.to_string(),
            })
        }
    };
    pin_mut!(stream);
    let response = tokio::time::timeout(Duration::from_secs(5), stream.next()).await;
    match response {
        Ok(Some(Ok(response))) => {
            // Get the first IP address from the response
            let ip = response
                .records()
                .filter_map(to_ip_addr)
                .next()
                .ok_or(DiscoveryError {
                    msg: "No IP address found in response".into(),
                })?;
            Ok(ip)
        }
        Ok(Some(Err(e))) => Err(DiscoveryError { msg: e.to_string() }),
        Ok(None) => Err(DiscoveryError {
            msg: "No response from bridge".into(),
        }),
        Err(_e) => Err(DiscoveryError {
            msg: "No response from bridge".into(),
        }),
    }
}

// Define a helper function that converts a record to an IP address
fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}
