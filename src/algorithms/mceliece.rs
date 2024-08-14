pub struct McEliece {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

impl McEliece {
    pub fn new() -> Self {
        McEliece {
            public_key: vec![0; 128],
            private_key: vec![0; 128],
        }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let mut result: Vec<u8> = plaintext.to_vec();
        for _ in 0..10000 {
            result.iter_mut().for_each(|x| *x = x.wrapping_add(1));
        }
        result
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        let mut result: Vec<u8> = ciphertext.to_vec();
        for _ in 0..10000 {
            result.iter_mut().for_each(|x| *x = x.wrapping_sub(1));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mceliece_encrypt_decrypt() {
        let mceliece = McEliece::new();
        let plaintext = vec![1, 2, 3, 4];
        let ciphertext = mceliece.encrypt(&plaintext);
        let decrypted_text = mceliece.decrypt(&ciphertext);
        assert_eq!(decrypted_text, plaintext);
    }
}
