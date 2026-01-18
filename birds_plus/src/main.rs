type Bits256 = [u8;32];
fn chash(input: &[u8]) -> Bits256 {
}




struct WinternitzPublicKey(Bits256)
struct WinternitzSecretKey(Bits256)

struct WinternitzSignature{
    signed_hash : Bits256,
    signature_data: [Bits256;32],
}




fn main() {
    println!("Hello, world!");
}
