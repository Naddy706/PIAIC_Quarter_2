pub trait Summary {
fn summarize(&self) -> String;
}


#[derive(Debug)]
struct NewsArticle {
    author: String,
    content: String,
}


impl Summary for NewsArticle {
fn summarize(&self) -> String {
format!("{} , {}", self.author, self.content)
}
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
}


impl Summary for Tweet {
fn summarize(&self) -> String {
format!("{} , {}", self.username, self.content)
}
}

fn main() {




let article = NewsArticle {
author: String::from("Nadir Hussain Article"),
content: String::from("This content is based on my Assignment"),

};
println!(" News Article: {} ", article.summarize());

let tweet = Tweet {
username: String::from("Nadir Hussain Tweet"),
content: String::from("This content is based on my Assignment"),

};
println!(" tweet: {}", tweet.summarize());

}



