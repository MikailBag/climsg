
/// Visitor which prints all messages as one-line JSON
pub struct JsonVisitor {
    _priv: (),
}

impl JsonVisitor {
    pub fn new() -> JsonVisitor {
        JsonVisitor { _priv: () }
    }
}

impl crate::Visitor for JsonVisitor {
    fn visit_message(&self, message: crate::Message<'_>) {
        let message =
            serde_json::to_string(&message.structured).expect("message can not be serialized");
        assert!(!message.contains('\n'));
        println!("{}", message);
    }
}