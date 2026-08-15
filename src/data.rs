//! Word banks and random-line generators for the fake exploit log, hex dump,
//! and progress bar.

use macroquad::rand::gen_range;

const TOOL_COMMANDS: &[&str] = &[
    "nmap -sS -p- {ip}",
    "nmap -A -T4 {ip}",
    "hydra -l root -P rockyou.txt ssh://{ip}",
    "sqlmap -u http://{ip}/login.php --dump",
    "msfconsole -q -x \"use exploit/multi/handler; set RHOST {ip}; run\"",
    "ssh root@{ip}",
    "scp payload.elf root@{ip}:/tmp/",
    "openssl enc -aes-256-cbc -d -in {file}",
    "john --wordlist=rockyou.txt {file}",
    "tcpdump -i eth0 host {ip}",
    "curl -s -X POST http://{ip}/api/auth -d @creds.json",
    "./exploit.py --target {ip} --port {port}",
    "aircrack-ng -w wordlist.cap capture-01.cap",
    "nc -nvlp {port}",
    "gpg --decrypt {file}",
    "chmod +x payload.elf && ./payload.elf",
    "iptables -F && iptables -P INPUT ACCEPT",
    "python3 -m http.server {port}",
];

const INFO_MESSAGES: &[&str] = &[
    "Host {ip} is up (0.0{n}s latency)",
    "{port}/tcp open  ssh",
    "{port}/tcp open  http",
    "Resolving DNS for target subnet...",
    "Enumerating shares on {ip}",
    "Fingerprinting OS: Linux 5.{n}.x",
    "Establishing reverse shell...",
    "Uploading payload ({n}84 KB)",
    "Session {n} opened",
    "Dumping table users ({n}032 rows)",
    "Bruteforce attempt {n}/14344",
    "Routing traffic through {n} relays",
];

const SUCCESS_MESSAGES: &[&str] = &[
    "Login successful: root:********",
    "Shell obtained on {ip}",
    "Password cracked: ********",
    "Privilege escalation successful (root)",
    "Exfiltration complete ({n}2 MB)",
    "Backdoor installed",
    "Encryption key recovered",
];

const ERROR_MESSAGES: &[&str] = &[
    "Connection timed out",
    "Permission denied (publickey)",
    "Authentication failed, retrying...",
    "Packet loss detected, rerouting",
    "Rate limited by {ip}, backing off",
];

const BANNER_MESSAGES: &[&str] = &[
    "ACCESS GRANTED",
    "ROOT ACCESS OBTAINED",
    "BREACH COMPLETE",
    "FIREWALL DOWN",
    "ENCRYPTION DEFEATED",
    "UPLINK SECURED",
];

const TARGETS: &[&str] = &[
    "firewall", "mainframe", "kernel", "root partition", "encryption keys", "auth server",
    "proxy chain", "database cluster", "backdoor daemon", "session token", "SSH tunnel",
    "DNS resolver", "load balancer", "certificate authority", "VPN gateway", "packet filter",
    "credentials table", "core switch", "biometric lock", "satellite uplink",
];

const HOSTNAMES: &[&str] = &[
    "shadow-relay", "ghost-ops", "dark-fiber", "void-proxy", "black-ice", "null-gate",
    "cipher-node", "phantom-vpn", "obsidian-hub", "wraith-tap",
];

const HEX_DIGITS: &[u8] = b"0123456789ABCDEF";
const MATRIX_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*<>/\\|=+-";

pub enum LineKind {
    Command,
    Info,
    Success,
    Error,
}

pub fn random_matrix_char() -> char {
    let idx = gen_range(0, MATRIX_CHARS.len());
    MATRIX_CHARS.chars().nth(idx).unwrap()
}

pub fn random_hex_byte() -> String {
    let hi = HEX_DIGITS[gen_range(0, HEX_DIGITS.len())] as char;
    let lo = HEX_DIGITS[gen_range(0, HEX_DIGITS.len())] as char;
    format!("{hi}{lo}")
}

pub fn random_ip() -> String {
    format!(
        "{}.{}.{}.{}",
        gen_range(1, 255),
        gen_range(0, 255),
        gen_range(0, 255),
        gen_range(0, 255)
    )
}

pub fn random_hostname() -> String {
    format!(
        "{}-{}",
        HOSTNAMES[gen_range(0, HOSTNAMES.len())],
        gen_range(1, 99)
    )
}

fn fill_template(template: &str) -> String {
    template
        .replace("{ip}", &random_ip())
        .replace("{port}", &gen_range(20, 65000).to_string())
        .replace("{file}", &format!("dump_{:04}.bin", gen_range(0, 9999)))
        .replace("{n}", &gen_range(0, 9).to_string())
}

/// Returns the next fake log entry as (kind, rendered text). Command lines
/// are the bare command (the caller prepends a shell prompt); the other
/// kinds already carry their own `[*]`/`[+]`/`[!]` tag.
pub fn random_log_entry() -> (LineKind, String) {
    match gen_range(0, 10) {
        0..=3 => (
            LineKind::Command,
            fill_template(TOOL_COMMANDS[gen_range(0, TOOL_COMMANDS.len())]),
        ),
        4..=6 => (
            LineKind::Info,
            format!(
                "[*] {}",
                fill_template(INFO_MESSAGES[gen_range(0, INFO_MESSAGES.len())])
            ),
        ),
        7..=8 => (
            LineKind::Success,
            format!(
                "[+] {}",
                fill_template(SUCCESS_MESSAGES[gen_range(0, SUCCESS_MESSAGES.len())])
            ),
        ),
        _ => (
            LineKind::Error,
            format!(
                "[!] {}",
                fill_template(ERROR_MESSAGES[gen_range(0, ERROR_MESSAGES.len())])
            ),
        ),
    }
}

pub fn random_banner_message() -> &'static str {
    BANNER_MESSAGES[gen_range(0, BANNER_MESSAGES.len())]
}

pub fn random_target() -> &'static str {
    TARGETS[gen_range(0, TARGETS.len())]
}
