use hex;
use k256::ecdsa::VerifyingKey;

/// Converts a VerifyingKey to a hex string with "0x" prefix
pub fn to_hex_with_prefix(key: &VerifyingKey) -> String {
    let bytes = key.to_encoded_point(true).to_bytes().to_vec();
    format!("0x{}", hex::encode(bytes))
}

/// Removes the "0x" prefix from a hex string and decodes it to bytes
pub fn from_hex_with_prefix(hex_str: &str) -> Result<Vec<u8>, hex::FromHexError> {
    if hex_str.starts_with("0x") {
        hex::decode(&hex_str[2..])
    } else {
        hex::decode(hex_str)
    }
}
