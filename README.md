# aws-rust-assume-role-with-mfa

If it helps anyone...this is a test to assume a role with MFA using the AWS rust sdk.

I created a trait that can be reused in any application as demonstrated by the main.rs module.

This is NOT production code. This simply shows the neccessary libraries and such and is a working simple to get you started.

What this code does is asks you to input the values neccessary to assume a role with MFA and the rust code assumes the role.

The main.rs applicaiton lists the s3 buckets in the account and region of the assumed role.

Why would you want to do this? Well, you probably don't want to enter your creds over and over again so you probably don't want to do exactly what this code is doing. What you can do though is pull the credentials from AWS secrets manager so the hard coded credentials are never on your EC2 instance.

But wait, you say, what has permission to access secrets manager?

You add a role profile to your EC2 instance that has permission to access the specific secrets it's allowed to access in Secrets Manager.

But why not just use the EC2 role then? 

Because you can't enforce MFA. So the purpose of this code is to assume a role with MFA to perform some critical actions - without ever storing the credentials to do it on the EC2 instance - OR a developer's machine. But to run the code a developer has to provide their MFA device.

In the IAM policy and role trust policy you can also add restrictions to limit to a speciifc organization, IP address, EC2 isntance ID, account, region, etc.

## Testing your IAM role policy, user policy, and trust policy

Before you test this code, you can perform a simpler test, creating a source role and a role to assume to test MFA role assumption with the AWS CLI. You can use these two scripts which will walk you through entering the credentials and information to create and test the permissions using the AWS CLI. 

Source Profile:
https://github.com/2ndSightLab/aws-scripts/tree/main/scripts/aws-cli-source-profile

Role Profile: 
https://github.com/2ndSightLab/aws-scripts/tree/main/scripts/aws-cli-role-profile

Once you have the permissions working correctly, delete the credentials and AWS CLI profile information and test the rust application and you'll see that you can assume a role with MFA in rust the same way.
