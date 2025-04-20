use crate::config::Config;
use serenity::all::Context;

pub mod server;

pub struct CommandContext {
    pub config: Config,
    pub default_ctx: Context,
}
