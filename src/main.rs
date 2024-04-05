use std::{io, net::Ipv4Addr, cmp::Ordering};

#[derive(Debug)]
enum IPClass {
    A,
    B,
    C,
}


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
    let ip_address_class: IPClass;
    let net_mask: Ipv4Addr;

    let subnet_count: u32;
    let theor_maximum_subnet_count: u32;
    let subnet: u32;
    
    (ip_address, ip_address_class, theor_maximum_subnet_count, net_mask) = ip_address_input();
    
    let maximum_subnet_count = theor_maximum_subnet_count - 1;
    
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

        match subnet_count_inn {
            0  | 1 => {
                println!("Invalid subnet, please try again.");
                continue;
            }
            _ => (),
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

        match subnet_inn {
            0 => {
                println!("Invalid subnet, please try again.");
                continue;
            }
            _ => (),
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
        IPClass::A => 2_u32.pow(24 - reserved_bits) - 2,
        IPClass::B => 2_u32.pow(16 - reserved_bits) - 2,
        IPClass::C => 2_u32.pow(8 - reserved_bits) -2,
    };
    while marked_subnet_mask_bits.len() % 8 != 0 {
        marked_subnet_mask_bits.push('0');
    }

    let subnet_other = subnet - 1;
    let subnet_bin = format!("{subnet_other:b}"); 
    let mut subnet_address = String::new();

    if subnet_bin.len() != subnet_mask_bits {
        for _ in 0..(subnet_mask_bits - subnet_bin.len()) {
            subnet_address.push('0');
        }
    }

    subnet_address.push_str(&subnet_bin);

    let mut broadcast_address = subnet_address.clone();
   
    while subnet_address.len() % 8 != 0 {
        subnet_address.push('0');
    }

    while broadcast_address.len() % 8 != 0 {
        broadcast_address.push('1');
    }

    let subnet_mask = address_sanitization(marked_subnet_mask_bits, ip_address, &ip_address_class, 2);

    let subnet_address = address_sanitization(subnet_address, ip_address, &ip_address_class, 0);
    let broadcast_address = address_sanitization(broadcast_address, ip_address, &ip_address_class, 1);

    let range_min = Ipv4Addr::new(subnet_address.octets()[0], subnet_address.octets()[1], subnet_address.octets()[2], subnet_address.octets()[3] + 1);
    let range_max = Ipv4Addr::new(broadcast_address.octets()[0], broadcast_address.octets()[1], broadcast_address.octets()[2], broadcast_address.octets()[3] -1);

    println!("IP address: {}", ip_address);
    println!("IP address class: {:?}", ip_address_class);
    println!("IP address mask: {}", net_mask);
    println!("Theoretical maximum subnet count: {}", theor_maximum_subnet_count);
    println!("Usable maximum subnet count: {}", maximum_subnet_count);
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

fn ip_address_input() -> (Ipv4Addr, IPClass, u32, Ipv4Addr) {
    loop {
        let mut ip_address = String::new(); 
        let ip_address_class: IPClass;
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
                ip_address_class = IPClass::A;
                maximum_subnet_count = 2_u32.pow(22);
                net_mask = Ipv4Addr::new(255, 0, 0, 0);
            },
            128..=191 => {    
                ip_address_class = IPClass::B;
                maximum_subnet_count = 2_u32.pow(14);
                net_mask = Ipv4Addr::new(255, 255, 0, 0);
            },
            192..=223 => {
                ip_address_class = IPClass::C;
                maximum_subnet_count = 2_u32.pow(6);
                net_mask = Ipv4Addr::new(255, 255, 255, 0);
            },
            _ => {
                println!("Invalid IP Address, please input a correct IPv4 Address.");
                continue
            },
        };

        match ip_address_class {
            IPClass::A => {
                if ip_address_octets[1] != 0 || ip_address_octets[2] != 0 || ip_address_octets[3] != 0 {
                    println!("Invalid IP Address, please input a correct IPv4 Address.");
                    continue;
                }    
            },
            IPClass::B => {
                if ip_address_octets[2] != 0 || ip_address_octets[3] != 0 {
                    println!("Invalid IP Address, please input a correct IPv4 Address.");
                    continue;
                }    
            },
            IPClass::C => {
                if ip_address_octets[3] != 0 {
                    println!("Invalid IP Address, please input a correct IPv4 Address.");
                    continue;
                }
            },
        }


        break (ip_address, ip_address_class, maximum_subnet_count, net_mask);
    }
}

fn address_sanitization(address_tsan: String, ip_address: Ipv4Addr, ip_address_class: &IPClass, mode: u8) -> Ipv4Addr {
    let octet_1: u8;
    let mut octet_2: u8;
    let mut octet_3: u8;
    let octets = ip_address.octets();

    let address_san: Ipv4Addr;

    match mode {
        0 | 2 => {
            octet_2 = 0;
            octet_3 = 0;
        }
        1 => {
            octet_2 = 255;
            octet_3 = 255;
        }
        _ => {
            octet_2 = 0;
            octet_3 = 0;
        }
    }

    if address_tsan.len() == 8 {
        octet_1 = match u8::from_str_radix(&address_tsan[..8], 2) {
            Ok(octet_1) => octet_1,
            Err(_) => 255,
        };
    } else if address_tsan.len() == 16 {
        octet_1 = match u8::from_str_radix(&address_tsan[..8], 2) {
            Ok(octet_1) => octet_1,
            Err(_) => 255,
        };
        octet_2 = match u8::from_str_radix(&address_tsan[8..16], 2) {
            Ok(octet_2) => octet_2,
            Err(_) => 255,
        };
    } else {
        octet_1 = match u8::from_str_radix(&address_tsan[..8], 2) {
            Ok(octet_1) => octet_1,
            Err(_) => 255,
        };
        octet_2 = match u8::from_str_radix(&address_tsan[8..16], 2) {
            Ok(octet_2) => octet_2,
            Err(_) => 255,
        };
        octet_3 = match u8::from_str_radix(&address_tsan[16..24], 2) {
            Ok(octet_3) => octet_3,
            Err(_) => 255,
        };
    }

    if mode == 0 || mode == 1 {
        address_san = match ip_address_class {
            IPClass::A => Ipv4Addr::new(octets[0], octet_1, octet_2, octet_3),
            IPClass::B => Ipv4Addr::new(octets[0], octets[1], octet_1, octet_2),
            IPClass::C => Ipv4Addr::new(octets[0], octets[1], octets[2], octet_1),
        };
    } else if mode == 2 {
        address_san = match ip_address_class {
            IPClass::A => Ipv4Addr::new(255, octet_1, octet_2, octet_3),
            IPClass::B => Ipv4Addr::new(255, 255, octet_1, octet_2),
            IPClass::C => Ipv4Addr::new(255, 255, 255, octet_1),
        };
    } else {
        address_san = Ipv4Addr::new(255,255,255,255);
    }
    return address_san;
}
