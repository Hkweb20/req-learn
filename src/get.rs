
use reqwest::{Error, Response};
use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize,Deserialize)]
struct Body {
    body : String,
    title: String
}
async fn get_api()-> Result<(),Error>{
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts").await?;

     match response.status().as_u16() {
         200..= 299 => {
            let body = response.text().await?;
            let post: Vec<Body> = serde_json::from_str(&body).expect("error");
         for posts in post{
            println!("the body is {} \n the title is {}",posts.body,posts.title);
         } 
        }
         400..= 599 => {
            let status = response.status();
            let error_message  = response.text().await?;
            println!("the error {} \n the status {}", error_message,status);

         }
        _ => println!("unexpected error "),
     };


    Ok(())
}
#[tokio::main]
async fn main()-> Result<(),Error>{
    get_api().await?;
    Ok(())

}