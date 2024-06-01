
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

fn main() {

    let tweet = Tweet {
        username: "@johndoe".to_owned(),
        content: "Hello world".to_owned(),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        author: "John Doe".to_owned(),
        headline: "The sky if falling".to_owned(),
        content: "The sky is not actually falling".to_owned()
    };
    println!("Tweet summary {}", tweet.summarize());
    println!("Article summary {}", article.summarize());
}

