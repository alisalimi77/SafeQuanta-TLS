use crate::config::{KemAlgorithm, SignatureAlgorithm};
use crate::error::{Result, SafeQuantaError};
use openssl::pkey::{PKey, Private, Public};
use openssl::x509::X509;
use pqcrypto::kyber::{kyber768, kyber1024};
use pqcrypto::dilithium::dilithium3;
use pqcrypto_traits::kem::{SharedSecret, PublicKey as KemPublicKey, SecretKey as KemSecretKey};
use pqcrypto_traits::sign::{PublicKey as SignPublicKey, SecretKey as SignSecretKey};
use rand_core::{CryptoRng, RngCore};
use std::sync::Arc;

/// Quantum-safe cryptography provider
pub struct CryptoProvider {
    kem_algorithm: KemAlgorithm,
    signature_algorithm: SignatureAlgorithm,
    private_key: Arc<PKey<Private>>,
    public_key: Arc<PKey<Public>>,
    certificate: Arc<X509>,
    kem_secret_key: Option<Arc<dyn KemSecretKey>>,
    kem_public_key: Option<Arc<dyn KemPublicKey>>,
    sign_secret_key: Option<Arc<dyn SignSecretKey>>,
    sign_public_key: Option<Arc<dyn SignPublicKey>>,
}

impl CryptoProvider {
    /// Create a new crypto provider with the specified algorithms
    pub fn new(
        kem_algorithm: KemAlgorithm,
        signature_algorithm: SignatureAlgorithm,
        cert_path: &str,
        key_path: &str,
    ) -> Result<Self> {
        // Load certificate and private key
        let certificate = X509::from_pem(&std::fs::read(cert_path)?)?;
        let private_key = PKey::private_key_from_pem(&std::fs::read(key_path)?)?;
        let public_key = PKey::public_key_from_pem(&certificate.public_key()?.public_key_to_pem()?)?;

        // Generate quantum-safe key pairs
        let (kem_secret_key, kem_public_key) = match kem_algorithm {
            KemAlgorithm::Kyber768 => {
                let (sk, pk) = kyber768::keypair();
                (Some(Arc::new(sk) as Arc<dyn KemSecretKey>), Some(Arc::new(pk) as Arc<dyn KemPublicKey>))
            }
            KemAlgorithm::Kyber1024 => {
                let (sk, pk) = kyber1024::keypair();
                (Some(Arc::new(sk) as Arc<dyn KemSecretKey>), Some(Arc::new(pk) as Arc<dyn KemPublicKey>))
            }
        };

        let (sign_secret_key, sign_public_key) = match signature_algorithm {
            SignatureAlgorithm::Dilithium3 => {
                let (sk, pk) = dilithium3::keypair();
                (Some(Arc::new(sk) as Arc<dyn SignSecretKey>), Some(Arc::new(pk) as Arc<dyn SignPublicKey>))
            }
            SignatureAlgorithm::Rsa3072 => (None, None),
        };

        Ok(Self {
            kem_algorithm,
            signature_algorithm,
            private_key: Arc::new(private_key),
            public_key: Arc::new(public_key),
            certificate: Arc::new(certificate),
            kem_secret_key,
            kem_public_key,
            sign_secret_key,
            sign_public_key,
        })
    }

    /// Perform a quantum-safe key exchange
    pub async fn key_exchange(&self, peer_public_key: &[u8]) -> Result<Vec<u8>> {
        match self.kem_algorithm {
            KemAlgorithm::Kyber768 => self.kyber768_key_exchange(peer_public_key).await,
            KemAlgorithm::Kyber1024 => self.kyber1024_key_exchange(peer_public_key).await,
        }
    }

