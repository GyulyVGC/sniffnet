/// This enum defines the possible web pages to be opened.
#[derive(Debug, Clone)]
pub enum WebPage {
    /// Sniffnet's GitHub repository.
    Repo,
    /// Sniffnet's website/download page.
    WebsiteDownload,
}

impl WebPage {
    pub fn get_url(&self) -> &str {
        match self {
            WebPage::Repo => "https://github.com/GyulyVGC/sniffnet",
            WebPage::WebsiteDownload => "https://www.sniffnet.net/download/",
        }
    }
}
