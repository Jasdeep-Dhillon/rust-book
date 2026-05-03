use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let text = trpl::get(url).await.text().await;

    let parsed = Html::parse(&text);
    // parsed.select_first("section").map(|section| println!("{:?}", section.inner_html()));
    parsed.select_first("title").map(|title| title.inner_html())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // trpl::run wrapper of block_on function from tokio
    // It creates a runtime and waits for the futures to finish executing
    trpl::Runtime::new().unwrap().block_on(async {
        let Some(url1) = args.get(1) else {
            println!("Please provide 2 urls as arguments");
            return;
        };
        let Some(url2) = args.get(2) else {
            println!("Please provide 2 urls as arguments");
            return;
        };
        
        // Only gets computed when .await is called
        let title1 = page_title(url1);
        let title2 = page_title(url2);
        
        // Both futures get computed here
        // Select function throws the slower result
        let (url, title) = match trpl::select(title1, title2).await {
            trpl::Either::Left(left) => (url1, left),
            trpl::Either::Right(right) => (url2, right),
        };
        println!("{url} returned first");
        match title {
            Some(title) => println!("With title: {title}"),
            None => println!("Page had no title"),
        }
    });
}
