extern crate google_sheets4 as sheets4;
use serde_json::Value;
use sheets4::api::{ValueRange, Sheet, NamedRange};
use sheets4::hyper::client::HttpConnector;
use sheets4::hyper_rustls::HttpsConnector;
use sheets4::oauth2::{ApplicationSecret, InstalledFlowAuthenticator};
use sheets4::{Result, Error};
use sheets4::{Sheets, oauth2, hyper, hyper_rustls};

use serde::Deserialize;

use std::collections::HashMap;
use std::fs;

// pub const GUILD_ID: &str = "450289520009543690";                 // TEPCOTT
// pub const SUBMISSIONS_CHANNEL_ID: &str = "1058730856073670656";  // #submissions

pub const GUILD_ID: &str = "789181254120505386";                 // Phyner
pub const SUBMISSIONS_CHANNEL_ID: &str = "789182513633427507";   // #private-testing

const CLIENT_SECRET: &str = "src/servers/tepcott/google_api/client_secret.json";  // src/servers/tepcott/tepcott-30c3532764ae.json
const SEASON_7_SPREADSHEET_KEY: &str = "1axNs6RyCy8HE8AEtH5evzBt-cxQyI8YpGutiwY8zfEU";

async fn get_sheets_client() -> Result<Sheets<HttpsConnector<HttpConnector>>>{

    let cwd = std::env::current_dir().unwrap();
    let google_apis_secret_path = std::path::Path::new(CLIENT_SECRET);

    // secret
    let secret: oauth2::ApplicationSecret = oauth2::read_application_secret(google_apis_secret_path)
        .await
        .expect("Error reading client secret file");
        
    // authenticator
    let auth = InstalledFlowAuthenticator::builder(secret, oauth2::InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk("src/servers/tepcott/google_api/token.json")
        .build()
        .await
        .expect("Error building authenticator");

    // sheets client
    let sheets_client = Sheets::new(
        hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build()), 
        auth
    );

    Ok(sheets_client)
}


pub async fn submit_quali_time(user_id: &str, lap_time: &str, link: &str) {

    println!("Submitting quali time for user {} with lap time {} and link {}", user_id, lap_time, link);

    let sheets_client = get_sheets_client()
        .await
        .unwrap();

    let spreadsheet = sheets_client.spreadsheets().get(SEASON_7_SPREADSHEET_KEY)
        .include_grid_data(false)
        .doit()
        .await
        .unwrap()
        .1;
        
    let sheets: HashMap<String, Sheet> = spreadsheet.sheets.as_ref()
        .unwrap()
        .iter()
        .map(|sheet| (sheet.properties.as_ref().unwrap().title.as_ref().unwrap().clone(), sheet.clone()))
        .collect::<HashMap<String, Sheet>>();

    let named_ranges = spreadsheet.named_ranges.as_ref()
        .unwrap()
        .iter()
        .map(|named_range| (named_range.name.clone().unwrap(), named_range.clone()))
        .collect::<std::collections::HashMap<String, NamedRange>>();

    let quali_sheet = sheets.get("qualifying").unwrap();

    let quali_drivers_named_range = named_ranges.get("qualifying_drivers").unwrap();
    let quali_lap_times_named_range = named_ranges.get("qualifying_lap_times").unwrap();

    let quali_drivers_range = quali_drivers_named_range.range.as_ref().unwrap();
    let quali_lap_times_range = quali_lap_times_named_range.range.as_ref().unwrap();

    let quali_values = sheets_client.spreadsheets().values_batch_get(spreadsheet.spreadsheet_id.as_ref().unwrap())
        .value_render_option("FORMATTED_VALUE")
        .add_ranges(&format!("{}!R{}C{}:R{}C{}", 
            "qualifying", 
            quali_drivers_range.start_row_index.as_ref().unwrap() + 1, 
            quali_drivers_range.start_column_index.as_ref().unwrap() + 1, 
            quali_drivers_range.end_row_index.as_ref().unwrap() + 1, 
            quali_drivers_range.end_column_index.as_ref().unwrap() + 1
        ))
        .add_ranges(&format!("{}!R{}C{}:R{}C{}", 
            "qualifying", 
            quali_lap_times_range.start_row_index.as_ref().unwrap() + 1, 
            quali_lap_times_range.start_column_index.as_ref().unwrap() + 1, 
            quali_lap_times_range.end_row_index.as_ref().unwrap() + 1, 
            quali_lap_times_range.end_column_index.as_ref().unwrap() + 1
        ))
        .major_dimension("ROWS")
        .doit()
        .await
        .unwrap()
        .1;

}