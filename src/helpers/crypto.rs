// // use data_encoding::HEXUPPER;
// // use ring::error::Unspecified;
// use ring::rand::SecureRandom;
// use ring::{digest, pbkdf2, rand};
// use std::num::NonZeroU32;

// const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;

// pub fn encrypt_password(password: String) -> ([u8; 64], [u8; 64]) {
//     let n_iter = NonZeroU32::new(100_000).unwrap();
//     let rng = rand::SystemRandom::new();
//     let mut salt = [0u8; CREDENTIAL_LEN];
//     rng.fill(&mut salt);
//     // let password = "Guess Me If You Can!";
//     let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
//     pbkdf2::derive(
//         pbkdf2::PBKDF2_HMAC_SHA512,
//         n_iter,
//         &salt,
//         password.as_bytes(),
//         &mut pbkdf2_hash,
//     );

//     (pbkdf2_hash, salt)
// }

// pub fn verify_password(password: String, salt: &[u8], hash: &[u8]) -> bool {
//     let n_iter = NonZeroU32::new(100_000).unwrap();
//     let should_succeed = pbkdf2::verify(
//         pbkdf2::PBKDF2_HMAC_SHA512,
//         n_iter,
//         &salt,
//         password.as_bytes(),
//         &hash,
//     );

//     should_succeed.is_ok()
// }
