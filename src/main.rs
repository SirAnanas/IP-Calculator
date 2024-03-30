use std::{io, net::Ipv4Addr, cmp::Ordering};
fn main() {
    loop {
        println!("IP Address Calculator. Made by Ananas");
        println!("What do you want to do:");

        println!("Calculate IP address -> c");
        println!("Exit -> q");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim();

        match choice {
            "q" | "quit" | "exit" | "Q" | "Quit" | "QUIT" => {
                println!("Goodbye.");
                break;
            },
            "c" | "calculate" | "C" | "CALCULATE" => ip_handling(),
            _ => eprintln!("Invalid input"),
        }
    }
}

fn ip_handling() {
    println!("Please input an IP Address");
    
    let ip_address: Ipv4Addr;
    let ip_address_class: char;
    let net_mask: Ipv4Addr;

    let subnet_count: u32;
    let maximum_subnet_count: u32;
    let subnet: u32;
    
    (ip_address, ip_address_class, maximum_subnet_count, net_mask) = ip_address_input();

    println!("Please input a subnet count (max count: {})", maximum_subnet_count);

    loop {
        let mut subnet_count_inn = String::new();

        io::stdin()
            .read_line(&mut subnet_count_inn)
            .expect("Failed to read line");

        let subnet_count_inn: u32 = match subnet_count_inn.trim().parse() {
            Ok(subnet_count_inn) => subnet_count_inn,
            Err(_) => {
                println!("Invalid subnet count, please try again");
                continue;
            },
        };

        match subnet_count_inn.cmp(&maximum_subnet_count) {
            Ordering::Greater => {
                println!("Invalid subnet count, please try again");
                continue;
            },
            _ => (),
        };

        subnet_count = subnet_count_inn;
        break;    
    }
    
    println!("Please input a subnet to calculate (max: {})", subnet_count);

    loop {
        let mut subnet_inn = String::new();

        io::stdin()
            .read_line(&mut subnet_inn)
            .expect("Failed to read line");

        let subnet_inn: u32 = match subnet_inn.trim().parse() {
            Ok(subnet_inn) => subnet_inn,
            Err(_) => {
                println!("Invalid subnet, please try again");
                continue;
            }
        };

        match subnet_inn.cmp(&subnet_count) {
            Ordering::Greater => {
                println!("Invalid subnet, please try again");
                continue;
            },
            _ => (),
        };
        subnet = subnet_inn;
        break;
    }

    let subnet_mask_bits = format!("{subnet_count:b}").len();
    let mut marked_subnet_mask_bits = String::new();
    for _ in 0..subnet_mask_bits {
        marked_subnet_mask_bits.push('1');
    }

    let mut reserved_bits: u32 = 0;
    while reserved_bits != marked_subnet_mask_bits.len() as u32 {
        reserved_bits += 1;
    }

    let subnet_amount = 2_u32.pow(reserved_bits);

    let host_per_subnet = match ip_address_class {
        'A' => 2_u32.pow(24 - reserved_bits) - 2,
        'B' => 2_u32.pow(16 - reserved_bits) - 2,
        'C' => 2_u32.pow(8 - reserved_bits) -2,
        _ => 0,
    };
    while marked_subnet_mask_bits.len() % 8 != 0 {
        marked_subnet_mask_bits.push('0');
    }
    
    let subnet_mask_1: u8;
    let subnet_mask_2: u8;
    let subnet_mask_3: u8;

    if marked_subnet_mask_bits.len() == 8 {
        subnet_mask_1 = match u8::from_str_radix(&marked_subnet_mask_bits, 2) {
            Ok(subnet_mask_1) => subnet_mask_1,
            Err(_) => 255,
        };
        subnet_mask_2 = 0;
        subnet_mask_3 = 0;
    } else if marked_subnet_mask_bits.len() == 16 {
        subnet_mask_1 = match u8::from_str_radix(&marked_subnet_mask_bits[..8], 2) {
            Ok(subnet_1) => subnet_1,
            Err(_) => 255,
        };
        subnet_mask_2 = match u8::from_str_radix(&marked_subnet_mask_bits[8..16], 2) {
            Ok(subnet_2) => subnet_2,
            Err(_) => 255,
        };
        subnet_mask_3 = 0;
    } else { 
        subnet_mask_1 = match u8::from_str_radix(&marked_subnet_mask_bits[..8], 2) {
            Ok(subnet_1) => subnet_1,
            Err(_) => 255,
        };
        subnet_mask_2 = match u8::from_str_radix(&marked_subnet_mask_bits[8..16], 2) {
            Ok(subnet_2) => subnet_2,
            Err(_) => 255,
        };
        subnet_mask_3 = match u8::from_str_radix(&marked_subnet_mask_bits[16..24], 2) {
            Ok(subnet_3) => subnet_3,
            Err(_) => 255,
        };
    }

    let subnet_mask: Ipv4Addr = match ip_address_class {
        'A' => Ipv4Addr::new(255, subnet_mask_1, subnet_mask_2, subnet_mask_3), 
        'B' => Ipv4Addr::new(255, 255, subnet_mask_1, subnet_mask_2),        
        'C' => Ipv4Addr::new(255, 255, 255, subnet_mask_1), 
        _ => Ipv4Addr::new(255, 255, 255, 255),
    };

    let subnet_other = subnet - 1;
    let subnet_bin = format!("{subnet_other:b}"); 
    let mut subnet_address = String::new();
    if subnet_bin.len() != subnet_mask_bits {
        for _ in 0..(subnet_mask_bits - subnet_bin.len()) {
            subnet_address.push('0');
        }
        subnet_address.push_str(&subnet_bin);
    }

    let mut broadcast_address = subnet_address.clone();
   
    while subnet_address.len() % 8 != 0 {
        subnet_address.push('0');
    }

    while broadcast_address.len() % 8 != 0 {
        broadcast_address.push('1');
    }

    let subnet_1: u8;
    let subnet_2: u8;
    let subnet_3: u8;

    if subnet_address.len() == 8 {
        subnet_1 = match u8::from_str_radix(&subnet_address, 2) {
            Ok(subnet_1) => subnet_1,
            Err(_) => 255,
        };
        subnet_2 = 0;
        subnet_3 = 0;
    } else if subnet_address.len() == 16 { 
        subnet_1 = match u8::from_str_radix(&subnet_address[..8], 2) {
            Ok(subnet_1) => subnet_1,
            Err(_) => 255,
        };
        subnet_2 = match u8::from_str_radix(&subnet_address[8..16], 2) {
            Ok(subnet_2) => subnet_2,
            Err(_) => 255,
        };
        subnet_3 = 0;
    } else {
        subnet_1 = match u8::from_str_radix(&subnet_address[..8], 2) {
            Ok(subnet_1) => subnet_1,
            Err(_) => 255,
        };
        subnet_2 = match u8::from_str_radix(&subnet_address[8..16], 2) {
            Ok(subnet_2) => subnet_2,
            Err(_) => 255,
        };
        subnet_3 = match u8::from_str_radix(&subnet_address[16..24], 2) {
            Ok(subnet_3) => subnet_3,
            Err(_) => 255,
        };
    }
    
    let broadcast_1: u8;
    let broadcast_2: u8;
    let broadcast_3: u8;

    if broadcast_address.len() == 8 {
        broadcast_1 = match u8::from_str_radix(&broadcast_address, 2) {
            Ok(broadcast_1) => broadcast_1,
            Err(_) => 255,
        };
        broadcast_2 = 255;
        broadcast_3 = 255;
    } else if broadcast_address.len() == 16 { 
        broadcast_1 = match u8::from_str_radix(&broadcast_address[..8], 2) {
            Ok(broadcast_1) => broadcast_1,
            Err(_) => 255,
        };
        broadcast_2 = match u8::from_str_radix(&broadcast_address[8..16], 2) {
            Ok(broadcast_2) => broadcast_2,
            Err(_) => 255,
        };
        broadcast_3 = 255;
    } else {
        broadcast_1 = match u8::from_str_radix(&broadcast_address[..8], 2) {
            Ok(broadcast_1) => broadcast_1,
            Err(_) => 255,
        };
        broadcast_2 = match u8::from_str_radix(&broadcast_address[8..16], 2) {
            Ok(broadcast_2) => broadcast_2,
            Err(_) => 255,
        };
        broadcast_3 = match u8::from_str_radix(&broadcast_address[16..24], 2) {
            Ok(broadcast_3) => broadcast_3,
            Err(_) => 255,
        };
    }

    let octets = ip_address.octets();

    let subnet_address = match ip_address_class {
        'A' => Ipv4Addr::new(octets[0], subnet_1, subnet_2, subnet_3),
        'B' => Ipv4Addr::new(octets[0], octets[1], subnet_1, subnet_2),
        'C' => Ipv4Addr::new(octets[0], octets[1], octets[2], subnet_1),
        _ => Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
    };
    let broadcast_address = match ip_address_class {
        'A' => Ipv4Addr::new(octets[0], broadcast_1, broadcast_2, broadcast_3),
        'B' => Ipv4Addr::new(octets[0], octets[1], broadcast_1, broadcast_2),
        'C' => Ipv4Addr::new(octets[0], octets[1], octets[2], broadcast_1),
        _ => Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
    };

    let range_min = Ipv4Addr::new(subnet_address.octets()[0], subnet_address.octets()[1], subnet_address.octets()[2], subnet_address.octets()[3] + 1);
    let range_max = Ipv4Addr::new(broadcast_address.octets()[0], broadcast_address.octets()[1], broadcast_address.octets()[2], broadcast_address.octets()[3] -1);

    println!("IP address: {}", ip_address);
    println!("IP address class: {}", ip_address_class);
    println!("IP address mask: {}", net_mask);
    println!("Maximum subnet count: {}", maximum_subnet_count);
    println!("Subnet count: {}", subnet_count);
    println!("Subnet mask: {}", subnet_mask);
    println!("Chosen subnet: {}", subnet);
    println!("Subnet address: {}", subnet_address);
    println!("Broadcast address: {}", broadcast_address);
    println!("Host address range: {} - {}", range_min, range_max);
    println!("Amount of subnets: {}", subnet_amount);
    println!("Amount of host addresses per subnet: {}", host_per_subnet);
    println!("Amount of host addresses on all subnets combined: {}", subnet_amount * host_per_subnet);
}

