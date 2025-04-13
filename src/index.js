const {
  Client,
  GatewayIntentBits,
  Partials,
  Collection,
} = require('discord.js');
const express = require('express');
const fs = require('fs');
const path = require('path');

const config = require('./config');

// Express server configuration
const app = express();
const port = process.env.PORT || 3000;

// Health check route
app.get('/health', (req, res) => {
  res.status(200).json({ status: 'ok' });
});

// Start Express server
app.listen(port, () => {
  console.log(`Health check server running on port ${port}`);
});

// Discord client configuration
const client = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.MessageContent,
  ],
  partials: [Partials.Channel],
});

// Command collection
client.commands = new Collection();

// Load commands
const commandsPath = path.join(__dirname, 'commands');
const commandFiles = fs
  .readdirSync(commandsPath)
  .filter((file) => file.endsWith('.js'));

for (const file of commandFiles) {
  const filePath = path.join(commandsPath, file);
  const command = require(filePath);
  client.commands.set(command.data.name, command);
}

// Event when bot is ready
client.once('ready', () => {
  console.log(`Bot is online as ${client.user.tag}`);
});

// Interaction event
client.on('interactionCreate', async (interaction) => {
  if (!interaction.isCommand()) return;

  const command = client.commands.get(interaction.commandName);

  if (!command) return;

  try {
    await command.execute(interaction);
  } catch (error) {
    console.error(error);
    await interaction.reply({
      content: 'There was an error while executing this command!',
      ephemeral: true,
    });
  }
});

// Bot login
client.login(config.DISCORD_TOKEN);
