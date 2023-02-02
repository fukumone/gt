use clap::{arg, Command};
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;

use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct OpenAIResponse {
    data: String,
}

#[derive(Deserialize)]
struct OpenAIResponseData {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<OpenAIResponseChoice>,
    usage: OpenAIResponseUsage,
}

#[derive(Deserialize)]
struct OpenAIResponseChoice {
    text: String,
    index: i64,
    logprobs: Option<String>,
    finish_reason: String,
}

#[derive(Deserialize)]
struct OpenAIResponseUsage {
    prompt_tokens: i64,
    completion_tokens: i64,
    total_tokens: i64,
}

#[derive(Serialize, Debug)]
struct OpenAIRequestData {
    prompt: String,
    model: String,
    temperature: f32,
    max_tokens: i32,
}

async fn fetch_openai_request(prompt: String, model: String) -> Result<String, reqwest::Error> {
    dotenv::dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = reqwest::Client::new();

    let request_data = OpenAIRequestData {
        prompt,
        model,
        temperature: 0.5,
        max_tokens: 2048,
    };

    let response = client
        .post("https://api.openai.com/v1/completions")
        .bearer_auth(api_key)
        .json(&request_data)
        .send()
        .await?
        .text()
        .await?;

    let parse_response: OpenAIResponseData = serde_json::from_str(&response).unwrap();
    let text = parse_response.choices[0].text.trim().to_string();

    Ok(text)
}

fn cli() -> Command {
    Command::new("gt")
        .about("Translate messages from terminal using OpenAI.")
        .version("0.1.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("translate")
                .about("Translates text")
                .short_flag('t')
                .arg(arg!(<TEXT> "Text to translate"))
                .args(push_args())
                .arg_required_else_help(true),
        )
}

fn push_args() -> Vec<clap::Arg> {
    vec![
        clap::Arg::new("LANGUAGE")
            .long("language")
            .short('l')
            .default_value("English")
            .help("Language to use"),
    ]
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("translate", sub_matches)) => {
            let text = sub_matches.get_one::<String>("TEXT").expect("required");
            let language = match env::var("GT_LANGUAGE") {
                Ok(val) => val,
                Err(_) => sub_matches.get_one::<String>("LANGUAGE").expect("required").to_string(),
            };

            let prompt = format!("I want you to act as an {} translator, spelling corrector and improver. I will speak to you in any language and you will detect the language, translate it and answer in the corrected and improved version of my text, in {}. I want you to replace my simplified A0-level words and sentences with more beautiful and elegant, upper level {} words and sentences. Keep the meaning same, but make them more literary. I want you to only reply the correction, the improvements and nothing else, do not write explanations. My first sentence is '{}'", language, language, language, text);
            let model = "text-davinci-003".to_string();
            let response:Result<String, reqwest::Error> = fetch_openai_request(prompt, model).await;
            match response {
                Ok(text) => println!("{}", text),
                Err(e) => println!("Error: Translation failed {}", e),
            }
        },
        _ => {
            unreachable!("Unsupported subcommand `{}`", matches.subcommand_name().unwrap())
        }

    }
}
