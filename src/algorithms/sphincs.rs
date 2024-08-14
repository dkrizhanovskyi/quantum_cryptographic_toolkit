pub struct Sphincs {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl Sphincs {
    pub fn new() -> Self {
        Sphincs {
            private_key: vec![0; 64],
            public_key: vec![0; 32],
        }
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let mut result: Vec<u8> = message.to_vec();
        for _ in 0..10000 {
            result.iter_mut().for_each(|x| *x = x.wrapping_add(1));
        }
        result
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        // Reapply the signing process to see if we get the same signature
        let expected_signature = self.sign(message);
        expected_signature == signature
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphincs_sign_verify() {
        let sphincs = Sphincs::new();
        let message = vec![1, 2, 3, 4];
        let signature = sphincs.sign(&message);
        let is_valid = sphincs.verify(&message, &signature);
        assert!(is_valid);
    }
}
