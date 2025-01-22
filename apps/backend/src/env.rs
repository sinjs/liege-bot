use std::sync::LazyLock;

fn required_var(name: &str) -> String {
    std::env::var(name).expect(&format!("Missing environment variable `{name}`"))
}

#[derive(Debug)]
pub struct Env {
    pub ai_token: String,
    pub code_token: String,
    pub discord_app_id: String,
    pub discord_client_secret: String,
    pub discord_token: String,
    pub discord_public_key: String,
    pub jwt_secret: String,
}

pub static ENV: LazyLock<Env> = LazyLock::new(|| {
    let env = Env {
        ai_token: required_var("AI_TOKEN"),
        code_token: required_var("CODE_TOKEN"),
        discord_app_id: required_var("DISCORD_APP_ID"),
        discord_client_secret: required_var("DISCORD_CLIENT_SECRET"),
        discord_public_key: required_var("DISCORD_PUBLIC_KEY"),
        discord_token: required_var("DISCORD_TOKEN"),
        jwt_secret: required_var("JWT_SECRET"),
    };

    tracing::debug!("lazily initialized environment");

    env
});
