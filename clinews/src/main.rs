use std::error::Error;

struct Articles{
   
   articles: Vec<Article>  
}


struct Article{
    
}


fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>>{
    
    let response = ureq::get(url).call()?.into_string()?;
    
    dbg!(response);
    
    todo!()
    
}


fn main() {

    // contains top headlines end point as a string
    let url: &str = "https://newsapi.org/v2/top-headlines?country=in&apiKey=81ad1c2efc7049d8882727ee26749dc1";
    
    let articles: Result<Articles, Box<dyn Error>> = get_articles(url);
    
}
