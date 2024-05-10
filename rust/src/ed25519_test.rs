pub mod ed25519_test {
    use ed25519_dalek::ed25519::signature::{ Keypair, SignerMut };
    use ed25519_dalek::SigningKey;
    use ed25519_dalek::{ Signature };
    use ed25519_dalek::{ SECRET_KEY_LENGTH, KEYPAIR_LENGTH };
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
        //    let secret_key_bytes: [u8; SECRET_KEY_LENGTH] = [
        //    157, 097, 177, 157, 239, 253, 090, 096,
        //    186, 132, 074, 244, 146, 236, 044, 196,
        //    068, 073, 197, 105, 123, 050, 105, 025,
        //    112, 059, 172, 003, 028, 174, 127, 096, ];
        // generate key pair
        //let secret_key = "zrv4gLTkvUgXPWok7r71vTzcmF5tLuPGzzGL5ZzEQwySaCBmKJerJUSNGM8peNMfhqfkXQXBM2S6JSiCUuvw1W9ziQ5";
        let secret_key_bytes = decode_multibase_public_key(secret_key_str).unwrap();

        const TOTAL_LENGTH: usize = KEYPAIR_LENGTH + 2;
        let secret_key_array = vec_to_array::<TOTAL_LENGTH>(secret_key_bytes.to_owned()).unwrap();
        let key_pair_start_index = 2;
        let key_pair_end_index = secret_key_array.len();
        let key_pair_arry: [u8; 64] = secret_key_array[key_pair_start_index..key_pair_end_index]
            .try_into()
            .expect("Failed to create new array");

        // extract secret key from index 2 to 32
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

        let public_key_start_index = 2;
        let public_key_end_index = t_public_key_array.len();
        let public_key_array: [u8; 32] = t_public_key_array[
            public_key_start_index..public_key_end_index
        ]
            .try_into()
            .expect("Failed to create new array");

        // const ARRAY_LENGTH1: usize = 64;
        // let signature_str = "z28ymbJ768VraSWc78v4t9vVvNMPHv3aThpCd1iA2P6ATooVM9AiG4dzEhpomF4VLcYXNVkB25Pi2XWDMJub4RyGB";
        // println!("signature_str {:?}", signature_str);
        // let signature_bytes = decode_multibase_public_key(signature_str).unwrap();
        // let signature_array = vec_to_array::<ARRAY_LENGTH1>(signature_bytes.to_owned()).unwrap();

        // Anyone else, given the public half of the signing_key can also easily verify this signature:
        // extract secret key from index 32 to 64
        // let public_key_start_index = 32;
        // let public_key_end_index = 64;
        // let public_key_array: [u8; 32] = key_pair_arry[public_key_start_index..public_key_end_index].try_into().expect("Failed to create new array");

        let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = public_key_array;
        let verifying_key = VerifyingKey::from_bytes(&public_key_bytes).unwrap();

        return Ed25519KeyPair {
            signing_key: signing_key,
            verifying_key: verifying_key,
        };
    }
    
    pub fn test() {
        let secret_key =
            "zrv4gLTkvUgXPWok7r71vTzcmF5tLuPGzzGL5ZzEQwySaCBmKJerJUSNGM8peNMfhqfkXQXBM2S6JSiCUuvw1W9ziQ5";
        let public_key_str = "z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH";
        let mut key_pair: Ed25519KeyPair = generate_key(&secret_key, &public_key_str);

        // message
        let message: &[u8] = b"hello, world!";
        // let json_message: Value = json!({
        //     "@context": [
        //       "https://www.w3.org/ns/did/v1",
        //       "https://w3id.org/security/suites/ed25519-2020/v1"
        //     ],
        //     "id": "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH",
        //     "controller": [
        //       "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH"
        //     ],
        //     "alsoKnownAs": [
        //       "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH"
        //     ],
        //     "verificationMethod": [
        //       {
        //         "id": "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH#key-1",
        //         "type": "Ed25519VerificationKey2020",
        //         "controller": "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH",
        //         "publicKeyMultibase": "z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH"
        //       }
        //     ],
        //     "authentication": [
        //       "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH#key-1"
        //     ],
        //     "assertionMethod": [
        //       "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH#key-1"
        //     ],
        //     "keyAgreement": [],
        //     "capabilityInvocation": [],
        //     "capabilityDelegation": [],
        //     "service": [],
        //     "pro"
        // });

        // sign message using privaye key
        let signature: Signature = key_pair.signing_key.sign(message);
        println!("signature {:?}", signature.to_string());

        // let multibase_encoded_base58btc = multibase::encode(Base::Base58Btc, "E1D72AD37E78BB5FDA6B9D016CDFDF21C68D1B0BFA07EEE0564679DA58C2C01078E3802A4C2FED40098E464AC44945879FA62BD3D330484CF0EF1D95412F2C07");
        // println!("multibase_encoded_base58btc {:?}", multibase_encoded_base58btc);

        let res1 = key_pair.verifying_key.verify_strict(message, &signature);
        println!("result1 {:?}", res1.is_ok());
    }
}
