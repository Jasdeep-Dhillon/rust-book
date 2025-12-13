use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;
    // Default implementation
    fn default(&self) -> String {
        format!("Default implementation")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{} written by {} at {}",
            self.location, self.author, self.location
        )
    }

    fn default(&self) -> String {
        // Cannot call the pre-existing implementation
        format!("Custom implementation")
    }
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Local trait can be implemented on any type
// Imported trait cannot be implemented on imported types
impl Summary for String {
    fn summarize(&self) -> String {
        format!("Implemented summary trait for string type")
    }
}

// Any type that implements the summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
    println!("Item1 Summary: {}", item1.summarize());
    println!("Item2 Summary: {}", item2.summarize());
}

// Trait Bound Syntax
// Use to make sure both types are the same while implementing the trait
pub fn notify_trait_bound<T: Summary>(item1: &T, item2: &T) {
    println!(
        "Breaking news! {} & {}",
        item1.summarize(),
        item2.summarize()
    );
}

// Have multiple traits implemented
pub fn notify3(_item: &(impl Summary + Display)) {}
// Have trait bounds with multiple traits
pub fn notify4<T: Summary + Display>(_item: &T) {}

pub fn function1<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) {}
// Where clause for trait bounds
pub fn function2<T, U>(_t: &T, _u: &U)
// -> String return type here
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

// Returning any type that implements the specified trait
// Can only return one type but has to implement the trait beforehand
// Cannot return different types based on input
fn _returns_summarizable() -> impl Summary {
    NewsArticle {
        author: String::new(),
        content: String::new(),
        location: String::new(),
        headline: String::new(),
    }
}

fn _invalid_return(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            author: String::new(),
            content: String::new(),
            location: String::new(),
            headline: String::new(),
        }
    } else {
        // This works
        NewsArticle {
            author: String::from("something"),
            content: String::new(),
            location: String::new(),
            headline: String::new(),
        }
        // This doesn't work
        // Since the return type is different even though the trait is implemented
        // SocialPost {
        //     username: String::from("username"),
        //     content: String::from("Some content for the post"),
        //     reply: false,
        //     repost: false,
        // }
    }
}

struct _Pair<T> {
    x: T,
    y: T,
}

impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display> Display for _Pair<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{} y:{}", self.x, self.y)
    }
}

impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
