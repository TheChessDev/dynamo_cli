use std::io::stdout;

use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{Client, Error};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::Terminal;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&config);

    let table_names = list_tables(&client).await?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let items: Vec<ListItem> = table_names
            .iter()
            .map(|name| ListItem::new(name.clone()))
            .collect();

        let list =
            List::new(items).block(Block::default().borders(Borders::ALL).title("Collections"));

        f.render_widget(list, f.size());
    })?;

    Ok(())
}

async fn list_tables(client: &Client) -> Result<Vec<String>, Error> {
    let resp = client.list_tables().send().await?;
    let table_names = resp.table_names().to_vec();
    Ok(table_names)
}
