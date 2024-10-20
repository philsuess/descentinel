use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub content: Vec<u8>,
}

#[derive(Error, Debug)]
pub enum IpcError {
    #[error("ipc error: {0}")]
    ConnectionError(#[from] lapin::Error),
    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub async fn create_connection(ampq_url: &str) -> Result<Arc<Connection>, IpcError> {
    let connection = Connection::connect(&ampq_url, ConnectionProperties::default()).await?;
    Ok(Arc::new(connection))
}

async fn create_channel(connection: Arc<Connection>) -> Result<Channel, IpcError> {
    let channel = connection.create_channel().await?;
    Ok(channel)
}

pub async fn declare_queue(connection: Arc<Connection>, queue_name: &str) -> Result<(), IpcError> {
    let channel = create_channel(connection).await?;
    channel
        .queue_declare(
            &queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    Ok(())
}

pub async fn send_message(
    connection: Arc<Connection>,
    queue_name: &str,
    message: &Message,
) -> Result<(), IpcError> {
    let channel = create_channel(connection.clone()).await?;
    let serialized_msg = serde_json::to_vec(&message)?;

    channel
        .basic_publish(
            "",
            queue_name,
            BasicPublishOptions::default(),
            &serialized_msg,
            BasicProperties::default(),
        )
        .await?
        .await?; // Await message confirmation
                 //info!("Published message to '{}': {:?}", queue_name, message);
    Ok(())
}

pub async fn consume_messages<F, M>(
    connection: Arc<Connection>,
    queue_name: &str,
    process_message: F,
) -> Result<(), IpcError>
where
    F: Fn(M) -> Result<(), IpcError> + Send + Sync + 'static,
    M: DeserializeOwned + Send + 'static,
{
    let channel = create_channel(connection.clone()).await?;
    let mut consumer = channel
        .basic_consume(
            &queue_name,
            &format!("{queue_name}_consumer"),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery_result) = consumer.next().await {
        if let Ok(delivery) = delivery_result {
            let (data, ack) = (
                delivery.data.clone(),
                delivery.ack(BasicAckOptions::default()),
            );

            // Process message
            if let Ok(message) = serde_json::from_slice(&data) {
                process_message(message)?;
            }

            // Acknowledge message
            ack.await.unwrap();
        }
    }
    Ok(())
}

pub async fn process_message_pipeline<F>(
    connection: Arc<Connection>,
    from_queue_name: &str,
    to_queue_name: &str,
    filter_message: F,
) -> Result<(), IpcError>
where
    F: Fn(&Message) -> Option<Message> + Send + Sync + 'static,
{
    let to_queue_name_copy = to_queue_name.to_string();
    consume_messages(
        connection.clone(),
        from_queue_name,
        move |message: Message| {
            if let Some(downstream_message) = filter_message(&message) {
                tokio::spawn({
                    let connection_clone = connection.clone();
                    let downstream_message_clone = downstream_message.clone();
                    let to_queue_name_copy_clone = to_queue_name_copy.clone();
                    async move {
                        send_message(
                            connection_clone,
                            &to_queue_name_copy_clone,
                            &downstream_message_clone,
                        )
                        .await
                        .unwrap();
                    }
                });
            }

            Ok(())
        },
    )
    .await
}
