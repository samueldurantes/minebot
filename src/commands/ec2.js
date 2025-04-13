const { SlashCommandBuilder } = require('@discordjs/builders');
const { EmbedBuilder } = require('discord.js');
const AWS = require('aws-sdk');

const config = require('../config');

// AWS SDK configuration
AWS.config.update({
  accessKeyId: config.AWS_ACCESS_KEY_ID,
  secretAccessKey: config.AWS_SECRET_ACCESS_KEY,
  region: config.AWS_REGION,
});

const ec2 = new AWS.EC2();

module.exports = {
  data: new SlashCommandBuilder()
    .setName('ec2')
    .setDescription('Controls the Minecraft server EC2 instance')
    .addSubcommand((subcommand) =>
      subcommand.setName('start').setDescription('Starts the EC2 instance')
    )
    .addSubcommand((subcommand) =>
      subcommand.setName('stop').setDescription('Stops the EC2 instance')
    )
    .addSubcommand((subcommand) =>
      subcommand
        .setName('status')
        .setDescription('Checks the EC2 instance status')
    ),

  async execute(interaction) {
    if (!config.EC2_INSTANCE_ID) {
      return interaction.reply({
        content: '❌ Error: EC2_INSTANCE_ID not configured in config.js',
        ephemeral: true,
      });
    }

    const subcommand = interaction.options.getSubcommand();

    try {
      switch (subcommand) {
        case 'start':
          await ec2
            .startInstances({ InstanceIds: [config.EC2_INSTANCE_ID] })
            .promise();
          await interaction.reply({
            embeds: [
              new EmbedBuilder()
                .setTitle('✅ EC2 Instance Started')
                .setDescription(
                  'The instance is starting. It may take a few minutes to become available.'
                )
                .setColor(0x00ff00),
            ],
          });
          break;

        case 'stop':
          await ec2
            .stopInstances({ InstanceIds: [config.EC2_INSTANCE_ID] })
            .promise();
          await interaction.reply({
            embeds: [
              new EmbedBuilder()
                .setTitle('🛑 EC2 Instance Stopped')
                .setDescription('The instance is shutting down.')
                .setColor(0xff0000),
            ],
          });
          break;

        case 'status':
          const response = await ec2
            .describeInstances({ InstanceIds: [config.EC2_INSTANCE_ID] })
            .promise();
          const state = response.Reservations[0].Instances[0].State.Name;

          let statusMessage, statusColor;
          switch (state) {
            case 'running':
              statusMessage = '🟢 Instance is running';
              statusColor = 0x00ff00;
              break;
            case 'stopped':
              statusMessage = '🔴 Instance is stopped';
              statusColor = 0xff0000;
              break;
            case 'pending':
              statusMessage = '🟡 Instance is starting';
              statusColor = 0xffff00;
              break;
            case 'stopping':
              statusMessage = '🟠 Instance is stopping';
              statusColor = 0xffa500;
              break;
            default:
              statusMessage = '⚪ Unknown status';
              statusColor = 0x808080;
          }

          await interaction.reply({
            embeds: [
              new EmbedBuilder()
                .setTitle('📊 EC2 Instance Status')
                .setDescription(statusMessage)
                .setColor(statusColor),
            ],
          });
          break;
      }
    } catch (error) {
      console.error('Error interacting with EC2:', error);
      await interaction.reply({
        content: `❌ Error executing command: ${error.message}`,
        ephemeral: true,
      });
    }
  },
};
