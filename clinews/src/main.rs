use std::error::Error;
use serde::Deserialize;
use colour::{dark_green, yellow}

#[derive(Deserialize, Debug)]


struct Articles{
   
   articles: Vec<Article>  
}

#[derive(Deserialize, Debug)]

struct Article{
    title: String,
    url: String
}


fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>>{
    
    let response = ureq::get(url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles);

}

fn render_articles(articles: &Articles){
    
    for i: &Articles in &articles.articles{
    
         dark_green();
         yellow();
    
    }

}


fn main() -> Result<(), Box<dyn Error> {

    // contains top headlines end point as a string
    let url: &str = "https://newsapi.org/v2/top-headlines?country=in&apiKey=81ad1c2efc7049d8882727ee26749dc1";
    let articles: Articles = get_articles(url)?;
    
    render_articles(&articles);
    
    Ok(())

}
