use std::env::var;

#[derive(Clone)]
pub struct Config {
    pub discord_token: String,
    pub ec2_instance_id: String,
    pub minecraft_server_ip: String,
    pub minecraft_server_port: String,
}

impl Config {
    pub fn init() -> Self {
        // Only load .env file in development
        if cfg!(debug_assertions) {
            dotenvy::from_path(".env").ok();
        }

        Self {
            discord_token: var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set"),
            ec2_instance_id: var("EC2_INSTANCE_ID").expect("EC2_INSTANCE_ID must be set"),
            minecraft_server_ip: var("MINECRAFT_SERVER_IP").expect("MINECRAFT_SERVER_IP must be set"),
            minecraft_server_port: var("MINECRAFT_SERVER_PORT").expect("MINECRAFT_SERVER_PORT must be set"),
        }
    }
}
