#![crate_name = "rusoto"]
#![crate_type = "lib"]
#![allow(dead_code)]

//! # Rusoto
//!
//! Rusoto is an SDK for [Amazon Web Services](http://aws.amazon.com/).  It follows best practices
//! for [AWS Credentials](https://blogs.aws.amazon.com/security/post/Tx3D6U6WSFGOK2H/A-New-and-Standardized-Way-to-Manage-Credentials-in-the-AWS-SDKs).
//!
//! The library is designed with use on AWS EC2 instances in mind but can be used as an
//! [AWS CLI](https://aws.amazon.com/cli/) replacement.
//!
//! ## Credentials
//!
//! Credentials are sourced from environment variables, AWS credentials file and IAM instance profiles,
//! in that order.  IAM instance profile credentials are refreshed automatically as needed.
//!
//! ## Supported services
//!
//! * SQS
//! * S3 (partial implementation)
//!
//! ## Requests and request signing
//!
//! Rusoto uses [AWS Signature 4](http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html)
//! to sign requests.

extern crate time;
extern crate xml;
extern crate hyper;
extern crate openssl;
extern crate url;
extern crate rustc_serialize as serialize;
extern crate regex;
extern crate crypto;

#[macro_use] pub mod params;
#[macro_use] pub mod signature;
pub mod credentials;
pub mod error;
pub mod sqs;
pub mod s3;
pub mod xmlutil;
pub mod regions;
pub mod request;
