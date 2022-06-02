pub mod model;
pub use base64ct::{Base64, Encoding};
pub use sha2::{Digest, Sha256};

#[macro_export]
macro_rules! sha256_hash {
    ($($x:expr),+) => {
        {
            use $crate::Digest;
            use $crate::Encoding;
            let mut hasher = $crate::Sha256::new();
            $(
                hasher.update($x);
            )+
            let hash = hasher.finalize();
            $crate::Base64::encode_string(&hash)
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_sha256_hash() {
        let hash = sha256_hash!("bla", "blub");
        assert_eq!(&hash, "Yaqea3U7/ChYyHM6JBIUc5SsZcimXZ3IxFxgK5i6NSc=");
    }
}
