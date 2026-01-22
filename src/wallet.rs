use ed25519_dalek::{
    SigningKey, VerifyingKey, Signature, Signer, Verifier, SECRET_KEY_LENGTH,
};
use rand::rngs::OsRng;
use rand::RngCore;
use hex;

#[derive(Debug, Clone)]
pub struct Wallet {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl Wallet {
    /// Create a new wallet with random keypair
    pub fn new() -> Self {
        let mut csprng = OsRng;

        // Generate random secret key bytes
        let mut secret_bytes = [0u8; SECRET_KEY_LENGTH];
        csprng.fill_bytes(&mut secret_bytes);

        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let verifying_key = signing_key.verifying_key();

        Wallet {
            signing_key,
            verifying_key,
        }
    }

    /// Public address = hex-encoded public key
    pub fn address(&self) -> String {
        hex::encode(self.verifying_key.as_bytes())
    }

    /// Sign arbitrary message bytes
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    /// Verify a signature using a public key (hex)
    pub fn verify(
        public_key_hex: &str,
        message: &[u8],
        signature: &Signature,
    ) -> bool {
        let public_key_vec = match hex::decode(public_key_hex) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        // Convert Vec<u8> → [u8; 32]
        let public_key_bytes: [u8; 32] = match public_key_vec.try_into() {
            Ok(arr) => arr,
            Err(_) => return false,
        };

        let verifying_key = match VerifyingKey::from_bytes(&public_key_bytes) {
            Ok(key) => key,
            Err(_) => return false,
        };

        verifying_key.verify(message, signature).is_ok()
    }
}
