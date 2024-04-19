pub fn login(creds: models:: Credentials){
    // match creds.username {
    //     "vishwas" => "vishwas"
    // }
    crate::database::get_user()
}


fn logout(){
    // log our user
}


pub mod models;