fn ip_address_input() -> (Ipv4Addr, char, u32, Ipv4Addr) {
    loop {
        let mut ip_address = String::new(); 
        let ip_address_class: char;
        let net_mask: Ipv4Addr;

        let maximum_subnet_count: u32;
        
        io::stdin()
            .read_line(&mut ip_address)
            .expect("Failed to read line");

        let ip_address: Ipv4Addr = match ip_address.trim().parse() {
            Ok(ip_address) => ip_address,
            Err(_) => {
                println!("Invalid IP Address, please input a correct IPv4 Address.");
                continue
            },
        };

        let ip_address_octets = ip_address.octets();

        match ip_address_octets[0] {
            1..=126 => {
                ip_address_class = 'A';
                maximum_subnet_count = 2_u32.pow(22);
                net_mask = Ipv4Addr::new(255, 0, 0, 0);
            },
            128..=191 => {    
                ip_address_class = 'B';
                maximum_subnet_count = 2_u32.pow(14);
                net_mask = Ipv4Addr::new(255, 255, 0, 0);
            },
            192..=223 => {
                ip_address_class = 'C';
                maximum_subnet_count = 2_u32.pow(6);
                net_mask = Ipv4Addr::new(255, 255, 255, 0);
            },
            _ => {
                println!("Invalid IP Address, please input a correct IPv4 Address.");
                continue
            },
        };
        break (ip_address, ip_address_class, maximum_subnet_count, net_mask);
    }
}
