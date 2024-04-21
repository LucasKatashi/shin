use futures::stream::{self, StreamExt};
use text_colorizer::*;
use regex::Regex;
use clap::Parser;
use reqwest;
use std::fs;

#[derive(Debug, Parser)]
struct Arguments {
    #[arg(short, long)]
    target: String,

    #[arg(short, long)]
    wordlist: String,

    #[arg(long, default_value_t = 1)]
    threads: usize,

    #[arg(long)]
    mc: bool,
}

fn reader(path: &str) -> Vec<String> {
    let wordlist = fs::read_to_string(&path).expect("Error trying to read the text file.");
    let content = wordlist.split_whitespace().map(|s| s.to_string()).collect();
    return content
}

async fn fuzz(target: &str, wordlist: Vec<String>, threads: usize, mc: bool) { 
    let client = reqwest::Client::new();

    stream::iter(wordlist)
        .for_each_concurrent(Some(threads as usize), |each| {
            let client = &client;
            let target = target.to_string();
            let mc = mc;
            async move {
                let url = format!("{}/{}", &target, each);
                let response = client.get(&url).send().await;

                if let Ok(res) = response {
                    if res.status() == 200 {
                        println!(
                            "[{}] /{} ({}) - {}",
                            "+".blue().bold(),
                            each,
                            res.status().to_string().green().bold(),
                            url
                        );
                    } else if res.status() == 302 && !mc {
                        println!(
                            "[{}] /{} ({}) - {}",
                            "+".yellow().bold(),
                            each,
                            res.status().to_string().yellow().bold(),
                            url
                        );
                    } else if !mc {
                        println!(
                            "[{}] /{} ({})",
                            "!".red().bold(),
                            each,
                            res.status().to_string().red().bold()
                        );
                    }
                }
            }
        })
        .await;
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut args = Arguments::parse();
    
    if !Regex::new(r"ˆhttps://?").unwrap().is_match(&args.target) {
        args.target = format!("https://{}", args.target);
    }
    
    let wordlist = reader(&args.wordlist);

    let banner = r#"
      ::::::::  :::    ::: ::::::::::: ::::    ::: 
    :+:    :+: :+:    :+:     :+:     :+:+:   :+:  
   +:+        +:+    +:+     +:+     :+:+:+  +:+   
  +#++:++#++ +#++:++#++     +#+     +#+ +:+ +#+    
        +#+ +#+    +#+     +#+     +#+  +#+#+#     
#+#    #+# #+#    #+#     #+#     #+#   #+#+#      
########  ###    ### ########### ###    ####       
                                           katashi"#;

    println!("{}", banner.bright_magenta());

    fuzz(&args.target, wordlist, args.threads, args.mc).await;

    Ok(())
}