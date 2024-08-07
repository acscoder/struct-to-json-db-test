struct_to_json_db::auto_json_db_config!("./db/");

#[auto_json_db]
struct Post {
    title: String,
    description: String,
    categories: Vec<String>
}
 
fn main() {
    let all_posts = Post::get_all(); 
    println!("{:?}", all_posts);
}
fn add_posts(title: String, description: String, categories: Vec<String>) {
    let p1 = Post::new( title,description, categories);
    p1.save();
}
