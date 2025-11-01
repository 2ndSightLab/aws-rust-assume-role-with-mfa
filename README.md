# aws-rust-assume-role-with-mfa

This repository contains a test to assume a role with MFA using the AWS rust sdk.

## Trait that can be used in any application
I created a trait that can be reused in any application as demonstrated by the main.rs module. You can create a trait with a library that can be pulled into any application to handle your authentiation properly.

## What it does

What this code does is asks you to input the values neccessary to assume a role with MFA and the rust code assumes the role. It is a simple test to make sure I have crates and code that works.

The main.rs applicaiton implements the trait which handles the role assumption. Then main.rs lists the s3 buckets in the account and region of the assumed role.

## Requiring MFA in policies nad other restrictions

In the IAM policy and role trust policy you can also add restrictions to limit to a speciifc organization, IP address, EC2 isntance ID, account, region, etc.

https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition.html

## Why assume a role with MFA in an application?

Why would you want to do this? Well, you probably don't want to enter your creds over and over again so you probably don't want to do exactly what this code is doing. What you can do though is pull the credentials from AWS secrets manager so the hard coded credentials are never on your EC2 instance.

But wait, you say, what has permission to access secrets manager?

You add a role profile to your EC2 instance that has permission to access the specific secrets it's allowed to access in Secrets Manager.

But why not just use the EC2 role then? 

Because you can't enforce MFA. So the purpose of this code is to assume a role with MFA to perform some critical actions - without ever storing the credentials to do it on the EC2 instance - OR on a developer's machine. But to run the appliation, a developer has to provide their MFA device each time the application runs.

## An example

I perform penetration tests on customer accounts. I don't want the role a customer gives me to be abused by an attacker. I do my best to ensure only I can assume the role when running penetration test tools. I ask them to create a role which requires MFA and an external ID which is unique for each test. I also provide them IP addresses that I will be using for the test, which are different for each test, and the account number and user taht needs to assume the role.

When I run my tools, I have to be coming from the correct IP address with the correct uesr in the correct account and provide a one time use MFA token to start the session. If an attacker were to get access to a particular session, they would need to enter MFA a again to start a new session or refresh the existing session. 

That makes it more difficult to use the credentials the customer has given me to abuse their resources.

Do your vendors offer you the ability to do that to prevent supply chain and confused deputy attacks? They should.

Need a pentest?
https://2ndSightLab.com

## NOT PRODUCTION CODE
This is NOT production code. This simply shows the neccessary libraries and such and is a working simple to get you started. I asked the AI engines to do things securely but then through the rounds of revsions I'm not sure if this code is properly using the secrecy crate. That would ensure the values are only available in memory for a short time when being used. You likely don't want to have someone entring all these values all the time. This was just a test to prove that I could assume a role with MFA and other restrictions using rust.

## Testing your IAM role policy, user policy, and trust policy

Before you test this code, you can perform a simpler test, creating a source role and a role to assume to test MFA role assumption with the AWS CLI. You can use these two scripts which will walk you through entering the credentials and information to create and test the permissions using the AWS CLI. 

Source Profile:
https://github.com/2ndSightLab/aws-scripts/tree/main/scripts/aws-cli-source-profile

Role Profile: 
https://github.com/2ndSightLab/aws-scripts/tree/main/scripts/aws-cli-role-profile

Once you have the permissions working correctly, delete the credentials and AWS CLI profile information and test the rust application and you'll see that you can assume a role with MFA in rust the same way.

## Other considerations

* AWS sessions must be at least 15 minutes long last time I checked. So you're sessions will be hanging around at least that long.
* AWS does not offer a way to terminate IAM sessions (not really).
* You can set a policy to disallow sessions over a certain age from performing actions, but what if you have some older sessions that need to keep running while others are terminated?

Some things to consider if you want to run a sensitive action and then completely terminate the associated session when the session is complete.

* I am new to Rust. I just learned it a month ago using Amazon Q CLI (though I have 30 years of programming experience).
* I have not fully reviewed this code.
* I couldn't easily find which were the most up to date crates for the AWS Rust SDK for STS. I hope these are correct but not sure honestly.
* I used a combination of Amazon Q CLI, console, and Google Gemini to get this working. Gemini couldn't get it right. Mostly used Q. If anything is wrong I blame Q :-)
* But I probably won't be done by now if I hadn't used Amazon Q...the errors were tricky as the libraries seemed to have changed a lot over time.

## Why not use IAM Identity Center?

I don't want to use a browser. The OAuth device code flow is too often the source of phishing attacks. An attacker can also start a session and potentially trick me to completing the login in my browser. Hopefully this method is harder to phish, the nothing is foolproof.
