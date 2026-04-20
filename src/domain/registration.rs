pub fn country_from_registration(reg: &str) -> Option<&'static str> {
    let reg = reg.trim();

    if is_bundeswehr(reg) {
        return Some("Germany (Military)");
    }

    // ── Military serial formats ──────────────────────────────────────────────
    // These typically don't follow civil prefix rules at all.

    // Germany – Bundeswehr: two digits + '+' + two digits  e.g. "54+01", "98+35"

    // Italy – Aeronautica Militare: "MM" + digits  e.g. "MM62293"
    if reg.starts_with("MM") && reg[2..].chars().all(|c| c.is_ascii_digit()) {
        return Some("Italy (Military)");
    }

    // France – Armée de l'Air: "F-R" + letters or pure digits with no dash
    // French military often use callsigns like "F-RAAF", "101", "FAF01" etc.
    // The most reliable pattern in ADS-B data is "F-R" prefix.
    if reg.starts_with("F-R") {
        return Some("France (Military)");
    }

    // United Kingdom – RAF / RN: "ZA"-"ZZ" + digits/letters  e.g. "ZK034", "ZB500"
    if reg.len() >= 2 {
        let first_two = &reg[..2];
        if first_two >= "ZA" && first_two <= "ZZ" && reg.chars().next() == Some('Z') {
            return Some("United Kingdom (Military)");
        }
    }

    // USA – USAF/USN/USMC: pure digits (serial like "12-3456") or "AF" + digits
    if reg.starts_with("AF") && reg[2..].chars().all(|c| c.is_ascii_digit() || c == '-') {
        return Some("United States (Military)");
    }

    // Netherlands – KLu: two letters starting with 'Q' or specific serials like "D-yyy"
    // Dutch military use serials like "Q-17", "F-362", "R-01"
    if matches!(&reg[..1], "Q") && reg.contains('-') {
        return Some("Netherlands (Military)");
    }

    // Belgium – Belgian Air Component: "FA-xx", "AT-xx", "CM-xx", "RS-xx"
    if matches!(
        reg.get(..2),
        Some("FA") | Some("AT") | Some("CM") | Some("RS") | Some("BN")
    ) && reg.contains('-')
    {
        return Some("Belgium (Military)");
    }

    // Spain – Ejército del Aire: "T.xx", "E.xx", "U.xx", "HE.xx"  (dot-separated)
    if reg.contains('.') && reg.len() <= 8 {
        let prefix = reg.split('.').next().unwrap_or("");
        if matches!(prefix, "T" | "E" | "U" | "HE" | "AN" | "TR" | "TM" | "P") {
            return Some("Spain (Military)");
        }
    }

    // Sweden – Swedish Air Force: "10xxx", "SE-" civil handled below
    // Serials like "10036", "10271"
    if reg.len() == 5 && reg.starts_with("10") && reg.chars().all(|c| c.is_ascii_digit()) {
        return Some("Sweden (Military)");
    }

    // Norway – RNoAF: "041", "060" – 3-digit serials with leading zeros
    // Too ambiguous on its own; handled via civil prefix fallback.

    // Greece – Hellenic Air Force: e.g. "743", often just numbers
    // Poland – Polish Air Force: e.g. "012", "4013"

    // ── Civil ICAO prefix lookup ─────────────────────────────────────────────
    // Longest-prefix-first matching for multi-character prefixes.
    None
}
fn is_bundeswehr(reg: &str) -> bool {
    let bytes = reg.as_bytes();
    bytes.len() == 5
        && bytes[0].is_ascii_digit()
        && bytes[1].is_ascii_digit()
        && bytes[2] == b'+'
        && bytes[3].is_ascii_digit()
        && bytes[4].is_ascii_digit()
}
