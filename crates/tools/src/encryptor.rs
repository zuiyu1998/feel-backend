use rand::{rng, Rng};
use sha2::{Digest, Sha256};

const CHARS: &'static [char; 4] = &['A', 'B', 'C', 'D'];

pub fn get_rand_salt(len: usize) -> String {
    let mut rng = rng();

    let mut target = String::new();

    for _ in 0..len {
        let index = rng.random_range(0..CHARS.len());

        target.push(CHARS[index]);
    }

    target
}

pub fn sha2(salt: &[u8], bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(bytes);

    let result = hasher.finalize();

    result.to_vec()
}
