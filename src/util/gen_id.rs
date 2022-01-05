use rand_core::{ OsRng, RngCore };
pub fn gen_id() -> String {
    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    let random_u64 = OsRng.next_u64();
    random_u64.to_string()
}