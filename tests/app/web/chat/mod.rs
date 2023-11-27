use super::data_test_id;

mod message;
mod messages;
mod input;

fn get_message_content(test_id: String) -> String {
    data_test_id(&test_id)
        .child_nodes()
        .get(1)
        .unwrap()
        .text_content()
        .unwrap()
}
