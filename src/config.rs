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
        dotenvy::from_path(".env").expect(".env file not found");

        Self {
            discord_token: var("DISCORD_TOKEN").expect("DISCORD_TOKEN is empty"),
            ec2_instance_id: var("EC2_INSTANCE_ID").expect("EC2_INSTANCE_ID is empty"),
            minecraft_server_ip: var("MINECRAFT_SERVER_IP").expect("MINECRAFT_SERVER_IP is empty"),
            minecraft_server_port: var("MINECRAFT_SERVER_PORT")
                .expect("MINECRAFT_SERVER_PORT is empty"),
        }
    }
}
