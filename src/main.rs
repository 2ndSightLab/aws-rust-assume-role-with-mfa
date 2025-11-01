// Author: Teri Radichel
// Organization: 2ndSightLab.com

use rust_diggity_aws_auth::assume_role_with_mfa;
use rust_diggity_aws_auth::ConsolePrompt;
use aws_sdk_s3 as s3;


#[tokio::main]
async fn main() {
    let prompter = ConsolePrompt;
    match assume_role_with_mfa(&prompter).await {
        Ok(assumed_role_config) => {
            println!("\nSuccessfully assumed role!");

            let s3_client = s3::Client::new(&assumed_role_config);

            match s3_client.list_buckets().send().await {
                Ok(output) => {
                    println!("Successfully listed S3 buckets with the assumed role.");
                    // Correctly handle the Option<Vec<Bucket>> returned by `buckets()`
                    let buckets = output.buckets();
                    for bucket in buckets {

                            if let Some(bucket_name) = bucket.name() {
                                println!("  - {}", bucket_name);
                            } else {
                               println!("  - {}", bucket.name().unwrap_or("unknown"));
                            }
                    
                    }
                    
                },
                Err(e) => eprintln!("Failed to list buckets: {:?}", e),
            }
        },
        Err(e) => {
            eprintln!("\nFailed to assume role: {:?}", e);
        }
    }
}
