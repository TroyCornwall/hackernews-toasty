#![windows_subsystem = "windows"]
use std::error::Error;
use windows::{
    Data::Xml::Dom::XmlDocument,
    UI::Notifications::ToastNotification,
    UI::Notifications::ToastNotificationManager,
};
use windows::core::{HSTRING};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // let s = setup();
    let mut max = 0;
    loop{
        let new_max = create_toast_if_new_post(max).await;
        match new_max {
            Ok(parsed) => max = parsed,
            Err(_) => {}
        }
        tokio::time::sleep(std::time::Duration::from_secs(300)).await;
    }
}

async fn create_toast_if_new_post(max: i32) -> Result<i32, Box<dyn Error>>{
    let mut new_max = max;
    let posts = get_posts().await?;
    let max_post = posts.iter().max().unwrap();
    if max_post.gt(&max) {
        let item = get_item(max_post).await?;
        let _ = toast(item.title, item.url, max_post);
        new_max = max_post.clone();
    }
    Ok(new_max)
}

async fn get_posts() -> reqwest::Result<Vec<i32>> {
    Ok(reqwest::Client::new()
        .get("https://hacker-news.firebaseio.com/v0/topstories.json?print=pretty")
        .send()
        .await?
        .json::<Vec<i32>>()
        .await?)
}
async fn get_item(item_num: &i32) -> reqwest::Result<HNItem> {
    Ok(reqwest::Client::new()
        .get(format!("https://hacker-news.firebaseio.com/v0/item/{}.json", item_num))
        .send()
        .await?
        .json::<HNItem>()
        .await?)
}
fn toast(msg: String, url: String, item: &i32) -> Result<(), Box<dyn Error>> {
    let toast_xml = XmlDocument::new()?;
    let mut hn_url : String = "https://news.ycombinator.com/item?id=".to_owned(); //hint-crop="circle"
    hn_url.push_str(&*item.to_string());
    toast_xml.LoadXml(
        &HSTRING::from(
            format!(r#"<toast duration="long">
                <visual>
                    <binding template="ToastGeneric">
                        <text id="1">{}</text>
                         <image src="C:\Dev\hackernews-toasty\src\hackernews.png" placement="appLogoOverride"  />
                    </binding>
                </visual>
                <audio silent="true" />
                <actions>
	                <action content="Article" arguments="{}" activationType="protocol" />
                    <action content="HackerNews" arguments="{}" activationType="protocol" />
                </actions>
            </toast>"#
            , msg, url, hn_url
            ))).unwrap();
    let toast_template = ToastNotification::CreateToastNotification(&toast_xml)?;

    // If you have a valid app id, (ie installed using wix) then use it here.
    let toast_notifier = ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(
        "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe",
    ))?;

    // Show the toast.
    // Note this returns success in every case, including when the toast isn't shown.
    toast_notifier.Show(&toast_template).expect("TODO: panic message");
    Ok(())
}


#[derive(Serialize, Deserialize, Debug)]
struct HNItem {
    pub by: String,
    pub descendants: i64,
    pub id: i64,
    pub kids: Vec<i64>,
    pub score: i64,
    pub time: i64,
    pub title: String,
    pub r#type: String,
    pub url: String,
}