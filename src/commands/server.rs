use super::CommandContext;
use crate::ec2::Ec2Manager;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use serenity::{
    all::{CommandOptionType, CreateCommandOption, CreateEmbed},
    model::colour,
};

fn codebox(message: &str) -> String {
    format!("```\n\n{}\n\n```", message)
}

fn embed_error(message: String) -> CreateEmbed {
    CreateEmbed::new()
        .title("An error occurred")
        .description(message)
        .color(colour::Colour::from_rgb(255, 0, 0))
}

async fn start_server(ec2_manager: &Ec2Manager) -> CreateEmbed {
    match ec2_manager.start_instance().await {
        Ok(_) => CreateEmbed::new()
            .title("✅ Instance server started")
            .description("The instance is starting. It may take a few minutes to become available.")
            .color(colour::Colour::from_rgb(0, 255, 100)),
        Err(e) => embed_error(format!("Error starting server: {}", e)),
    }
}

async fn stop_server(ec2_manager: &Ec2Manager) -> CreateEmbed {
    match ec2_manager.stop_instance().await {
        Ok(_) => CreateEmbed::new()
            .title("🛑 Instance server stopped")
            .description(
                "The instance is stopping. It may take a few minutes to become unavailable.",
            )
            .color(colour::Colour::from_rgb(255, 0, 0)),
        Err(e) => embed_error(format!("Error stopping server: {}", e)),
    }
}

async fn status_server(ctx: &CommandContext, ec2_manager: &Ec2Manager) -> CreateEmbed {
    match ec2_manager.get_instance_status().await {
        Ok(status) => {
            let is_stopped = status.contains("stopped");

            if is_stopped {
                return CreateEmbed::new()
                    .title("🖥️ Server status")
                    .description("The server is shutting down.")
                    .color(colour::Colour::from_rgb(255, 0, 0));
            }

            let mc_connection_builder = async_minecraft_ping::ConnectionConfig::build(
                ctx.config.minecraft_server_ip.clone(),
            )
            .with_port(ctx.config.minecraft_server_port.parse::<u16>().unwrap());

            let mc_connect = mc_connection_builder.connect().await;

            if let Ok(mc_connect) = mc_connect {
                if let Ok(status) = mc_connect.status().await {
                    let status = status.status;
                    let players = match status.players.sample {
                        Some(players) => players
                            .iter()
                            .map(|player| format!("{} ({})", player.name, player.id))
                            .collect::<Vec<String>>()
                            .join("\n")
                            .replace("_", "\\_"),
                        None => String::from("No players online"),
                    };

                    return CreateEmbed::new()
                        .title("🖥️ Server status")
                        .fields(vec![
                            ("Status", codebox("Online"), true),
                            (
                                "Player count",
                                codebox(&format!(
                                    "{} / {}",
                                    status.players.online, status.players.max
                                )),
                                true,
                            ),
                            ("Players", codebox(&players), false),
                            (
                                "Server IP",
                                codebox(&format!(
                                    "{}:{}",
                                    ctx.config.minecraft_server_ip,
                                    ctx.config.minecraft_server_port
                                )),
                                true,
                            ),
                            ("Version", codebox(&status.version.name), true),
                        ])
                        .color(colour::Colour::from_rgb(0, 255, 0));
                }
            }

            return embed_error("It was not possible to connect to the server.".to_string());
        }
        Err(e) => embed_error(format!("Error getting server status: {}", e)),
    }
}

pub async fn run(ctx: CommandContext, options: &[ResolvedOption<'_>]) -> CreateEmbed {
    let option = options.get(0);

    if let Some(option) = option {
        let name = option.name.to_owned();

        if name == "start" {
            return start_server(&ctx.ec2_manager).await;
        }

        if name == "stop" {
            return stop_server(&ctx.ec2_manager).await;
        }

        if name == "status" {
            return status_server(&ctx, &ctx.ec2_manager).await;
        }
    }

    embed_error("Error getting option".to_string())
}

pub fn register() -> CreateCommand {
    let options = vec![
        CreateCommandOption::new(CommandOptionType::SubCommand, "start", "Start the server"),
        CreateCommandOption::new(CommandOptionType::SubCommand, "stop", "Stop the server"),
        CreateCommandOption::new(CommandOptionType::SubCommand, "status", "Get server status"),
    ];

    CreateCommand::new("server")
        .description("Controls the Minecraft server")
        .set_options(options)
}