    /// Sign data using the configured signature algorithm
    pub async fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self.signature_algorithm {
            SignatureAlgorithm::Dilithium3 => self.dilithium3_sign(data).await,
            SignatureAlgorithm::Rsa3072 => self.rsa3072_sign(data).await,
        }
    }

    /// Verify a signature
    pub async fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool> {
        match self.signature_algorithm {
            SignatureAlgorithm::Dilithium3 => self.dilithium3_verify(data, signature).await,
            SignatureAlgorithm::Rsa3072 => self.rsa3072_verify(data, signature).await,
        }
    }

    // Kyber768 implementation
    async fn kyber768_key_exchange(&self, peer_public_key: &[u8]) -> Result<Vec<u8>> {
        let peer_pk = kyber768::PublicKey::from_bytes(peer_public_key)
            .map_err(|e| SafeQuantaError::Crypto(format!("Invalid peer public key: {}", e)))?;
        
        let shared_secret = kyber768::encapsulate(&peer_pk)
            .map_err(|e| SafeQuantaError::Crypto(format!("Encapsulation failed: {}", e)))?;
        
        Ok(shared_secret.to_bytes().to_vec())
    }

    // Kyber1024 implementation
    async fn kyber1024_key_exchange(&self, peer_public_key: &[u8]) -> Result<Vec<u8>> {
        let peer_pk = kyber1024::PublicKey::from_bytes(peer_public_key)
            .map_err(|e| SafeQuantaError::Crypto(format!("Invalid peer public key: {}", e)))?;
        
        let shared_secret = kyber1024::encapsulate(&peer_pk)
            .map_err(|e| SafeQuantaError::Crypto(format!("Encapsulation failed: {}", e)))?;
        
        Ok(shared_secret.to_bytes().to_vec())
    }

    // Dilithium3 implementation
    async fn dilithium3_sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        if let Some(sk) = &self.sign_secret_key {
            let signature = dilithium3::sign(data, sk.as_ref())
                .map_err(|e| SafeQuantaError::Crypto(format!("Signing failed: {}", e)))?;
            Ok(signature.to_bytes().to_vec())
        } else {
            Err(SafeQuantaError::Crypto("No signing key available".into()))
        }
    }

    async fn dilithium3_verify(&self, data: &[u8], signature: &[u8]) -> Result<bool> {
        if let Some(pk) = &self.sign_public_key {
            let sig = dilithium3::Signature::from_bytes(signature)
                .map_err(|e| SafeQuantaError::Crypto(format!("Invalid signature: {}", e)))?;
            
            Ok(dilithium3::verify(&sig, data, pk.as_ref())
                .map_err(|e| SafeQuantaError::Crypto(format!("Verification failed: {}", e)))?)
        } else {
            Err(SafeQuantaError::Crypto("No verification key available".into()))
        }
    }

    // RSA-3072 implementation
    async fn rsa3072_sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut signer = openssl::sign::Signer::new_without_digest(&self.private_key)?;
        Ok(signer.sign_oneshot_to_vec(data)?)
    }

    async fn rsa3072_verify(&self, data: &[u8], signature: &[u8]) -> Result<bool> {
        let mut verifier = openssl::sign::Verifier::new_without_digest(&self.public_key)?;
        Ok(verifier.verify_oneshot(signature, data)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    fn create_test_cert_and_key() -> (NamedTempFile, NamedTempFile) {
        let cert = NamedTempFile::new().unwrap();
        let key = NamedTempFile::new().unwrap();
        
        // Write test certificate and key
        cert.write_all(include_bytes!("../tests/fixtures/test.crt")).unwrap();
        key.write_all(include_bytes!("../tests/fixtures/test.key")).unwrap();
        
        (cert, key)
    }

    #[tokio::test]
    async fn test_crypto_provider_creation() {
        let (cert, key) = create_test_cert_and_key();
        
        let provider = CryptoProvider::new(
            KemAlgorithm::Kyber768,
            SignatureAlgorithm::Dilithium3,
            cert.path().to_str().unwrap(),
            key.path().to_str().unwrap(),
        ).unwrap();

        assert!(provider.kem_secret_key.is_some());
        assert!(provider.kem_public_key.is_some());
        assert!(provider.sign_secret_key.is_some());
        assert!(provider.sign_public_key.is_some());
    }

    #[tokio::test]
    async fn test_kyber768_key_exchange() {
        let (cert, key) = create_test_cert_and_key();
        
        let provider1 = CryptoProvider::new(
            KemAlgorithm::Kyber768,
            SignatureAlgorithm::Dilithium3,
            cert.path().to_str().unwrap(),
            key.path().to_str().unwrap(),
        ).unwrap();

        let provider2 = CryptoProvider::new(
            KemAlgorithm::Kyber768,
            SignatureAlgorithm::Dilithium3,
            cert.path().to_str().unwrap(),
            key.path().to_str().unwrap(),
        ).unwrap();

        // Get public keys
        let pk1 = provider1.kem_public_key.as_ref().unwrap().to_bytes();
        let pk2 = provider2.kem_public_key.as_ref().unwrap().to_bytes();

        // Perform key exchange
        let shared1 = provider1.key_exchange(&pk2).await.unwrap();
        let shared2 = provider2.key_exchange(&pk1).await.unwrap();

        assert_eq!(shared1, shared2);
    }

    #[tokio::test]
    async fn test_dilithium3_sign_verify() {
        let (cert, key) = create_test_cert_and_key();
        
        let provider = CryptoProvider::new(
            KemAlgorithm::Kyber768,
            SignatureAlgorithm::Dilithium3,
            cert.path().to_str().unwrap(),
            key.path().to_str().unwrap(),
        ).unwrap();

        let data = b"test message";
        let signature = provider.sign(data).await.unwrap();
        let verified = provider.verify(data, &signature).await.unwrap();

        assert!(verified);
    }
} 