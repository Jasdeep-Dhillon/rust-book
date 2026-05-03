use traits::{NewsArticle, SocialPost, Summary, notify, notify_trait_bound, notify2};

// Cannot compile because Summary is imported and not local to this crate
// impl Summary for String {
//     fn summarize(&self) -> String {
//         format!("String implementation")
//     }
// }

fn main() {
    let post = SocialPost {
        username: String::from("username"),
        content: String::from("Some content for the post"),
        reply: false,
        repost: false,
    };
    println!("1 new post -> {} \n{}", post.summarize(), post.default());

    let article = NewsArticle {
        author: String::from("article_author"),
        location: String::from("author_location"),
        headline: String::from("headline"),
        content: String::from("Placeholder content"),
    };
    println!("{}", article.default());
    println!("{}", String::new().summarize());

    notify(&article);
    // Can have different types as long as the trait is implemented
    notify2(&post, &article);
    // Wont compile, must have same types while implementing the trait
    // notify_trait_bound(&post, &article);
    notify_trait_bound(&post, &post);
}
