use reqwest::Method;
// use anyhow::Result;
use serde::Deserialize;
use ureq::Agent;
use url::Url;
// https://newsapi.org/v2/everything?q=Iphone&from=2022-12-10&sortBy=popularity&apiKey=d45b03e5d78642e89229e742d0292f2f
const BASE_URL: &str = "https://newsapi.org/v2";

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed fetching articles")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Failed converting response to string")]
    FailedResponseToString(#[from] std::io::Error),
    #[error("Article Parsing failed")]
    ArticleParseFailed(#[from] serde_json::Error),
    #[error("Url parsing failed")]
    UrlParsing(#[from] url::ParseError),
    #[error("Request failed: {0}")]
    BadRequest(&'static str),
    // #[error("Async request failed")]
    // AsyncRequestFailed(#[from] reqwest::Error),
}

#[derive(Deserialize, Debug)]
pub struct NewsAPIResponse {
    status: String,
    code: Option<String>,
    articles: Vec<Article>,
}

impl NewsAPIResponse {
    pub fn articles(&self) -> &Vec<Article> {
        &self.articles
    }
}

#[derive(Debug, Deserialize)]
pub struct Article {
    title: String,
    url: String,
    description: Option<String>,
}

impl Article {
    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    pub fn url(&self) -> &str {
        self.url.as_ref()
    }

    pub fn desc(&self) -> Option<&String> {
        self.description.as_ref()
    }
}

pub enum Endpoint {
    TopHeadlines,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadlines => "top-headlines".to_string(),
        }
    }
}

pub enum Country {
    Us,
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Self::Us => "us".to_string(),
        }
    }
}

pub struct NewsAPI {
    api_key: String,
    endpoint: Endpoint,
    country: Country,
}

impl NewsAPI {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            endpoint: Endpoint::TopHeadlines,
            country: Country::Us,
        }
    }

    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut NewsAPI {
        self.endpoint = endpoint;
        self
    }

    pub fn country_mut(&mut self, country: Country) -> &mut NewsAPI {
        self.country = country;
        self
    }
    fn prepare_url(&self) -> Result<String, NewsApiError> {
        let mut url = Url::parse(BASE_URL)?;
        url.path_segments_mut()
            .unwrap()
            .push(&self.endpoint.to_string());
        let country = format!("country={}", self.country.to_string());
        url.set_query(Some(&country));
        Ok(url.to_string())
    }

    pub fn fetch(&self) -> Result<NewsAPIResponse, NewsApiError> {
        let url = self.prepare_url()?;
        println!("url: {}", url);
        let proxy = ureq::Proxy::new("http://127.0.0.1:7890");
        let agent: Agent = ureq::AgentBuilder::new()
            .proxy(proxy.unwrap())
            .build();
        let req = agent.get(&url)
        .set("Authorization", &self.api_key)
        .set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/84.0.4146.4 Safari/537.36");
        let response: NewsAPIResponse = req.call().unwrap().into_json()?;
        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_err(response.code)),
        }
    }

    pub async fn fetch_async(&self) -> Result<NewsAPIResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let client = reqwest::Client::new();
        let request = client
            .request(Method::GET, url)
            .header("Authorization", &self.api_key)
            .build()
            .map_err(|e| NewsApiError::RequestFailed(e))?;

        let response: NewsAPIResponse = client
            .execute(request)
            .await?
            .json()
            .await
            .map_err(|e| NewsApiError::RequestFailed(e))?;

        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_err(response.code)),
        }
    }
}

fn map_response_err(code: Option<String>) -> NewsApiError {
    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisabled" => NewsApiError::BadRequest("Your API key has been disabled"),
            _ => NewsApiError::BadRequest("Unknown error"),
        }
    } else {
        NewsApiError::BadRequest("Unknown error")
    }
}

#[cfg(test)]
mod tests {
    use ureq::Agent;
    use std::time::Duration;
    use super::*;
    #[test]
    fn it_works() {
        let url = "https://newsapi.org/v2/everything?q=Apple&from=2022-12-13&sortBy=popularity&apiKey=d45b03e5d78642e89229e742d0292f2f";
        // println!("url: {}", url);
        // let url = "https://spa3.scrape.center/api/movie/?limit=10&offset=0";
        let proxy = ureq::Proxy::new("http://127.0.0.1:7890");
        let agent: Agent = ureq::AgentBuilder::new()
            // .timeout_read(Duration::from_secs(5))
            // .timeout_write(Duration::from_secs(5))
            .proxy(proxy.unwrap())
            .build();
        let req = agent.get(url).set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/84.0.4146.4 Safari/537.36");
        let response = req.call().unwrap().into_string().unwrap();
        println!("{}", response);
    }
}
