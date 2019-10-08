use faster_hex::hex_decode_fallback;
use secp256k1::{PublicKey, Signature, Message, verify};
use tiny_keccak::sha3_256;

fn main() {
    run(0);
}

#[no_mangle]
pub fn run(i: u8) -> u8 {
    let pubkey = "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f";
    let signature = "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3";
    let message = if i == 1 {
        "foobar"
    } else {
        "foobarinvalid"
    };

    let mut pubkey_bytes = [0u8; 33];
    let mut signature_bytes = [0u8; 70];

    hex_decode_fallback(&pubkey.as_bytes(), &mut pubkey_bytes);
    hex_decode_fallback(&signature.as_bytes(), &mut signature_bytes);

    let pubkey = PublicKey::parse_compressed(&pubkey_bytes).unwrap();
    let signature = Signature::parse_der(&signature_bytes).unwrap();

    let message_bytes = sha3_256(&message.as_bytes());
    let message_bytes = sha3_256(&message_bytes);
    let message = Message::parse(&message_bytes);

    if verify(&message, &signature, &pubkey) {
        return 0;
    } else {
        return 1;
    }
}
