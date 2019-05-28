
extern crate reqwest;
extern crate chrono;
extern crate serde;

use std::env;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

static API_BASE: &str = "https://api.buildkite.com/v2";

//////////////////////////////////////////////////////////
/////////////////////// public api ///////////////////////
//////////////////////////////////////////////////////////

pub fn list_organizations() -> Result<Vec<Organization>, reqwest::Error> {
    let client = reqwest::Client::new();

    let request = client.get(build_url("/organizations").as_str())
        .bearer_auth(env_token());

    let mut response: reqwest::Response = request.send()?;
    response.json::<Vec<Organization>>()
}

pub fn list_builds() -> Result<Vec<Build>, reqwest::Error> {
    let client = reqwest::Client::new();

    let request = client.get(build_url("/builds").as_str())
        .bearer_auth(env_token());

    let mut response: reqwest::Response = request.send()?;
    response.json::<Vec<Build>>()
}

// fn get<'a, A>(url: &str) -> Result<A, reqwest::Error> 
// where A: Deserialize<'a> + Serialize {
//     let client = reqwest::Client::new();

//     let request = client.get(build_url(url).as_str())
//         .bearer_auth(env_token());

//     let mut response: reqwest::Response = request.send()?;
//     response.json()
// }

fn env_token() -> String {
    env::var("BUILDKITE_TOKEN").unwrap_or("<unspecified>".to_owned())
}

/**
 * {
    "id": "eb7bddc1-ce14-421e-aec4-3cbe83d61669",
    "url": "https://api.buildkite.com/v2/organizations/lyft",
    "web_url": "https://buildkite.com/lyft",
    "name": "Lyft",
    "slug": "lyft",
    "agents_url": "https://api.buildkite.com/v2/organizations/lyft/agents",
    "emojis_url": "https://api.buildkite.com/v2/organizations/lyft/emojis",
    "created_at": "2018-04-13T16:04:30.015Z",
    "pipelines_url": "https://api.buildkite.com/v2/organizations/lyft/pipelines"
  }
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Organization {
    id: String,
    url: String,
    name: String,
    slug: String,
    agents_url: String,
    pipelines_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Build {
    id: String,
    url: String,
    number: u64,
    commit: String,
    branch: String,
    created_at: String,
    scheduled_at: String,
    started_at: String,
    finished_at: String,
}

//////////////////////////////////////////////////////////
////////////////////// private api ///////////////////////
//////////////////////////////////////////////////////////

fn build_url(path: &str) -> String {
    format!("{}{}", API_BASE, path)
}