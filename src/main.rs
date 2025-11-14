use std::fmt;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use colored::*;
use dhcproto::{
    v4::{DhcpOption, Message, MessageType, Opcode},
    Decodable, Decoder, Encodable, Encoder,
};
use indicatif::{ProgressBar, ProgressStyle};
use rpassword::read_password;
use tokio::net::UdpSocket;

// --- Data Structures ---

#[derive(Clone, Copy, PartialEq, Eq)]
struct MacAddr([u8; 6]);

impl MacAddr {
    fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        MacAddr([a, b, c, d, e, f])
    }
}

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl Default for MacAddr {
    fn default() -> Self {
        MacAddr([0; 6])
    }
}

#[derive(Clone)]
struct NetworkInterface {
    name: String,
    ip: Ipv4Addr,
}

/// Holds the result from the DHCP server (the IP and MAC of the iDRAC).
#[derive(Clone, Default)]
struct DhcpLease {
    mac: Option<MacAddr>,
    ip: Option<Ipv4Addr>,
    is_leased: bool,
}

#[tokio::main]
async fn main() {
    println!("{}", "Dell iDRAC Log Fetcher".bold().green());
    println!("--------------------------------------");
    println!("This tool will automate connecting to an iDRAC in DHCP mode and collecting logs.");

    // Step 1: Select the network interface.
    let interface = match select_interface() {
        Some(iface) => iface,
        None => {
            eprintln!(
                "{}",
                "No suitable network interface found or selected. Exiting.".red()
            );
            return;
        }
    };

    let server_ipv4 = interface.ip;

    // Step 2: Start the DHCP server.
    println!(
        "\n[{}] Starting DHCP server on interface {} ({})",
        "STEP 2".yellow(),
        interface.name,
        server_ipv4
    );
    let lease_ip = get_lease_ip(server_ipv4);
    let lease_info = Arc::new(Mutex::new(DhcpLease::default()));
    
    let shutdown_signal = Arc::new(tokio::sync::Notify::new());
    let server_shutdown_signal = shutdown_signal.clone();
    let server_lease_info = lease_info.clone();

    let server_handle = tokio::spawn(async move {
        if let Err(e) =
            run_dhcp_server(server_ipv4, lease_ip, server_lease_info, server_shutdown_signal).await
        {
            eprintln!("\n{} {}", "DHCP Server Error:".red(), e);
        }
    });
    
    println!(
        "Waiting for the iDRAC to request an IP address. Please ensure the server is powered on."
    );

    // Step 3: Wait for the iDRAC to get an IP.
    let idrac_ip = match wait_for_lease(lease_info).await {
        Some(ip) => ip,
        None => {
            eprintln!(
                "\n{}",
                "Timeout: iDRAC did not request an IP within 2 minutes. Exiting.".red()
            );
            eprintln!("Please check the physical connection and ensure the iDRAC is set to DHCP.");
            shutdown_signal.notify_one();
            let _ = server_handle.await;
            return;
        }
    };
    println!(
        "\n[{}] iDRAC has been assigned IP address: {}",
        "SUCCESS".green(),
        idrac_ip.to_string().bold()
    );
    
    // Stop the DHCP server.
    shutdown_signal.notify_one();
    let _ = server_handle.await;
    println!("[{}] DHCP server has been stopped.", "INFO".cyan());

    // Step 4: Get credentials from the user.
    println!("\n[{}] Please enter iDRAC credentials.", "STEP 3".yellow());
    let (username, password) = get_credentials();

    // Step 5: Run racadm commands.
    println!("\n[{}] Running initial racadm commands...", "STEP 4".yellow());
    let idrac_ip_str = idrac_ip.to_string();
    
    if !run_racadm_command(&idrac_ip_str, &username, &password, &["getsysinfo"]) {
        eprintln!(
            "{}",
            "Failed to run getsysinfo. Please check credentials and network connectivity. Exiting."
                .red()
        );
        return;
    }
    run_racadm_command(&idrac_ip_str, &username, &password, &["getractime"]);
    run_racadm_command(
        &idrac_ip_str,
        &username,
        &password,
        &["getconfig", "-g", "cfgSerial"],
    );

    // Step 6: Collect TSR log.
    println!(
        "\n[{}] Starting TSR (SupportAssist) log collection...",
        "STEP 5".yellow()
    );
    if collect_tsr_log(&idrac_ip_str, &username, &password) {
        println!("\n{}", "Process completed successfully!".bold().green());
    } else {
        eprintln!("\n{}", "Process finished with errors.".bold().red());
    }

    println!("\nPress Enter to exit...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

async fn run_dhcp_server(
    server_ip: Ipv4Addr,
    lease_ip: Ipv4Addr,
    lease_info: Arc<Mutex<DhcpLease>>,
    shutdown_signal: Arc<tokio::sync::Notify>,
) -> io::Result<()> {
    let socket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 67)).await?;
    socket.set_broadcast(true)?;

    let mut buf = vec![0u8; 1500];

    loop {
        tokio::select! {
            Ok((len, _)) = socket.recv_from(&mut buf) => {
                let mut decoder = Decoder::new(&buf[..len]);
                if let Ok(msg) = Message::decode(&mut decoder) {
                    if let Some(MessageType::Discover) = msg.opts().msg_type() {
                        let chaddr = msg.chaddr();
                        let client_mac = MacAddr::new(
                            chaddr[0], chaddr[1], chaddr[2], chaddr[3], chaddr[4], chaddr[5],
                        );
                        println!(
                            "\n[{}] DHCP DISCOVER received from MAC: {}",
                            "INFO".cyan(),
                            client_mac
                        );
                        println!(
                            "[{}] Offering IP address {} to the iDRAC.",
                            "INFO".cyan(),
                            lease_ip
                        );
                        
                        let mut offer = Message::default();
                        offer.set_opcode(Opcode::BootReply);
                        offer.set_xid(msg.xid());
                        offer.set_secs(msg.secs());
                        offer.set_flags(msg.flags());
                        offer.set_yiaddr(lease_ip);
                        offer.set_chaddr(msg.chaddr());
                        offer.opts_mut().insert(DhcpOption::MessageType(MessageType::Offer));
                        offer.opts_mut().insert(DhcpOption::AddressLeaseTime(3600));
                        offer.opts_mut().insert(DhcpOption::ServerIdentifier(server_ip));
                        
                        let mut offer_buf = Vec::new();
                        let mut encoder = Encoder::new(&mut offer_buf);
                        offer.encode(&mut encoder).unwrap();
                        socket.send_to(&offer_buf, "255.255.255.255:68").await?;

                    } else if let Some(MessageType::Request) = msg.opts().msg_type() {
                        println!(
                            "[{}] DHCP REQUEST received. Acknowledging lease.",
                            "INFO".cyan()
                        );

                        {
                            let mut lease = lease_info.lock().unwrap();
                            if !lease.is_leased {
                                let chaddr = msg.chaddr();
                                let client_mac = MacAddr::new(
                                    chaddr[0], chaddr[1], chaddr[2], chaddr[3], chaddr[4], chaddr[5],
                                );
                                lease.mac = Some(client_mac);
                                lease.ip = Some(lease_ip);
                                lease.is_leased = true;
                            }
                        }

                        let mut ack = Message::default();
                        ack.set_opcode(Opcode::BootReply);
                        ack.set_xid(msg.xid());
                        ack.set_secs(msg.secs());
                        ack.set_flags(msg.flags());
                        ack.set_yiaddr(lease_ip);
                        ack.set_chaddr(msg.chaddr());
                        ack.opts_mut().insert(DhcpOption::MessageType(MessageType::Ack));
                        ack.opts_mut().insert(DhcpOption::AddressLeaseTime(3600));
                        ack.opts_mut().insert(DhcpOption::ServerIdentifier(server_ip));

                        let mut ack_buf = Vec::new();
                        let mut encoder = Encoder::new(&mut ack_buf);
                        ack.encode(&mut encoder).unwrap();
                        socket.send_to(&ack_buf, "255.255.255.255:68").await?;
                    }
                }
            },
            _ = shutdown_signal.notified() => {
                break;
            }
        }
    }
    Ok(())
}

