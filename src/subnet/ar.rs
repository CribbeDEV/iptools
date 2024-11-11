use std::net::Ipv4Addr;
use cidr::Ipv4Cidr;

pub fn calculate_subnet_info(ip_cidr: &str) -> Result<super::SubnetInfo, &'static str> {
    let parts: Vec<&str> = ip_cidr.split('/').collect();
    if parts.len() != 2 {
        return Err("Invalid IP/CIDR format");
    }

    let ip = parts[0].parse::<Ipv4Addr>()
        .map_err(|_| "Invalid IP address")?;
    
    let cidr: u8 = parts[1].parse()
        .map_err(|_| "Invalid CIDR")?;
    if cidr > 32 {
        return Err("CIDR must be <= 32");
    }

    let ip_int: u32 = u32::from(ip);
    
    let mask_int: u32 = if cidr == 0 { 
        0 
    } else { 
        !0 << (32 - cidr)
    };
    
    let network_int: u32 = ip_int & mask_int;
    let broadcast_int: u32 = network_int | !mask_int;
    
    let network = Ipv4Addr::from(network_int);
    let broadcast = Ipv4Addr::from(broadcast_int);
    let subnet_mask = Ipv4Addr::from(mask_int);
    
    let usable = if cidr < 31 {
        format!("{}-{}", Ipv4Addr::from(network_int + 1), Ipv4Addr::from(broadcast_int - 1))
    } else {
        String::from("N/A")
    };

    Ok(super::SubnetInfo {
        full: ip_cidr.to_string(),
        network: format!("{}/{}", network, cidr),
        usable_range: usable,
        broadcast: broadcast.to_string(),
        subnet_mask: subnet_mask.to_string(),
    })
}

pub fn cidr_info(c: u8) {
    let mask = match Ipv4Cidr::new(Ipv4Addr::new(10, 0, 0, 0), c) {
        Ok(r) => r.mask(),
        Err(e) => panic!("{}", e),
    };
    let wildcard_mask: [u8; 4] = mask.octets().map(|octet| !octet);
    println!("\nMask: {}", mask);
    println!("Wildcard Mask: {}.{}.{}.{}\n", wildcard_mask[0], wildcard_mask[1], wildcard_mask[2], wildcard_mask[3]);
}

pub fn subnet_to_wildcard(subnet: &str) {
    let subnet = subnet.parse::<Ipv4Addr>().unwrap();
    let wildcard_mask: [u8; 4] = subnet.octets().map(|octet| !octet);
    println!("\nWildcard Mask: {}.{}.{}.{}\n", wildcard_mask[0], wildcard_mask[1], wildcard_mask[2], wildcard_mask[3]);
}
