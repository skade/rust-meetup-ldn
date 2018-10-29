pub struct Posts;

impl Relation for Posts {
    type PrimaryKey = i64;
    type Model = Post;

    // ...

    fn name() -> &'static str {
        "posts"
    }
}

pub struct Comments;
// ..