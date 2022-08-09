use isahc::prelude::*;

pub const DEMO_URL: &str = "https://www.phoronix.com/rss.php";

pub async fn fetch_channel(url: &str) -> Result<Channel, FetchError> {
    let mut response = isahc::get_async(url).await?;
    let content = response.bytes().await?;
    let channel = rss::Channel::read_from(content.as_slice())?;

    let mut image = None;
    if let Some(img) = &channel.image {
        let mut response = isahc::get_async(&img.url).await?;
        let data = response.bytes().await?;
        image = Some(data);
    }

    Ok(Channel::from(channel, image))
}

#[derive(Debug)]
pub enum FetchError {
    Network(isahc::Error),
    Traffic(std::io::Error),
    Parsing(rss::Error),
}
impl From<isahc::Error> for FetchError {
    fn from(error: isahc::Error) -> Self {
        FetchError::Network(error)
    }
}
impl From<std::io::Error> for FetchError {
    fn from(error: std::io::Error) -> Self {
        FetchError::Traffic(error)
    }
}
impl From<rss::Error> for FetchError {
    fn from(error: rss::Error) -> Self {
        FetchError::Parsing(error)
    }
}

#[derive(Debug)]
pub struct Channel {
    pub title: String,
    pub image: Option<Vec<u8>>,
    pub link: String,
    pub description: String,
    pub items: Vec<Item>,
}

impl Channel {
    fn from(channel: rss::Channel, image: Option<Vec<u8>>) -> Self {
        let items = channel
            .items
            .into_iter()
            .map(Item::try_from)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();

        Channel {
            title: channel.title,
            image,
            link: channel.link,
            description: channel.description,
            items,
        }
    }
}

#[derive(Debug)]
pub struct Item {
    pub title: String,
    pub link: String,
}

#[derive(Debug, Clone, Copy)]
pub enum InvalidItemError {
    NoTitle,
    NoLink,
}

impl TryFrom<rss::Item> for Item {
    type Error = InvalidItemError;

    fn try_from(item: rss::Item) -> Result<Self, Self::Error> {
        if item.title.is_none() {
            return Err(InvalidItemError::NoTitle);
        }
        if item.link.is_none() {
            return Err(InvalidItemError::NoLink);
        }

        Ok(Item {
            title: item.title.unwrap(),
            link: item.link.unwrap(),
        })
    }
}
