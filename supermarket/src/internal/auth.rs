use reqwest::RequestBuilder;

pub trait Auth {
    fn request(&self, builder: RequestBuilder) -> RequestBuilder;
}
