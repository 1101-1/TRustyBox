use aes::{
    cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt},
    Aes256,
};
use aes_gcm::KeyInit;
use base64::{engine::general_purpose, Engine};
use rand::thread_rng;
use rand_core::RngCore;

pub async fn set_aes_key() -> [u8; 32] {
    let mut gen_aes_key = [0u8; 32];
    thread_rng().fill_bytes(&mut gen_aes_key);
    gen_aes_key
}

pub async fn encrypt_data(data: &[u8], aes_key: [u8; 32]) -> Result<Vec<u8>, tokio::io::Error> {
    let cipher = Aes256::new(&GenericArray::from_slice(&aes_key));
    let mut padded_data = data.to_vec();
    let pad_len = 16 - (data.len() % 16);
    let pad_byte = pad_len as u8;

    padded_data.resize(data.len() + pad_len, pad_byte);

    for block in padded_data.chunks_exact_mut(16) {
        let block = GenericArray::from_mut_slice(block);
        cipher.encrypt_block(block);
    }

    Ok(padded_data)
}

pub async fn decrypt_data(buf: &[u8], aes_key: [u8; 32]) -> Result<Vec<u8>, tokio::io::Error> {
    let cipher = Aes256::new(&GenericArray::from_slice(&aes_key));
    let mut decrypted_data = buf.to_vec();

    for block in decrypted_data.chunks_exact_mut(16) {
        let block = GenericArray::from_mut_slice(block);
        cipher.decrypt_block(block);
    }

    let pad_byte = *decrypted_data.last().unwrap();
    let pad_len = pad_byte as usize;

    decrypted_data.truncate(decrypted_data.len() - pad_len);

    Ok(decrypted_data)
}

pub async fn get_aes_key_from_base64(aes_key: String) -> Result<[u8; 32], tokio::io::Error> {
    let key_vec = match convert_base64_to_aes(aes_key).await {
        Ok(key) => key,
        Err(_err) => {
            return Err(tokio::io::Error::new(
                tokio::io::ErrorKind::Other,
                "Can not convert aes_key into base64",
            ))
        }
    };

    let key_array: [u8; 32] = match key_vec.try_into() {
        Ok(key) => key,
        Err(_err) => {
            return Err(tokio::io::Error::new(
                tokio::io::ErrorKind::Other,
                "Can not convert aes_key to bytes",
            ))
        }
    };
    Ok(key_array)
}

async fn convert_base64_to_aes(aes_key: String) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::URL_SAFE_NO_PAD.decode(aes_key)
}

pub async fn convert_aes_to_base64(aes_bytes: [u8; 32]) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(aes_bytes)
}
