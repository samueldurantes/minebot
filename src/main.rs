use minebot::{
    commands::{self, CommandContext},
    config::Config,
};
use serenity::{
    Client,
    all::{
        Command, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
        EventHandler, GatewayIntents, Interaction, Ready,
    },
    async_trait,
};

struct Handler {
    pub config: Config,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let command_ctx = CommandContext {
            default_ctx: ctx.clone(),
            config: self.config.clone(),
        };

        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "server" => Some(commands::server::run(command_ctx, &command.data.options()).await),
                _ => None,
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().add_embed(content);
                let builder = CreateInteractionResponse::Message(data);

                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }

                return;
            }

            println!("Command {} doesn't exist", command.data.name);
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let commands = vec![commands::server::register()];

        for command in commands {
            let _ = Command::create_global_command(&ctx.http, command).await;
        }

        println!("{} is connected!", ready.user.tag());
    }
}

#[tokio::main]
async fn main() {
    let config = Config::init();

    let mut client = Client::builder(config.clone().discord_token, GatewayIntents::empty())
        .event_handler(Handler { config })
        .await
        .expect("Error when trying to create the client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
