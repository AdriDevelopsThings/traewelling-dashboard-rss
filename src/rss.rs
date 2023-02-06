use crate::traewelling::{DashboardResponse, BASE_URL, DashboardData};
use rss::{Channel, ChannelBuilder, ItemBuilder, Item};
use chrono_tz::Tz;

fn get_content(d: &DashboardData, timezone: Tz) -> String {
    let mut s = String::new();
    if let Some(arrival) = d.train.destination.arrival_real {
        let arrival= arrival.with_timezone(&timezone);
        s += format!("Arrival: {}<br />", arrival.format("%H:%M")).as_str();
    }
    if !d.body.is_empty() {
        s += format!("Body: {}", d.body).as_str();
    }
    s
}

impl DashboardResponse {
    pub fn to_channel(&self, timezone: String, ignore_users: Vec<&str>) -> Channel {
        let timezone: Tz = timezone.parse().unwrap();
        let items = self.data.iter()
            .filter(|d| !ignore_users.contains(&d.username.as_str()))
            .map(|d| {
            ItemBuilder::default()
             .title(Some(format!("{}: {} from {} to {}", d.username, d.train.line_name, d.train.origin.name, d.train.destination.name)))
             .link(Some(format!("{}/status/{}", BASE_URL, d.id)))
             .pub_date(Some(d.created_at.to_rfc2822()))
             .content(Some(get_content(d, timezone)))
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