fn select_interface() -> Option<NetworkInterface> {
    println!(
        "\n[{}] Select the network interface connected to the iDRAC:",
        "STEP 1".yellow()
    );

    let interfaces = match if_addrs::get_if_addrs() {
        Ok(addrs) => addrs,
        Err(e) => {
            eprintln!("Failed to get network interfaces: {}", e);
            return None;
        }
    };

    let valid_interfaces: Vec<NetworkInterface> = interfaces
        .into_iter()
        .filter_map(|iface| {
            if !iface.is_loopback() {
                if let IpAddr::V4(ipv4) = iface.addr.ip() {
                    return Some(NetworkInterface {
                        name: iface.name,
                        ip: ipv4,
                    });
                }
            }
            None
        })
        .collect();

    if valid_interfaces.is_empty() {
        return None;
    }

    for (i, iface) in valid_interfaces.iter().enumerate() {
        println!("  [{}] {} - {}", i + 1, iface.name.bold(), iface.ip);
    }

    loop {
        print!("Enter number: ");
        io::stdout().flush().ok()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;

        match input.trim().parse::<usize>() {
            Ok(n) if n > 0 && n <= valid_interfaces.len() => {
                return Some(valid_interfaces[n - 1].clone());
            }
            _ => eprintln!("{}", "Invalid selection. Please try again.".red()),
        }
    }
}

