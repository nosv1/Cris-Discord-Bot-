use std::str::Split;

use regex::Regex;

use serenity::prelude::Context;
use serenity::model::prelude::Message;

use super::super::tepcott::submit_quali_time;

pub async fn submit(context: &Context, msg: &Message, mut split_message: Split<'_, &str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Handling submit command...");

    let time = split_message.next();
    let link = split_message.next();

    if time.is_none() || link.is_none() {                                       // check valid number of arguments
        let _ = msg.channel_id.say(
            &context.http, 
            "Invalid number of arguments. Please use the following format: !submit <MM:SS.mmm> <weblink to your video/screenshot>"
        )
            .await;
        return Ok(());
    }

    
    let time_format = Regex::new(r"^\d{1,2}:\d{2}\.\d{3}$").unwrap();
    if !time_format.is_match(time.unwrap()) {                              // check valid time format
        let _ = msg.channel_id.say(
            &context.http, 
            "Invalid time format. Please use the following format: MM:SS.mmm"
        )
            .await;
        return Ok(());
    }

    let link_format = Regex::new(r"^(https?://)?(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)$").unwrap();
    if !link_format.is_match(link.unwrap()) {                              // check valid link format
        // return error message if invalid link format
        let _ = msg.channel_id.say(
            &context.http,
            "Invalid link format. Please provide a valid weblink.")
            .await;
        return Ok(());
    }

    // if valid time and link,
    let _ = submit_quali_time(
        &msg.author.id.0.to_string().as_str(),
        time.unwrap(), 
        link.unwrap()
    ).await;

    Ok(())
    

}