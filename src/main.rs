use exitfailure::ExitFailure;

mod user;
mod address;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    
    // fetch user API
    let url: String = "https://dummyjson.com/c/41de-dc42-40cc-8141".to_string();
    let res: user::User = user::User::get(url).await?;
    println!("Name: {}", res.name);

    // // fetch address API
    let url: String = "https://dummyjson.com/c/0dea-f36c-43aa-a7c1".to_string();
    let res: address::Address = address::Address::get(url).await?;
    println!("Address: {}", res.name);

    Ok(())
}
