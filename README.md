# Minecraft Server Discord Bot

A Discord bot that controls a Minecraft server hosted on AWS EC2.

## Features

- Start/stop the Minecraft server
- Check server status
- Slash commands support
- Terraform files for creating the infrastructure

## Setup

1. Install dependencies:

```bash
pnpm install
```

2. Create a `.env` file with:

```bash
cp .env.example .env
```

3. Edit the `.env` file with your Discord token and AWS credentials:

4. Start the bot:

```bash
pnpm start
```

5. Run bot in development mode:

```bash
pnpm dev
```
