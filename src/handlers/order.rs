use crate::models::MessageOrder;

pub async fn process_message_order(message: MessageOrder) {
    println!("Message: {:?}", message);
    println!("Processing message from order queue");
}