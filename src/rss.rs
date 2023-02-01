use crate::traewelling::{DashboardResponse, BASE_URL, DashboardData};
use rss::{Channel, ChannelBuilder, ItemBuilder, Item};

fn get_content(d: &DashboardData) -> String {
    let mut s = String::new();
    if let Some(arrival) = d.train.destination.arrival_real {
        s += format!("Arrival: {}<br />", arrival.format("%H:%M")).as_str();
    }
    if !d.body.is_empty() {
        s += format!("Body: {}", d.body).as_str();
    }
    s
}

impl From<DashboardResponse> for Channel {
    fn from(data: DashboardResponse) -> Channel {
        let items = data.data.iter().map(|d| {
           ItemBuilder::default()
            .title(Some(format!("{}: {} from {} to {}", d.username, d.train.line_name, d.train.origin.name, d.train.destination.name)))
            .link(Some(format!("{}/status/{}", BASE_URL, d.id)))
            .pub_date(Some(d.created_at.to_rfc2822()))
            .content(Some(get_content(d)))
            .build() 
        }).collect::<Vec<Item>>();
        ChannelBuilder::default()
            .title(String::from("Your Traewelling dashboard"))
            .link(String::from("https://traewelling.de/dashboard"))
            .description(String::from("An rss feed with your traewelling dashboard content"))
            .items(items)
            .build()
    }
}