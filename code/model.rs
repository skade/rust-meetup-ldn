struct Post {
    id: u64,
    content: String
}

struct Comment {
    id: u64,
    post_id: u64,
    content: String
}
