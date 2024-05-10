



pub mod json_ld_test {
    include!("ed25519_test.rs");
    use didkit::ssi::{
        self,
        jsonld::{parse_ld_context, StaticLoader},
        rdf,
    };
    use json_ld::JsonLdProcessor;
    use sha256::digest;

    pub struct URDNA2015 {
        value: String,
    }

    pub async fn get_urdna2015_normalized_str(doc_str: &str) -> URDNA2015{
        let mut loader = StaticLoader;

        let json = ssi::jsonld::syntax::to_value_with(
            serde_json::from_str::<serde_json::Value>(&doc_str).unwrap(),
            Default::default,
        ).unwrap();
    
        let doc = ssi::jsonld::RemoteDocument::new(None, None, json);
        let mut generator = rdf_types::generator::Blank::new_with_prefix("b".to_string()).with_default_metadata();

        let mut to_rdf = doc
        .to_rdf(&mut generator, &mut loader)
        .await
        .map_err(Box::new)
        .unwrap();

        let dataset: rdf::DataSet = to_rdf
        .cloned_quads()
        .map(|q| q.map_predicate(|p| p.into_iri().unwrap()))
        .collect();
    

        let dataset_normalized = ssi::urdna2015::normalize(dataset.quads().map(Into::into));
        let normalized = dataset_normalized.into_nquads();
        println!("{:?}", normalized);
        return URDNA2015 {
            value: normalized
        };
    }


    pub async fn test() {
        
        let did_string = r#"
            {
                "@context": [
                    "https://www.w3.org/ns/did/v1",
                    "https://w3id.org/security/suites/ed25519-2020/v1"
                ],
                "id": "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH",
                "controller": [
                    "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH"
                ],
                "alsoKnownAs": [
                    "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH"
                ],
                "verificationMethod": [
                    {
                        "id": "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH#key-1",
                        "type": "Ed25519VerificationKey2020",
                        "controller": "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH",
                        "publicKeyMultibase": "z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH"
                    }
                ],
                "authentication": [
                    "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH#key-1"
                ],
                "assertionMethod": [
                    "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH#key-1"
                ],
                "keyAgreement": [],
                "capabilityInvocation": [],
                "capabilityDelegation": [],
                "service": []
            } 
        "#;

        // "proofValue": "z28ymbJ768VraSWc78v4t9vVvNMPHv3aThpCd1iA2P6ATooVM9AiG4dzEhpomF4VLcYXNVkB25Pi2XWDMJub4RyGB"

        let t = get_urdna2015_normalized_str(&did_string).await;
        let did_doc_normalized_hash = digest(t.value);
        println!("{:?}", did_doc_normalized_hash);

        let proof_string = r#"
            {
                "@context": [
                    "https://www.w3.org/ns/did/v1",
                    "https://w3id.org/security/suites/ed25519-2020/v1"
                ],
                "type": "Ed25519Signature2020",
                "created": "2024-05-09T08:11:42Z",
                "verificationMethod": "did:hid:testnet:z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH#key-1",
                "proofPurpose": "assertionMethod"
            } 
        "#;

        println!("================================================");
        let u = get_urdna2015_normalized_str(&proof_string).await;
        let did_doc_proof_normalized_hash = digest(u.value);
        println!("{:?}", did_doc_proof_normalized_hash);

        let message = format!("{}{}", did_doc_proof_normalized_hash, did_doc_normalized_hash);
        println!("Message: {:?}", message);

        let signature_str = "z28ymbJ768VraSWc78v4t9vVvNMPHv3aThpCd1iA2P6ATooVM9AiG4dzEhpomF4VLcYXNVkB25Pi2XWDMJub4RyGB";
        let public_key_str = "z6MkmX4KyqzfM4WGUVLFTtD2dVbyF5HmZwRrgWwkHLLgrCnH";
        ed25519_test::test(&public_key_str, &message, &signature_str);
    }
}
