use crate::structures::EmbedObject;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serenity::model::prelude::Member;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Url {
    pub raw: String,
    pub full: String,
    pub regular: String,
    pub small: String,
    pub thumb: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Link {
    pub download: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct UserLink {
    pub html: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    pub name: String,
    pub links: UserLink,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Query {
    pub urls: Url,
    pub links: Link,
    pub user: User,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct SearchResult {
    pub total: u32,
    pub total_pages: u32,
    pub results: Vec<Query>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct CatSearchResult {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct CatBreedSearchResult {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct DogSearchResult {
    pub message: String,
    pub status: String,
}

const ITEM_PER_PAGE: u32 = 10;
const CAT_API_URL: &str = "https://api.thecatapi.com/v1/images/search";
const DOG_API_URL: &str = "https://dog.ceo/api/breeds/image/random";

pub async fn get_image(
    keyword: &str,
    client: &reqwest::Client,
    member: &Member,
) -> anyhow::Result<Option<EmbedObject>> {
    let token = std::env::var("UNSPLASH_TOKEN")?;

    // Get available page count for the keyword
    let response = client
        .get(&format!(
            "https://api.unsplash.com/search/photos?client_id={}&query={}&page=1",
            &token, keyword
        ))
        .send()
        .await?;
    let search_result: SearchResult = response.json().await?;
    let (total, total_pages) = (search_result.total, search_result.total_pages);

    // If there are no results, returns an empty Vec.
    if total == 0 {
        return Ok(None);
    }

    // Limit to the first 25% of pages and get a random page number from it.
    let upper_page_limit = ((total_pages as f32) * 0.25_f32).ceil();
    let random_page_number = thread_rng().gen_range(0_u32..(upper_page_limit as u32) + 1_u32);

    // Get image data of a specific page and get a specific image from that page.
    let response = client
        .get(&format!(
            "https://api.unsplash.com/search/photos?client_id={}&query={}&page={}",
            &token, keyword, random_page_number
        ))
        .send()
        .await?;
    let search_result: SearchResult = response.json().await?;
    let modulo = search_result.total % ITEM_PER_PAGE;
    let item_no = {
        let mut rng = thread_rng();
        if random_page_number == total_pages || random_page_number <= 1 {
            rng.gen_range(0..modulo as usize)
        } else {
            rng.gen_range(0..ITEM_PER_PAGE as usize)
        }
    };

    // Get actual query result.
    let query = search_result.results[item_no].clone();

    // Construct the embed.
    let description = format!(
        "Here is your result for **{}**!\nPhoto by [{}]({}) on [Unsplash]({})",
        keyword,
        query.user.name,
        query.user.links.html,
        "https://unsplash.com/?utm_source=Taiga&utm_medium=referral"
    );

    let author_name = member.nick.clone().unwrap_or(member.user.name.clone());

    Ok(Some(
        EmbedObject::new()
            .title("Download Link")
            .description(&description)
            .url(&query.links.download)
            .color(0xdfd16c)
            .image(&query.urls.regular, None, None, None)
            .author(&author_name, member.user.avatar_url(), None, None),
    ))
}

pub async fn get_cat_image(
    keyword: &str,
    client: &reqwest::Client,
    member: &Member,
) -> anyhow::Result<Option<EmbedObject>> {
    let token = std::env::var("CAT_TOKEN")?;

    if keyword.is_empty() {
        // Get a random cat picture from the Cat API.
        let description = format!(
            "Here is your result for **cat**!\nPhoto by [The Cat API]({})",
            "https://thecatapi.com/"
        );

        let mut result = fetch_cat_image(CAT_API_URL, client, member).await?;
        result.description = Some(description);
        Ok(Some(result))
    } else {
        // Try searching for the breed.
        let response = client
            .get(&format!(
                "https://api.thecatapi.com/v1/breeds/search?q={}",
                keyword
            ))
            .header("x-api-key", &token)
            .send()
            .await?;

        let search_result: Vec<CatBreedSearchResult> = response.json().await?;

        // If there's a search result, use that breed ID to get cat pictures.
        if let Some(s) = search_result.get(0) {
            let url = format!(
                "https://api.thecatapi.com/v1/images/search?breed_ids={}",
                &s.id
            );

            let description = format!(
                "Here is your result for **{}**!\nPhoto by [The Cat API]({})",
                keyword, "https://thecatapi.com/"
            );

            let mut result = fetch_cat_image(&url, client, member).await?;
            result.description = Some(description);
            Ok(Some(result))
        } else {
            // If there are no results, just return a random cat.
            let description = format!(
                "I'm sorry, but I can't find any picture of **{}**!\nHere is a random cat for you.\nPhoto by [The Cat API]({})",
                keyword,
                "https://thecatapi.com/"
            );

            let mut result = fetch_cat_image(CAT_API_URL, client, member).await?;
            result.description = Some(description);
            Ok(Some(result))
        }
    }
}

pub async fn get_dog_image(
    keyword: &str,
    client: &reqwest::Client,
    member: &Member,
) -> anyhow::Result<Option<EmbedObject>> {
    if keyword.is_empty() {
        // Get a random cat picture from the Dog API.
        let description = format!(
            "Here is your result for **dog**!\nPhoto by [Dog API]({})",
            "https://dog.ceo/dog-api/"
        );

        let result = fetch_dog_image(DOG_API_URL, client, member)
            .await?
            .map(|e| e.description(&description));
        Ok(result)
    } else {
        // Try searching for the breed.
        let url = format!("https://dog.ceo/api/breed/{}/images/random", keyword);
        let search_result = fetch_dog_image(&url, client, member).await?;

        // If there's a result, construct an Embed and send that image.
        if let Some(mut result) = search_result {
            let description = format!(
                "Here is your result for **{}**!\nPhoto by [Dog API]({})",
                keyword, "https://dog.ceo/dog-api/"
            );
            result.description = Some(description);
            Ok(Some(result))
        } else {
            // If there are no results, just return a random dog.
            let description = format!(
                "I'm sorry, but I can't find any picture of **{}**!\nHere is a random dog for you.\nPhoto by [Dog API]({})",
                keyword,
                "https://dog.ceo/dog-api/"
            );
            let result = fetch_dog_image(DOG_API_URL, client, member)
                .await?
                .map(|e| e.description(&description));
            Ok(result)
        }
    }
}

async fn fetch_cat_image(
    url: &str,
    client: &reqwest::Client,
    member: &Member,
) -> anyhow::Result<EmbedObject> {
    let token = std::env::var("CAT_TOKEN")?;
    let author_name = member.nick.clone().unwrap_or(member.user.name.clone());

    let response = client.get(url).header("x-api-key", &token).send().await?;

    let search_result: Vec<CatSearchResult> = response.json().await?;
    let result = &search_result[0];

    Ok(EmbedObject::new()
        .title("Download Link")
        .url(&result.url)
        .color(0xdfd16c)
        .image(&result.url, None, None, None)
        .author(&author_name, member.user.avatar_url(), None, None))
}

async fn fetch_dog_image(
    url: &str,
    client: &reqwest::Client,
    member: &Member,
) -> anyhow::Result<Option<EmbedObject>> {
    let author_name = member.nick.clone().unwrap_or(member.user.name.clone());
    let response = client.get(url).send().await?;
    let search_result: DogSearchResult = response.json().await?;

    if search_result.status.as_str() != "error" {
        Ok(Some(
            EmbedObject::new()
                .title("Download Link")
                .url(&search_result.message)
                .color(0xdfd16c)
                .image(&search_result.message, None, None, None)
                .author(&author_name, member.user.avatar_url(), None, None),
        ))
    } else {
        Ok(None)
    }
}
