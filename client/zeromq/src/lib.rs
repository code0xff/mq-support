use futures::{SinkExt, StreamExt};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Message {
    topic: String,
    message: serde_json::Value,
}

#[derive(Clone)]
pub struct MessageQueue {
    pub sender: futures::channel::mpsc::Sender<Message>,
}

impl MessageQueue {
    pub fn new(sender: futures::channel::mpsc::Sender<Message>) -> Self {
        Self { sender }
    }

    pub async fn send(
        mut self,
        topic: &str,
        message: &serde_json::Value,
    ) -> Result<(), futures::channel::mpsc::SendError> {
        let message = Message {
            topic: topic.to_string(),
            message: message.clone(),
        };
        self.sender.send(message).await
    }
}

pub struct MessageQueueWorker {
    sender: futures::channel::mpsc::Sender<Message>,
    receiver: futures::channel::mpsc::Receiver<Message>,
    socket: zmq::Socket,
}

impl MessageQueueWorker {
    pub fn new(endpoint: &str, buffer: usize) -> Self {
        let ctx = zmq::Context::new();
        let socket = ctx.socket(zmq::PUB).unwrap();
        socket.bind(endpoint).expect("socket failed to bind");
        let (tx, rx) = futures::channel::mpsc::channel(buffer);
        Self {
            sender: tx,
            receiver: rx,
            socket,
        }
    }
}

impl MessageQueueWorker {
    pub fn publish(&self, topic: &str, message: &str) -> Result<(), zmq::Error> {
        self.socket.send(topic, zmq::SNDMORE)?;
        self.socket.send(message, 0)?;
        Ok(())
    }

    pub fn queue(&self) -> MessageQueue {
        MessageQueue::new(self.sender.clone())
    }

    pub async fn run(mut self) {
        while let Some(message) = self.receiver.next().await {
            let Message { topic, message } = message;
            let _ = self.publish(topic.as_str(), message.to_string().as_str());
        }
    }
}