fn get_lease_ip(server_ip: Ipv4Addr) -> Ipv4Addr {
    let mut octets = server_ip.octets();
    octets[3] = 200;
    Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])
}

async fn wait_for_lease(lease_info: Arc<Mutex<DhcpLease>>) -> Option<Ipv4Addr> {
    let start_time = Instant::now();
    let timeout = Duration::from_secs(120);
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Waiting for iDRAC DHCP request...");

    while start_time.elapsed() < timeout {
        pb.tick();
        {
            let lease = lease_info.lock().unwrap();
            if lease.is_leased {
                pb.finish_with_message("iDRAC IP acquired!");
                return lease.ip;
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    pb.finish_with_message("Timeout reached.");
    None
}

fn get_credentials() -> (String, String) {
    let mut username = String::new();
    print!("Enter iDRAC username [default: root]: ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut username);
    let username = username.trim();
    let final_username = if username.is_empty() {
        "root".to_string()
    } else {
        username.to_string()
    };

    print!("Enter iDRAC password [default: Password@_]: ");
    let _ = io::stdout().flush();
    let password = read_password().unwrap_or_default();
    let final_password = if password.is_empty() {
        "Password@_".to_string()
    } else {
        password
    };

    (final_username, final_password)
}

fn run_racadm_command(ip: &str, user: &str, pass: &str, args: &[&str]) -> bool {
    println!("\n> {} {}", "Running command:".dimmed(), args.join(" "));
    
    let mut command = Command::new("racadm");
    command
        .arg("-r")
        .arg(ip)
        .arg("-u")
        .arg(user)
        .arg("-p")
        .arg(pass)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    match command.status() {
        Ok(status) => {
            if !status.success() {
                eprintln!(
                    "\n{}",
                    format!("Command failed with exit code: {}", status).red()
                );
            }
            status.success()
        }
        Err(e) => {
            eprintln!(
                "\n{}",
                "Failed to execute 'racadm'. Please ensure it is installed and in your system's PATH."
                    .red()
            );
            eprintln!("  Error: {}", e);
            false
        }
    }
}

fn collect_tsr_log(ip: &str, user: &str, pass: &str) -> bool {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("TSR_{}_{}.zip", ip.replace('.', "-"), timestamp);

    println!("\nStep 1 of 2: Triggering SupportAssist Collection job on the iDRAC.");
    println!("This can take several minutes. Please be patient...");

    if !run_racadm_command(
        ip,
        user,
        pass,
        &["supportassist", "collect", "-t", "SysInfo,TTYLOG"],
    ) {
        eprintln!("{}", "Failed to start the SupportAssist job.".red());
        return false;
    }

    println!("\nStep 2 of 2: Waiting for the job to complete before downloading...");
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message("Polling job queue...");

    loop {
        pb.tick();

        let output = Command::new("racadm")
            .arg("-r")
            .arg(ip)
            .arg("-u")
            .arg(user)
            .arg("-p")
            .arg(pass)
            .args(&["jobqueue", "view"])
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                if stdout.contains("SupportAssist Collection") && stdout.contains("Completed") {
                    pb.finish_with_message("Job completed!");
                    break;
                } else if !stdout.contains("SupportAssist Collection") && !stdout.contains("JID_") {
                    pb.finish_with_message("Job queue is empty, assuming completion.");
                    break;
                }
            }
            Err(_) => {
                pb.finish_with_message("Failed to query job queue.");
                eprintln!(
                    "{}",
                    "Could not check job status. Proceeding to download anyway...".yellow()
                );
                break;
            }
        }
        thread::sleep(Duration::from_secs(10));
    }

    println!(
        "\nAttempting to download the SupportAssist report to '{}'...",
        filename.bold()
    );
    run_racadm_command(
        ip,
        user,
        pass,
        &["supportassist", "exportlastcollection", "-f", &filename],
    )
}