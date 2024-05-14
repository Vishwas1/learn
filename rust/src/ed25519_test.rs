pub mod ed25519_test {
    use ed25519_dalek::ed25519::signature::{ Keypair, SignerMut };
    use ed25519_dalek::SigningKey;
    use ed25519_dalek::{ Signature};
    use ed25519_dalek::{ SECRET_KEY_LENGTH, KEYPAIR_LENGTH,  };
    use ed25519_dalek::VerifyingKey;
    use json_ld::print;
    use ed25519_dalek::PUBLIC_KEY_LENGTH;
    use serde_json::{ json, Value };
    use sha256::digest;
    use ed25519_dalek::{Verifier};
    // use multibase::{encode, decode, Base};
    use ed25519_dalek::{DigestSigner};
    use bincode::deserialize;
    use base58::ToBase58;
    use multibase::{encode, Base};

    use didkit::ssi::{
        self,
        jsonld::{parse_ld_context, StaticLoader},
        rdf,
        jwk::{Algorithm, Error}
    };

    struct Ed25519KeyPair {
        signing_key: SigningKey,
        verifying_key: VerifyingKey,
    }

    fn decode_multibase_public_key(multibase_str: &str) -> Result<Vec<u8>, String> {
        let decoded = multibase::decode(multibase_str).unwrap();
        let (base, data) = decoded;
        match base {
            Base::Base58Btc => {
                // `data` is the decoded byte array
                // println!("Decoded data (Base58btc): {:?}", data);
                //println!("Decoded data (Base58btc) vec: {:?}", data.to_vec());
            }
            _ => {
                println!("Unsupported base: {:?}", base);
            }
        }

        Ok(data)

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
        let secret_key_bytes = decode_multibase_public_key(secret_key_str).unwrap();
        println!("secret_key_byteslen : {}", secret_key_bytes.len());

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
        let public_key_bytes = decode_multibase_public_key(public_key_str).unwrap();
        println!("public_key_bytes len : {}", public_key_bytes.len());
        let t_public_key_array = vec_to_array::<ARRAY_LENGTH>(public_key_bytes.to_owned()).unwrap();

        //extract secret key from index 2 to 32
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
    



    fn generate_verifying_key(public_key_str: &str) -> VerifyingKey {
        const ARRAY_LENGTH: usize = 34;
        let public_key_bytes = decode_multibase_public_key(public_key_str).unwrap();
        let t_public_key_array = vec_to_array::<ARRAY_LENGTH>(public_key_bytes.to_owned()).unwrap();
        println!("t_public_key_array.len {:?}",t_public_key_array.len());

        // extract secret key from index 2 to 32
        let public_key_start_index = 2;
        let public_key_end_index = t_public_key_array.len();
        let public_key_array: [u8; 32] = t_public_key_array[
            public_key_start_index..public_key_end_index
        ]
            .try_into()
            .expect("Failed to create new array");
        let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = public_key_array;
        println!("public_key_bytes.len {:?}",public_key_bytes.len());

        let verifying_key = ed25519_dalek::VerifyingKey::from_bytes(&public_key_bytes).unwrap();
        // let verifying_key =  VerifyingKey::from(&public_key_bytes);
        // println!("verifying_key.len {:?}",verifying_key);


        return verifying_key
    }

    // pub fn test1(encoded_verifying_key: &str, encoded_signature: &str)->bool{


        
    //     let message: &[u8] = b"53fb655cc23c4fccd5dba9d8be70bc46662cd7aaf7c22f5cf843f2919aaffd64290c8b14409f78bf6aab4aad985e7c5524bc2e7bf1f7663404ea628576ac4119";
    //     let decoded_verifying_key: VerifyingKey = deserialize(&encoded_verifying_key.as_bytes()).unwrap();
    //     let decoded_signature: Signature = deserialize(&encoded_signature.as_bytes()).unwrap();
    //     let verified: bool = decoded_verifying_key.verify(&message, &decoded_signature).is_ok();
    //     println!("Result {:?} ", verified);
    //     verified
    // }

    pub fn test(public_key_str: &str,  m:  &str, signature_str1: &str, algorithm: Algorithm, private_key_str: &str) {
        
        // message
        // m = "abcd";
        let hex_decode_message = hex::decode(m);
        let message: &[u8] = &hex_decode_message.unwrap();
    
        // sign message using privaye key
        // let secret_key = private_key_str;        
        // let mut key_pair: Ed25519KeyPair = generate_key(&secret_key, &public_key_str);
        // let signature: Signature = key_pair.signing_key.sign(message);
        // let signature_bytes = &signature.to_bytes();
        // let signature_hex = hex::encode(signature_bytes);
        // println!("Hex_sig: {}", signature_hex);
        // let base58_string = signature_bytes.to_base58();
        // println!("signature_multibase_encoded z{}", base58_string.to_string()); 
        
        const SIGNATURE_BYTE_SIZE: usize = 64;
        let signature_bytes = decode_multibase_public_key(signature_str1).unwrap();
        println!("signature_bytes {:?}", signature_bytes.len()); 
        let signature_array = vec_to_array::<SIGNATURE_BYTE_SIZE>(signature_bytes.to_owned()).unwrap();        
        println!("signature_str1_len {:?}", signature_array.len());
        let signature: Signature = Signature::from_bytes(&signature_array);
        let verifying_key: ed25519_dalek::VerifyingKey = generate_verifying_key(&public_key_str);

        let res1 = verifying_key.verify(&message, &signature).is_ok();
        println!("result1 {:?}", res1);
    }
}
