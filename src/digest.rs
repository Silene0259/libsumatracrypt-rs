use blake3;
use blake2_rfc;
use sha2::{Sha224, Sha256, Sha384, Sha512, Digest};
use tiny_keccak::{Shake, *};
use sha1::{Sha1};
use hex::*;

use serde::{Serialize,Deserialize};

use zeroize::{Zeroize,ZeroizeOnDrop,Zeroizing};

pub struct SumatraBlake3;
pub struct SumatraBlake2b;
pub struct SumatraSha2;
pub struct SumatraSha3;
pub struct SumatraShake256;

pub struct SumatraSha1;

#[derive(Zeroize,ZeroizeOnDrop,Clone,Serialize,Deserialize)]
pub struct SumatraDigest(String);

impl SumatraDigest {
    pub fn to_string(&self) -> Zeroizing<String> {
        let zeroizer = Zeroizing::new(self.0.to_string());
        return zeroizer
    }
}

impl SumatraBlake3 {
    pub fn new<T: AsRef<[u8]>>(bytes: T) -> SumatraDigest {
        let digest = blake3::hash(bytes.as_ref());
        return SumatraDigest(hex::encode_upper(digest.as_bytes()));
    }
}

impl SumatraBlake2b {
    pub fn new<T: AsRef<[u8]>>(bytes: T, key: T, digest_size: usize) -> SumatraDigest {
        if digest_size > 0usize && digest_size <= 64usize {
            let hash = blake2_rfc::blake2b::blake2b(digest_size, key.as_ref(), bytes.as_ref());

            return SumatraDigest(hex::encode_upper(hash.as_bytes()));
        }
        else {
            panic!("Digest size is too high or too low")
        }
    }
}

impl SumatraSha1 {
    pub fn new<T: AsRef<[u8]>>(bytes: T) -> SumatraDigest {
        use sha1::Digest;
        let mut hasher = Sha1::new();

        hasher.update(bytes.as_ref());

        let output = hasher.finalize();

        return SumatraDigest(hex::encode_upper(output))
    }
}

impl SumatraSha2 {
    pub fn new<T: AsRef<[u8]>>(bytes: T) -> SumatraDigest {
        return Self::sha256(bytes)
    }
    pub fn sha224<T: AsRef<[u8]>>(bytes: T) -> SumatraDigest {
        let mut hasher = Sha224::new();
        hasher.update(bytes.as_ref());
        let digest = hasher.finalize();
        return SumatraDigest(hex::encode_upper(digest));
    }
    pub fn sha256<T: AsRef<[u8]>>(bytes: T) -> SumatraDigest {
        let mut hasher = Sha256::new();
        hasher.update(bytes.as_ref());
        let digest = hasher.finalize();
        return SumatraDigest(hex::encode_upper(digest));
    }
    pub fn sha384<T: AsRef<[u8]>>(bytes: T) -> SumatraDigest {
        let mut hasher = Sha384::new();
        hasher.update(bytes.as_ref());
        let digest = hasher.finalize();
        return SumatraDigest(hex::encode_upper(digest));
    }
    pub fn sha512<T: AsRef<[u8]>>(bytes: T) -> SumatraDigest {
        let mut hasher = Sha512::new();
        hasher.update(bytes.as_ref());
        let digest = hasher.finalize();
        return SumatraDigest(hex::encode_upper(digest));
    }
}

impl SumatraSha3 {
    pub fn sha3_224<T: AsRef<[u8]>>(data: T) -> SumatraDigest {
        let mut output = [0u8;28];

        let mut sha3 = Sha3::v224();
        sha3.update(data.as_ref());
        sha3.finalize(&mut output);

        return SumatraDigest(hex::encode_upper(output))
    }
    pub fn sha3_256<T: AsRef<[u8]>>(data: T) -> SumatraDigest {
        let mut output = [0u8;32];

        let mut sha3 = Sha3::v256();
        sha3.update(data.as_ref());
        sha3.finalize(&mut output);

        return SumatraDigest(hex::encode_upper(output))
    }
    pub fn sha3_384<T: AsRef<[u8]>>(data: T) -> SumatraDigest {
        let mut output = [0u8;48];

        let mut sha3 = Sha3::v384();
        sha3.update(data.as_ref());
        sha3.finalize(&mut output);

        return SumatraDigest(hex::encode_upper(output))
    }
    pub fn sha3_512<T: AsRef<[u8]>>(data: T) -> SumatraDigest {
        let mut output = [0u8;64];

        let mut sha3 = Sha3::v512();
        sha3.update(data.as_ref());
        sha3.finalize(&mut output);

        return SumatraDigest(hex::encode_upper(output))
    }
}

impl SumatraShake256 {
    pub fn new<T: AsRef<[u8]>>(data: T) -> SumatraDigest {
        let mut output = [0u8;64];

        let mut shake = Shake::v256();
        shake.update(data.as_ref());
        shake.finalize(&mut output);

        return SumatraDigest(hex::encode_upper(output))
    }
}