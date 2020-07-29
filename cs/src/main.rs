// Declare modules
// mod miscellaneous;
// mod sorting;

pub trait Summary {
  fn summarize_author(&self) -> &String;

  fn summarize(&self) -> String {
    format!("Read more from {}", self.summarize_author())
  }
}

pub struct NewsArticle {
  author: String
}

impl Summary for NewsArticle {
  fn summarize_author(&self) -> &String {
    &self.author
  }
}

pub struct Sender {}

impl Sender {
  pub fn message(&self, article: &impl Summary) {
    println!("{}", article.summarize_author());
  }
}

fn main() {
  let article = NewsArticle {
    author: String::from("Arthur")
  };

  let sender = Sender {};

  sender.message(&article);
}
