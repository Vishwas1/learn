include!("tutorial.rs");
// include!("ed25519_test.rs");
include!("json-ld-test.rs");
use tokio::runtime::Runtime; 

fn main() {
    // tutorial::match_expression();
    // tutorial::loop_statement();
    // tutorial::while_loop_statement();
    // tutorial::enum_keyword();
    // tutorial::struct_keyword();
    // tutorial::tuples_keyword();
    // tutorial::impl_keyword();
    // tutorial::vector_keyword();
    // tutorial::string_type();
    // tutorial::result_data_type();
    // tutorial::hash_map();
    // tutorial::traits_keyword();

    // ed25519_test::test();
    // test().;
    

    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        json_ld_test::test().await; // Call the asynchronous function and await its completion
    });
}

