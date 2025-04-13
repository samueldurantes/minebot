resource "aws_security_group" "minecraft_security_group" {
  name        = "Minecraft Security Group"
  description = "Security Group for allowing server access to minecraft players"

  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["64.252.81.0/24"]
    description = "Admin IP"
  }

  ingress {
    from_port   = 25565
    to_port     = 25565
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow minecraft connections"
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic"
  }
}

resource "aws_key_pair" "minecraft_key" {
  key_name   = "minecraft-key"
  public_key = file("~/.ssh/id_ed25519.pub")
}

resource "aws_eip" "minecraft_eip" {
  instance = aws_instance.minecraft_server.id
  tags = {
    Name = "Minecraft Server EIP"
  }
}

resource "aws_instance" "minecraft_server" {
  # Amazon Linux 2023 ARM64 AMI
  ami           = "ami-00106fc08d77d8f51"
  # t4g.medium is the minimum instance type for Minecraft server
  instance_type = "t4g.medium"
  
  vpc_security_group_ids = [aws_security_group.minecraft_security_group.id]
  key_name               = aws_key_pair.minecraft_key.key_name

  user_data = <<-EOF
              #!/bin/bash

              MINECRAFTSERVERURL=https://piston-data.mojang.com/v1/objects/e6ec2f64e6080b9b5d9b471b291c33cc7f509733/server.jar

              sudo yum install -y java-21-amazon-corretto-headless
              adduser minecraft
              mkdir /opt/minecraft/
              mkdir /opt/minecraft/server/
              cd /opt/minecraft/server

              wget $MINECRAFTSERVERURL

              sudo chown -R minecraft:minecraft /opt/minecraft/
              sudo chmod -R u+rwX /opt/minecraft
              java -Xmx1300M -Xms1300M -jar server.jar nogui
              sleep 40
              sed -i 's/false/true/p' eula.txt
              touch start
              printf '#!/bin/bash\njava -Xmx1300M -Xms1300M -jar server.jar nogui\n' >> start
              chmod +x start
              sleep 1
              touch stop
              printf '#!/bin/bash\nkill -9 $(ps -ef | pgrep -f "java")' >> stop
              chmod +x stop
              sleep 1

              cd /etc/systemd/system/
              touch minecraft.service
              printf '[Unit]\nDescription=Minecraft Server\nAfter=network.target\n\n[Service]\nUser=minecraft\nNice=1\nKillMode=none\nSuccessExitStatus=0 1\nProtectHome=true\nProtectSystem=full\nPrivateDevices=true\nNoNewPrivileges=true\nWorkingDirectory=/opt/minecraft/server\nExecStart=/opt/minecraft/server/start\nExecStop=/opt/minecraft/server/stop\n\n[Install]\nWantedBy=multi-user.target' >> minecraft.service
              sudo systemctl daemon-reload
              sudo systemctl enable minecraft.service
              sudo systemctl start minecraft.service
              EOF

  tags = {
    Name = "Minecraft Server"
  }
}
