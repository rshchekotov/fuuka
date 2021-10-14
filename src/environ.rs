use std::env;

pub fn load_env() {
    let exists = env::var("ENVIRONMENT").is_ok();
    let environ = if exists {
        env::var("ENVIRONMENT").ok().unwrap()
    } else {
        "develop".to_string()
    };

    let mut file = String::from(".env.");
    file.push_str(environ.as_str());

    dotenv::from_filename(file).expect("Could not load environment!");
    check_env();
}

fn is_dev() -> bool {
    return env::var("ENVIRONMENT")
        .ok()
        .unwrap_or("develop".to_string())
        .eq("develop");
}

fn check_env() {
    env::var("BOT_TOKEN").expect("Bot Token is undefined!");
    env::var("DISCORD_APP_ID").expect("Bot Token is undefined!");

    if is_dev() {
        env::var("DISCORD_DEV_GUILD").expect("Bot Token is undefined!");
    }
}
