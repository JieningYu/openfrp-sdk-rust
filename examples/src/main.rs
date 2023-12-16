use openfrp_sdk::{
    login::{self, Account},
    sign,
    prelude::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = new_client()?;
    let account = Account {
        user: "example@example.com".to_string(),
        password: "example_password".to_string(),
    };
    let auth = login::login(&account, client.clone()).await?;
    let sign = sign::sign(&auth, client.clone()).await?;
    println!("{auth:#?}\n{sign:#?}");
    Ok(())
}
