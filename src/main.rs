use struct_to_json_db::*;
auto_json_db_config!("./db/");

#[auto_json_db]
struct Post {
    title: String,
    description: String
}
#[auto_json_db]
struct Category {
    name: String
}
struct_db_relation!(Post, Category);

fn main() {
    let all_posts = Post::get_all(); 
    println!("{:?}", all_posts);

    let name_val = "test".to_string();
    let p1 = db_relation_get!(Post=1, Category);
}
fn add_posts(title: String, description: String) {
    let p1 = Post::new( title,description);
    p1.save();
}
