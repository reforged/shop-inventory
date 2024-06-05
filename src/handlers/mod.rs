use std::sync::Arc;

use lapin::{options::QueueDeclareOptions, types::{AMQPValue, FieldTable}, Channel};
use futures_lite::stream::StreamExt;
pub mod order;


pub async fn listen_queue(queue_name: &str, channel: Arc<Channel>) {
    // écoute de la queue
    // declare queue type quorum
    //let queue = channel.queue_declare(queue_name, Default::default(), Default::default()).await.unwrap();
    let mut arguments = FieldTable::default();

    arguments.insert("x-queue-type".into(), AMQPValue::LongString("quorum".into()));
    let queue = channel
    .queue_declare(
        &queue_name, 
        QueueDeclareOptions {
            durable: true,
            ..Default::default()
        }, 
        arguments
    ).await;

    // if queue error go break
    if queue.is_err() {
        println!("Error declaring queue: {:?}", queue.err());
        return;
    }
    
    let mut consumer = channel.basic_consume(&queue_name, "my_consumer", Default::default(), Default::default()).await.unwrap();

    println!("Listening to queue {}", queue_name);

    // boucle d'écoute
    // loop {
    //     let delivery = consumer.recv().await.unwrap();
    //     let message = delivery.data;
    //     let message = std::str::from_utf8(&message).unwrap();
    //     println!("Received message: {:?}", message);
    //     match queue_name {
    //         "order" => order::process_message_order(channel.clone()).await,
    //         _ => (),
    //     }
    //     //process_message(queue_name, channel.clone()).await;
    // }

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.unwrap();
        let message = delivery.data;
        let message = std::str::from_utf8(&message).unwrap();
        println!("Received message: {:?}", message);
        match queue_name {
            "order" => {
               // parse content message to MEssageORder struct
               let message = serde_json::from_str(message);

               // match if err , ack message and print error
                match message {
                     Ok(message) => {
                          order::process_message_order(message).await;
                     },
                     Err(e) => {
                          println!("Error parsing message: {:?}", e);
                          channel.basic_ack(delivery.delivery_tag, Default::default()).await.unwrap();
                     }
                }

            },
            _ => (),
        }
    }
    // match queue_name {
    //     "order" => order::process_message_order(channel).await,
    //     _ => (),
    // }
}