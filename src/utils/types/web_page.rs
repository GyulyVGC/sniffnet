/// This enum defines the possible web pages to be opened.
#[derive(Debug, Clone)]
pub enum WebPage {
    /// Sniffnet's GitHub repository.
    Repo,
    // /// Sniffnet's website main page.
    // Website,
    /// Sniffnet's website/download page.
    WebsiteDownload,
    /// Sniffnet's website/news page.
    WebsiteNews,
    /// Sniffnet's website/sponsor page.
    WebsiteSponsor,
    /// Sniffnet Roadmap
    Roadmap,
    /// Sniffnet issues on GitHub
    Issues,
    /// Sniffnet issue #60 on GitHub
    IssueLanguages,
    /// Sniffnet Wiki
    Wiki,
    /// My GitHub profile
    MyGitHub,
}

impl WebPage {
    pub fn get_url(&self) -> &str {
        match self {
            WebPage::Repo => "https://github.com/GyulyVGC/sniffnet",
            // WebPage::Website => "https://www.sniffnet.net",
            WebPage::WebsiteSponsor => "https://www.sniffnet.net/sponsor",
            WebPage::WebsiteDownload => "https://www.sniffnet.net/download",
            WebPage::WebsiteNews => "https://www.sniffnet.net/news",
            WebPage::Roadmap => "https://whimsical.com/sniffnet-roadmap-Damodrdfx22V9jGnpHSCGo",
            WebPage::Issues => "https://github.com/GyulyVGC/sniffnet/issues",
            WebPage::IssueLanguages => "https://github.com/GyulyVGC/sniffnet/issues/60",
            WebPage::Wiki => "https://github.com/GyulyVGC/sniffnet/wiki",
            WebPage::MyGitHub => "https://github.com/GyulyVGC",
        }
    }
}
