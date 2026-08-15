//! Word banks and random-string generators for the fake exploit log / hex dump.

use macroquad::rand::gen_range;

pub const ACTIONS: &[&str] = &[
    "Bypassing", "Decrypting", "Injecting payload into", "Spoofing", "Rerouting traffic through",
    "Cracking", "Exploiting", "Scanning", "Flooding", "Hijacking session on", "Compromising",
    "Extracting data from", "Brute-forcing", "Deploying rootkit to", "Escalating privileges on",
    "Tunneling into", "Fingerprinting", "Patching", "Wiping logs on", "Cloning",
];

pub const TARGETS: &[&str] = &[
    "firewall", "mainframe", "kernel", "root partition", "encryption keys", "auth server",
    "proxy chain", "database cluster", "backdoor daemon", "session token", "SSH tunnel",
    "DNS resolver", "load balancer", "certificate authority", "VPN gateway", "packet filter",
    "user credentials table", "network switch", "biometric lock", "satellite uplink",
];

pub const SUCCESS_MESSAGES: &[&str] = &[
    "ACCESS GRANTED", "BREACH COMPLETE", "ROOT ACCESS OBTAINED", "FIREWALL DOWN",
    "SYSTEM COMPROMISED", "ENCRYPTION DEFEATED", "UPLINK SECURED",
];

const HEX_DIGITS: &[u8] = b"0123456789ABCDEF";
const MATRIX_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*<>/\\|=+-";

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

pub fn random_hex_addr(len: usize) -> String {
    let mut s = String::from("0x");
    for _ in 0..len {
        s.push(HEX_DIGITS[gen_range(0, HEX_DIGITS.len())] as char);
    }
    s
}

pub fn random_log_line() -> String {
    let action = ACTIONS[gen_range(0, ACTIONS.len())];
    let target = TARGETS[gen_range(0, TARGETS.len())];
    match gen_range(0, 3) {
        0 => format!("{action} {target} @ {}", random_ip()),
        1 => format!("{action} {target} [{}]", random_hex_addr(8)),
        _ => format!("{action} {target}..."),
    }
}

pub fn random_success_message() -> &'static str {
    SUCCESS_MESSAGES[gen_range(0, SUCCESS_MESSAGES.len())]
}

pub fn random_target() -> &'static str {
    TARGETS[gen_range(0, TARGETS.len())]
}
