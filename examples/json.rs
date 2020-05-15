use climsg::show;

fn main() {
    let msg = serde_json::json!({
        "hello": "world",
        "count": 42,
        "is_simple": true,
        "bugs": null,
    });
    show!(
        &climsg::visitors::JsonVisitor::new(),
        msg,
        "you will not see it",
    );
}
