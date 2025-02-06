fn main() {
    dotenv::dotenv().ok();
    if let Ok(database_url) = std::env::var("DATABASE_URL") {
        println!("cargo:rustc-env=DATABASE_URL={}", database_url);
    }
}
