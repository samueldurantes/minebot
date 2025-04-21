use crate::config::Config;
use crate::ec2::Ec2Manager;
use serenity::all::Context;

pub mod server;

pub struct CommandContext {
    pub config: Config,
    pub default_ctx: Context,
    pub ec2_manager: Ec2Manager,
}
