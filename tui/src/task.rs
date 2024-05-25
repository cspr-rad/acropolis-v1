use crate::app::App;
use reqwest::Error;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Vote {
    choice: String,
    government_public_key: Vec<u8>,
    public_identity: String,
}

pub async fn fetch_and_update(app: Arc<Mutex<App>>) -> Result<(), Error> {
    loop {
        // Fetch data from the API
        if let Err(e) = fetch_data(app.clone()).await {
            handle_error(app.clone(), e).await;
        }

        // Sleep for a defined interval before fetching data again
        sleep(Duration::from_secs(5)).await;
    }
}

async fn fetch_data(app: Arc<Mutex<App>>) -> Result<(), Error> {
    let elections: Vec<String> = reqwest::get("http://127.0.0.1:8080/fetch_elections")
        .await?
        .json()
        .await?;

    {
        let mut app = app.lock().unwrap();
        app.elections = elections;
        app.error = None; // Clear any previous errors
    } // Drop the MutexGuard here

    let tally = {
        let selected = {
            let app = app.lock().unwrap();
            app.list_elections.selected()
        };

        if let Some(selected) = selected {
            let election_name = {
                let app = app.lock().unwrap();
                app.elections[selected].clone()
            }; // Clone the election name to avoid borrowing issues

            // Fetch votes for the selected election
            let votes: Vec<Vote> =
                reqwest::get(format!("http://127.0.0.1:8080/fetch_votes/{election_name}"))
                    .await?
                    .json()
                    .await?;
            let mut tally: std::collections::HashMap<String, u64> =
                std::collections::HashMap::new();

            for vote in votes {
                *tally.entry(vote.choice).or_insert(0) += 1;
            }
            tally
        } else {
            std::collections::HashMap::new()
        }
    };

    {
        let mut app = app.lock().unwrap();
        app.tally = tally;
    }

    Ok(())
}

async fn handle_error(app: Arc<Mutex<App>>, e: Error) {
    let error_message = format!("Data might be old or unavailable, {}", e);
    let mut app = app.lock().unwrap();
    app.error = Some(error_message);
}
