use aes::{
    cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt},
    Aes256,
};
use aes_gcm::KeyInit;
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
