# minebot

A Discord bot that controls a Minecraft server hosted on AWS EC2.

## Features

- Start/stop the Minecraft server
- Check server status
- Slash commands support
- Terraform files for creating the infrastructure

## Setup

1. Create a `.env` file with:

```bash
cp .env.example .env
```

2. Edit the `.env` file with your credentials.

3. Start the bot:

```bash
cargo run
```
