use aws_sdk_ec2::Client;
use aws_sdk_ec2::types::InstanceStateName;
use std::fmt;

#[derive(Debug)]
pub enum Ec2Error {
    AwsError(String),
    InstanceNotFound,
    StateNotFound,
    ConfigurationError(String),
}

impl std::error::Error for Ec2Error {}

impl fmt::Display for Ec2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ec2Error::AwsError(e) => write!(f, "AWS Error: {}", e),
            Ec2Error::InstanceNotFound => write!(f, "Instance not found"),
            Ec2Error::StateNotFound => write!(f, "State not found"),
            Ec2Error::ConfigurationError(e) => write!(f, "Configuration Error: {}", e),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ec2Manager {
    client: Client,
    instance_id: String,
}

impl Ec2Manager {
    pub async fn new(instance_id: String) -> Result<Self, Ec2Error> {
        let config = aws_config::load_from_env().await;
        
        // Verify AWS credentials are set
        if config.credentials_provider().is_none() {
            return Err(Ec2Error::ConfigurationError(
                "AWS credentials not found. Please set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY".to_string(),
            ));
        }

        // Verify region is set
        if config.region().is_none() {
            return Err(Ec2Error::ConfigurationError(
                "AWS region not found. Please set AWS_REGION".to_string(),
            ));
        }

        let client = Client::new(&config);

        dbg!(&config);

        Ok(Self {
            client,
            instance_id,
        })
    }

    pub async fn start_instance(&self) -> Result<String, Ec2Error> {
        self.client
            .start_instances()
            .instance_ids(&self.instance_id)
            .send()
            .await
            .map_err(|e| Ec2Error::AwsError(e.to_string()))?;

        Ok("Starting server...".to_string())
    }

    pub async fn stop_instance(&self) -> Result<String, Ec2Error> {
        self.client
            .stop_instances()
            .instance_ids(&self.instance_id)
            .send()
            .await
            .map_err(|e| Ec2Error::AwsError(e.to_string()))?;

        Ok("Stopping server...".to_string())
    }

    pub async fn get_instance_status(&self) -> Result<String, Ec2Error> {
        let response = self
            .client
            .describe_instances()
            .instance_ids(&self.instance_id)
            .send()
            .await
            .map_err(|e| Ec2Error::AwsError(e.to_string()))?;

        if let Some(reservation) = response.reservations().first() {
            if let Some(instance) = reservation.instances().first() {
                if let Some(state) = instance.state() {
                    match state.name() {
                        Some(InstanceStateName::Running) => {
                            return Ok("Server is running".to_string());
                        }
                        Some(InstanceStateName::Stopped) => {
                            return Ok("Server is stopped".to_string());
                        }
                        Some(InstanceStateName::Pending) => {
                            return Ok("Server is starting...".to_string());
                        }
                        Some(InstanceStateName::Stopping) => {
                            return Ok("Server is stopping...".to_string());
                        }
                        _ => return Ok("Unknown status".to_string()),
                    }
                }
                return Err(Ec2Error::StateNotFound);
            }
        }
        Err(Ec2Error::InstanceNotFound)
    }
}
