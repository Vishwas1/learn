



pub mod json_ld_test {
    include!("ed25519_test.rs");

    use didkit::ssi::{
        self,
        jsonld::StaticLoader,
        rdf
    };
    // use json_ld::JsonLdProcessor;
    use sha256::digest;
    use serde_json::Value;
    use std::error::Error;

    use static_iref::iri;
    use sophia::jsonld::loader::NoLoader;
    use sophia::jsonld::parser::JsonLdParser;
    // use sophia::jsonld::JsonLdDataset;
    // use sophia::parser::jsonld::JsonLdParser;
    use sophia::inmem::graph::FastGraph;
    use json_ld::{JsonLdProcessor, Options, RemoteDocumentReference,RemoteDocument, warning};

    pub struct URDNA2015 {
        value: String,
    }

    fn parse_json_ld(json_ld_str: &str) -> Result<Value, Box<dyn Error>> {
        let json_ld: Value = serde_json::from_str(json_ld_str)?;
        Ok(json_ld)
    }

    // fn json_ld_to_rdf(json_ld: &Value) -> Result<FastGraph, Box<dyn Error>> {
    //     let mut graph = FastGraph::new();
    //     let mut parser = JsonLdParser::new();
    //     parser.parse_json(&mut graph, json_ld)?;
    //     Ok(graph)
    // }

   

    pub async fn get_urdna2015_normalized_str(doc_str: &str) -> URDNA2015{
        let mut loader =  StaticLoader;

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
        //println!("{:?}", normalized);
        return URDNA2015 {
            value: normalized
        };
    }

    //{"@context":["https://www.w3.org/ns/did/v1","https://w3id.org/security/suites/ed25519-2020/v1"],"id":"did:hid:testnet:z6Mkuiqxkri3QG7wMDBtBZ6HyqR7eH9FyWB4a2oLuaPcQjk9","controller":["did:hid:testnet:z6Mkuiqxkri3QG7wMDBtBZ6HyqR7eH9FyWB4a2oLuaPcQjk9"],"alsoKnownAs":["did:hid:testnet:1f49341a-de30993e6c52"],"verificationMethod":[{"id":"did:hid:testnet:z6Mkuiqxkri3QG7wMDBtBZ6HyqR7eH9FyWB4a2oLuaPcQjk9#key-1","type":"Ed25519VerificationKey2020","controller":"did:hid:testnet:z6Mkuiqxkri3QG7wMDBtBZ6HyqR7eH9FyWB4a2oLuaPcQjk9","publicKeyMultibase":"z6Mkuiqxkri3QG7wMDBtBZ6HyqR7eH9FyWB4a2oLuaPcQjk9"}],"service":[],"authentication":["did:hid:testnet:z6Mkuiqxkri3QG7wMDBtBZ6HyqR7eH9FyWB4a2oLuaPcQjk9#key-1"],"assertionMethod":["did:hid:testnet:z6Mkuiqxkri3QG7wMDBtBZ6HyqR7eH9FyWB4a2oLuaPcQjk9#key-1"]}
    pub async fn test() {
        
        let did_string = r#"
        {"@context":["https://www.w3.org/ns/did/v1","https://w3id.org/security/suites/ed25519-2020/v1"],"id":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B","controller":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"],"alsoKnownAs":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"],"verificationMethod":[{"id":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1","type":"Ed25519VerificationKey2020","controller":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B","publicKeyMultibase":"z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"}],"authentication":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"assertionMethod":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"keyAgreement":[],"capabilityInvocation":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"capabilityDelegation":[],"service":[{"id":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1","type":"LinkedDomains","serviceEndpoint":"https://www.linkeddomains.com"}]}
        "#;

        let t = get_urdna2015_normalized_str(&did_string).await;
        let did_doc_normalized_hash = digest(&t.value);
        //println!("did_doc_normalized_hash {:?}", did_doc_normalized_hash);

        let proof_string = r#"
            {
            "@context": [
                "https://www.w3.org/ns/did/v1",
                "https://w3id.org/security/suites/ed25519-2020/v1"
            ],
            "type":"Ed25519Signature2020",
            "created":"2010-01-01T19:23:24Z",
            "verificationMethod":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1",
            "proofPurpose":"assertionMethod"
            }
        "#;

        println!("================================================");
        let u = get_urdna2015_normalized_str(&proof_string).await;
        
        let did_doc_proof_normalized_hash = digest(&u.value);
        //println!("did_doc_proof_normalized_hash {:?}", did_doc_proof_normalized_hash);

        let message = format!("{}{}", did_doc_proof_normalized_hash, did_doc_normalized_hash);
        println!("Rust Side Message: {:?}", message);

        let signature_str = "z4S8Zxko4KLtHEKGkJVSPCrK4PcchJTYmcx3gsgxq3YG8uYQ3DJfaVufTDgjozNV174mZEmmUiib6J917jirmRfnY";                             
        let public_key_str = "z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B";
        let private_key_str =  "zrv1VisSe4RQXHFho3L6TjYmNPUUoHaCEfXt4gMGe4AVGyxMNRGCcHdZiFirrrw7yYoZeB9qCN9gGbLgkaHvBjCpcK7";

        ed25519_test::test(&public_key_str, &message, &signature_str, &private_key_str);
        // rdf_test::test();
    }
}
