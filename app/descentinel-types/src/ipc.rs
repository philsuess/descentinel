//use crate::message::Message;
//use anyhow::Result;
//use futures_util::stream::StreamExt;
use lapin::{Connection, ConnectionProperties};
//use serde::{de::DeserializeOwned, Serialize};
//use std::sync::Arc;
//use tokio::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IpcError {
    #[error("ipc error: {0}")]
    ConnectionError(#[from] lapin::Error),
}

#[allow(async_fn_in_trait)]
pub trait Ipc {
    async fn establish_connection(ampq_url: &str) -> Result<Connection, IpcError>;
    /*    async fn init(queue_name: &str) -> Result<Arc<Mutex<Channel>>>;
    async fn send_message<T: Serialize + Send + Sync>(
        channel: Arc<Mutex<Channel>>,
        queue_name: &str,
        message: T,
    ) -> Result<()>;

    async fn receive_message<T: DeserializeOwned + Send + Sync + 'static>(
        channel: Arc<Mutex<Channel>>,
        queue_name: &str,
        callback: impl Fn(T) + Send + Sync + 'static,
    ) -> Result<()>;*/
}

pub struct RabbitMqiIpc;

impl Ipc for RabbitMqiIpc {
    async fn establish_connection(ampq_url: &str) -> Result<Connection, IpcError> {
        Ok(Connection::connect(&ampq_url, ConnectionProperties::default()).await?)
    }

    /*async fn init(queue_name: &str) -> Result<Arc<Mutex<Channel>>> {
        let addr =
            std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
        let connection =
            Connection::connect(&addr, ConnectionProperties::default().with_tokio()).await?;
        let channel = connection.create_channel().await?;

        channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(Arc::new(Mutex::new(channel)))
    }

    async fn send_message<T: Serialize + Send + Sync>(
        channel: Arc<Mutex<Channel>>,
        queue_name: &str,
        message: T,
    ) -> Result<()> {
        let serialized_msg = serde_json::to_vec(&message)?;
        let channel = channel.lock().await;

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

        Ok(())
    }

    async fn receive_message<T: DeserializeOwned + Send + Sync + 'static>(
        channel: Arc<Mutex<Channel>>,
        queue_name: &str,
        callback: impl Fn(T) + Send + Sync + 'static,
    ) -> Result<()> {
        let channel = channel.lock().await;

        let mut consumer = channel
            .basic_consume(
                queue_name,
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        tokio::spawn(async move {
            while let Some(delivery) = consumer.next().await {
                if let Ok(delivery) = delivery {
                    let data = delivery.data;
                    if let Ok(message) = serde_json::from_slice::<T>(&data) {
                        callback(message);
                    }
                    delivery.ack(BasicAckOptions::default()).await.unwrap();
                }
            }
        });

        Ok(())
    }*/
}
