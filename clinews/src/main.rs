
fn get_articles(url: &str){
    
    let responce = ureq::get(url).call()?.into_string()?;
    
}


fn main() {

    let url: &str = "https://newsapi.org/v2/top-headlines?country=in&apiKey=API_KEY";
    let articles = get_articles(url);
    
}
