use struct_to_json_db::*;
use serde::{Deserialize, Serialize};

use dotenv::dotenv; 

#[auto_json_db(singleton,encript="APP_SECRET_KEY")]
struct SiteConfig {
    url: String,
    limit: i32
}
impl SiteConfig {
    fn default()->Self{
        Self::new("".to_owned(),0) 
    }
}

#[auto_json_db(bigsize,unique="title")]
struct Post {
    title: String,
    description: String,
    categories: Vec<u64>
}
#[auto_json_db(unique="name")]
struct Category {
    name: String
}
json_db_relation!(Post=categories, Category);
 
fn main() {
    dotenv().ok();
    set_struct_json_path("./db/");

    let mut cf = SiteConfig::load();
     
    println!("{:?}",  cf);
    
    let mut all_posts = Post::get_all(); 
    println!("{:?}", all_posts.len());
    
    let c1 = Category::new("cat_1".to_owned());
    let c2 =  Category::new("cat_2".to_owned());
    c1.save();
    c2.save();
   
    let c = add_posts("post_2".to_owned(), "desc_1".to_owned(), &vec![c1,c2]);
    println!("{:?}", c);
    
}
fn add_posts(title: String, description: String,cates:&Vec<Category>)->Option<u64> {
    let mut p1 = Post::new( title,description,vec![]);
    if cates.len() > 0{
        p1.set_categories(cates);
    }
    p1.save()
}