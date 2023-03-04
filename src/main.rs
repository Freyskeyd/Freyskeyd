use std::process::Command;

use inquire::{ui::RenderConfig, Select};

const GITHUB: &str = "Open my Github profile";
const GITHUB_PROFILE_URL: &str = "https://github.com/Freyskeyd";

const MAIL: &str = "Send me an email";
const MAIL_URL: &str = "mailto:freyskeyd@gmail.com";

const TWITCH: &str = "Open my Twitch channel";
const TWITCH_URL: &str = "https://www.twitch.tv/freyskeyd";

const TWITTER: &str = "Open my Twitter channel";
const TWITTER_URL: &str = "https://twitter.com/freyskeyd";

fn main() {
    let welcome = r#"
    Hello, I'm Freyskeyd!

    Thank's for installing my CLI.

    "#;

    println!("{}", welcome);

    let options = vec![GITHUB, MAIL, TWITCH, TWITTER];

    let actions = [
        [GITHUB, GITHUB_PROFILE_URL],
        [MAIL, MAIL_URL],
        [TWITCH, TWITCH_URL],
        [TWITTER, TWITTER_URL],
    ];

    let ans = Select::new("Feel free to pick one option:", options)
        .with_vim_mode(true)
        .prompt();

    match ans {
        Ok(choice) => {
            if let Some(&[_, url]) = actions.iter().find(|&[action, _]| *action == choice) {
                if webbrowser::open(url).is_err() {
                    println!("Can't open web page :'(");
                }
            }
        }
        Err(_) => println!("There was an error, please try again"),
    }
}
