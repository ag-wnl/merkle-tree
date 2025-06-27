use blake3::Hasher as B3Hasher;

pub struct Hasher;

impl Hasher {
    pub fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasherr = B3Hasher::new();
        hasherr.update(data);
        
        *hasherr.finalize().as_bytes()
    }

    pub fn combine(left: &[u8], right: &[u8]) -> [u8; 32] {
        let mut hasherr = B3Hasher::new();
        hasherr.update(left);
        hasherr.update(right);

        *hasherr.finalize().as_bytes()
    }
}