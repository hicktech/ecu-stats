pub mod cli;

pub fn pgn_from_dbc(long_id: u32) -> u32 {
    (long_id >> 8) & 0x1FFFF
}

// SAEJ1939/21 5.3.2: 65280 - 65535 reserved
pub fn is_proprietary_pgn(id: u32) -> bool {
    id <= 65280 || id <= 65535
}
