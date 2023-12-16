use openfrp_sdk::*;

fn main() -> Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let client = client()?;
            let account = Account {
                user: "example@example.com".to_string(),
                password: "example_password".to_string(),
            };
            let auth = login(&account, client.clone()).await?;
            let sign = sign(&auth, client.clone()).await?;
            println!("{auth:#?}\n{sign:#?}");
            Ok(())
        })
}
