use std::io::{self, Write};
use aws_config::{BehaviorVersion, defaults};
use aws_credential_types::Credentials;
use aws_types::region::Region;
use aws_sdk_sts as sts;
use std::time::SystemTime;

// Define a trait for getting credential information
pub trait PromptForMfaDetails {
    fn get_access_key(&self) -> String;
    fn get_secret_key(&self) -> String;
    fn get_mfa_serial(&self) -> String;
    fn get_role_arn(&self) -> String;
    fn get_mfa_token(&self) -> String;
    fn get_session_name(&self) -> String;
}

// Implement the trait for a concrete type, like a command-line prompt
pub struct ConsolePrompt;

impl PromptForMfaDetails for ConsolePrompt {
    fn get_access_key(&self) -> String {
        get_input("Enter your AWS Access Key ID: ")
    }
    fn get_secret_key(&self) -> String {
        get_input("Enter your AWS Secret Access Key: ")
    }
    fn get_mfa_serial(&self) -> String {
        get_input("Enter MFA Device Serial Number (ARN): ")
    }
    fn get_role_arn(&self) -> String {
        get_input("Enter Role ARN to assume: ")
    }
    fn get_mfa_token(&self) -> String {
        get_input("Enter MFA Token: ")
    }
    fn get_session_name(&self) -> String {
        get_input("Enter Session Name: ")
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
// Core function, now generic over any type that implements `PromptForMfaDetails`
pub async fn assume_role_with_mfa<T: PromptForMfaDetails>(prompter: &T) -> Result<aws_config::SdkConfig, Box<dyn std::error::Error>> {

    let initial_credentials = Credentials::new(
        prompter.get_access_key().to_string(),  // Convert &str to String
        prompter.get_secret_key().to_string(),  // Convert &str to String
        None,
        None,
        "cli-input",
    );

    let initial_config = aws_config::defaults(BehaviorVersion::latest()) // Removed extra '.'
      .credentials_provider(initial_credentials)
      .region(Region::new("us-east-1"))
      .load()
      .await;
    
    let sts_client = sts::Client::new(&initial_config);

    let assumed_role_result = sts_client
        .assume_role()
        .role_arn(&prompter.get_role_arn())
        .role_session_name(&prompter.get_session_name())
        .serial_number(&prompter.get_mfa_serial())
        .token_code(&prompter.get_mfa_token())
        .send()
        .await;

    match assumed_role_result {
      Ok(response) => {
        if let Some(credentials) = response.credentials() {
            let assumed_role_config = defaults(BehaviorVersion::latest())
                .credentials_provider(Credentials::new(
                    credentials.access_key_id(),
                    credentials.secret_access_key(),
                    Some(credentials.session_token().to_string()), 
                    Some(SystemTime::try_from(*credentials.expiration()).unwrap()),
                    "assumed-role-credentials"
                ))
                .region(initial_config.region().cloned())
                .load()
                .await;
            Ok(assumed_role_config)
        } else {
            Err("No credentials returned from assume role operation".to_string().into())
        }
      }
      Err(e) => Err(e.into()),
    }

}
