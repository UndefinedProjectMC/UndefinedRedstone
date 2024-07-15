use std::io;
use base64::Engine;
use binary_util::ByteWriter;
use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use openssl::cipher::CipherRef;
use openssl::cipher_ctx::CipherCtx;
use openssl::ec::{EcGroup, EcKey};
use openssl::nid::Nid;
use openssl::pkey::{PKey, Private, Public};
use p384::ecdh::diffie_hellman;
use p384::{PublicKey, SecretKey};
use p384::pkcs8::{DecodePrivateKey, DecodePublicKey};
use p384::pkcs8::der::Encode;
use serde::{Deserialize, Serialize};

pub struct MinecraftEncryption;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    salt: String,
}

/// 使用P-384(secp384r1)曲线与X509进行RSA加解密
impl MinecraftEncryption {
    pub const MOJANG_PUBLIC_KEY: &'static str = "MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAECRXueJeTDqNRRgJi/vlRufByu/2G0i2Ebt6YMar5QX/R0DIIyrJMcUpruK4QveTfJSTp3Shlq4Gk34cD/4GUWwkv0DVuzeuB+tXija7HBxii03NHDbPAD0AKnLr2wdAp";

    pub fn create_token() -> [u8; 16] {
        rand::random()
    }

    pub fn get_key_pair() -> (PKey<Private>, PKey<Public>) {
        let group = EcGroup::from_curve_name(Nid::SECP384R1).unwrap();
        let eckey = EcKey::generate(&group).unwrap();
        let private_key = eckey.private_key_to_der().unwrap();
        let private_key = PKey::private_key_from_der(private_key.as_slice()).unwrap();
        let public_key = eckey.public_key_to_der().unwrap();
        let public_key = PKey::public_key_from_der(public_key.as_slice()).unwrap();
        (private_key, public_key)
    }

    pub fn parse_key(key: &str) -> Option<PKey<Public>> {
        let base64_decode = base64::prelude::BASE64_STANDARD_NO_PAD.decode(key).ok()?;
        let base64_decode = base64_decode.as_slice();
        PKey::public_key_from_der(base64_decode).ok()
    }
    pub fn get_shared_secret(local_private_key: PKey<Private>, remote_public_key: PKey<Public>) -> Vec<u8> {
        let public_key = remote_public_key.public_key_to_der().unwrap();
        let public_key = PublicKey::from_public_key_der(public_key.as_slice()).unwrap();
        let private_key = local_private_key.private_key_to_pkcs8().unwrap();
        let private_key = SecretKey::from_pkcs8_der(private_key.as_slice()).unwrap();
        let shared_secret = diffie_hellman(private_key.to_nonzero_scalar(), public_key.as_affine());
        let raw_bytes = shared_secret.raw_secret_bytes();
        raw_bytes.to_vec()
    }

    pub fn get_secret_key(local_private_key: PKey<Private>, remote_public_key: PKey<Public>, token: [u8; 16]) -> Option<Vec<u8>> {
        let shared_secret = Self::get_shared_secret(local_private_key, remote_public_key);
        let mut hasher = openssl::sha::Sha256::new();
        hasher.update(token.as_slice());
        hasher.update(shared_secret.as_slice());
        let output = hasher.finish();
        Some(output.to_vec())
    }

    pub fn create_handshake_jwt(key_pair: (PKey<Private>, PKey<Public>), token: [u8; 16]) -> Option<String> {
        let mut header = Header::new(Algorithm::ES384);
        let public_key = key_pair.1.public_key_to_der().ok()?;
        let public_key = base64::encode(public_key.as_slice());
        header.x5u = Some(public_key.clone());
        header.typ = None;
        let claims = Claims {
            salt: base64::encode(token),
        };
        let binding = key_pair.0.private_key_to_pkcs8().unwrap();
        let temp = binding.as_slice();
        let key = EncodingKey::from_ec_der(temp);
        Some(encode(&header, &claims, &key).unwrap())
    }
}

pub struct MinecraftPacketEncryption {
    send_count: u32,
    receive_count: u32,
    encrypt_cipher: CipherCtx,
    decrypt_cipher: CipherCtx,
    secret_key: Vec<u8>,
}

impl MinecraftPacketEncryption {
    pub fn new(secret_key: Vec<u8>, protocol_version: u32) -> anyhow::Result<Self> {
        let cipher = Self::get_cipher(protocol_version);
        let temp = secret_key.clone();
        let key_leak = temp.leak();

        let iv = Self::get_iv(key_leak, protocol_version);
        let mut cipher_ctx = CipherCtx::new()?;
        cipher_ctx.set_padding(false);
        cipher_ctx.decrypt_init(Some(cipher), Some(key_leak), Some(iv))?;
        let decrypt_cipher = cipher_ctx;

        let mut cipher_ctx = CipherCtx::new()?;
        cipher_ctx.set_padding(false);
        cipher_ctx.encrypt_init(Some(cipher), Some(key_leak), Some(iv))?;
        let encrypt_cipher = cipher_ctx;

        Ok(Self {
            send_count: 0,
            receive_count: 0,
            encrypt_cipher,
            decrypt_cipher,
            secret_key,
        })
    }
    fn get_cipher(protocol_version: u32) -> &'static CipherRef {
        if protocol_version < 428 {//当协议版本低于428时
            openssl::cipher::Cipher::aes_256_cfb8()
        }else {
            openssl::cipher::Cipher::aes_256_gcm()
        }
    }

    fn get_iv(secret_key: &[u8], protocol_version: u32) -> &'static [u8] {
        if protocol_version < 428 {//当协议版本低于428时
            let mut temp = secret_key[0..16].to_vec();
            temp.leak()
        }else {
            let mut temp = vec![0u8; 16];
            temp[..12].copy_from_slice(&secret_key[..12]);
            temp[15] = 2;
            temp.leak()
        }
    }

    fn compute_checksum(&self, bytes: &[u8], send_count: u32) -> Vec<u8> {
        let mut hasher = openssl::sha::Sha256::new();
        let mut count = ByteWriter::new();
        count.write_i64_le(send_count as i64).unwrap();
        hasher.update(&count.as_slice());
        hasher.update(bytes);
        hasher.update(self.secret_key.as_slice());
        let output = hasher.finish();
        output[0..8].to_vec()
    }

    pub async fn decode(&mut self, bytes: &[u8]) -> Result<Vec<u8>, io::Error> {
        let mut packet = Vec::new();
        {
            let mut cipher_ctx = &mut self.decrypt_cipher;

            //解密
            cipher_ctx.cipher_update_vec(bytes, &mut packet).map_err(|error| io::Error::other(error))?;
        }

        let output = &packet[..packet.len() - 8];

        //验证checksum
        let output_checksum = packet[packet.len() - 8..].to_vec();
        let computed_checksum = self.compute_checksum(output, self.receive_count);
        if output_checksum != computed_checksum {
            return Err(io::Error::other("checksum error"))
        }
        self.receive_count += 1;
        Ok(output.to_vec())
    }

    pub fn encode(&mut self, bytes: &[u8]) -> Result<Vec<u8>, io::Error> {
        //计算checksum
        let checksum = self.compute_checksum(bytes, self.send_count);
        self.send_count += 1;

        let mut bytes = bytes.to_vec();
        bytes.extend(checksum);
        let mut output = Vec::new();
        //加密
        {
            let mut cipher_ctx = &mut self.encrypt_cipher;
            cipher_ctx.cipher_update_vec(bytes.as_slice(), &mut output).map_err(|error| io::Error::other(error))?;
        }
        Ok(output)
    }
}