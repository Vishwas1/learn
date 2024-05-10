pub mod ed25519_test {
    use ed25519_dalek::ed25519::signature::{ Keypair, SignerMut };
    use ed25519_dalek::SigningKey;
    use ed25519_dalek::{ Signature};
    use ed25519_dalek::{ SECRET_KEY_LENGTH, KEYPAIR_LENGTH,  };
    use ed25519_dalek::VerifyingKey;
    use multibase::{ decode };
    use ed25519_dalek::PUBLIC_KEY_LENGTH;
    use serde_json::{ json, Value };
    
    struct Ed25519KeyPair {
        signing_key: SigningKey,
        verifying_key: VerifyingKey,
    }

    fn decode_multibase_public_key(multibase_str: &str) -> Result<Vec<u8>, String> {
        let decoded = decode(multibase_str).unwrap();
        let (_, data) = decoded;
        Ok(data.to_vec())
    }

    fn vec_to_array<const N: usize>(input: Vec<u8>) -> Result<[u8; N], &'static str> {
        // Ensure the input Vec has the correct length for the array
        if input.len() != N {
            return Err("Input Vec length does not match the desired array length");
        }

        // Try to convert the Vec into a fixed-size array
        let mut array = [0u8; N];
        array.copy_from_slice(&input);

        Ok(array)
    }

    fn generate_key(secret_key_str: &str, public_key_str: &str) -> Ed25519KeyPair {
        //let secret_key = "zrv4gLTkvUgXPWok7r71vTzcmF5tLuPGzzGL5ZzEQwySaCBmKJerJUSNGM8peNMfhqfkXQXBM2S6JSiCUuvw1W9ziQ5";
        let secret_key_bytes = decode_multibase_public_key(secret_key_str).unwrap();

        const TOTAL_LENGTH: usize = KEYPAIR_LENGTH + 2;
        let secret_key_array = vec_to_array::<TOTAL_LENGTH>(secret_key_bytes.to_owned()).unwrap();
        let key_pair_start_index = 2;
        let key_pair_end_index = secret_key_array.len();
        let key_pair_arry: [u8; 64] = secret_key_array[key_pair_start_index..key_pair_end_index]
            .try_into()
            .expect("Failed to create new array");

        
        let secret_start_index = 0;
        let secret_end_index = 32;
        let secret_key_arry: [u8; 32] = key_pair_arry[secret_start_index..secret_end_index]
            .try_into()
            .expect("Failed to create new array");
        let mut signing_key: SigningKey = SigningKey::from_bytes(&secret_key_arry);

        const ARRAY_LENGTH: usize = 34;
        //let public_key_str = "z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH";
        let public_key_bytes = decode_multibase_public_key(public_key_str).unwrap();
        let t_public_key_array = vec_to_array::<ARRAY_LENGTH>(public_key_bytes.to_owned()).unwrap();

        // extract secret key from index 2 to 32
        let public_key_start_index = 2;
        let public_key_end_index = t_public_key_array.len();
        let public_key_array: [u8; 32] = t_public_key_array[
            public_key_start_index..public_key_end_index
        ]
            .try_into()
            .expect("Failed to create new array");

        let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = public_key_array;
        let verifying_key = VerifyingKey::from_bytes(&public_key_bytes).unwrap();

        return Ed25519KeyPair {
            signing_key: signing_key,
            verifying_key: verifying_key,
        };
    }
    
    pub fn test(public_key_str: &str, m: &str, signature_str: &str) {
        let secret_key =
            "zrv4gLTkvUgXPWok7r71vTzcmF5tLuPGzzGL5ZzEQwySaCBmKJerJUSNGM8peNMfhqfkXQXBM2S6JSiCUuvw1W9ziQ5";
        
        let mut key_pair: Ed25519KeyPair = generate_key(&secret_key, &public_key_str);

        // message
        let message: &[u8] = m.as_bytes();
    
        // sign message using privaye key
        // let signature: Signature = key_pair.signing_key.sign(message);
        // println!("signature {:?}", signature.to_string());

        println!("signature_str {:?}", signature_str);
        const SIGNATURE_BYTE_SIZE: usize = 64;
        let signature_bytes = decode_multibase_public_key(signature_str).unwrap();
        let signature_array = vec_to_array::<SIGNATURE_BYTE_SIZE>(signature_bytes.to_owned()).unwrap();
        println!("signature_str_len {:?}", signature_array.len());
        let signature_start_index = 0;
        let signature_end_index = signature_array.len();
        let signature_array: [u8; SIGNATURE_BYTE_SIZE] = signature_array[
            signature_start_index..signature_end_index
        ].try_into().expect("Failed to extract signature");
        let signature: Signature = Signature::from_bytes(&signature_array);
         
        let res1 = key_pair.verifying_key.verify_strict(&message, &signature);
        println!("result1 {:?}", res1.is_ok());
    }
}
