use futures::{SinkExt, StreamExt};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Message {
	topic: String,
	data: Vec<u8>,
}

#[derive(Clone)]
pub struct MessageQueue {
	pub sender: futures::channel::mpsc::Sender<Message>,
}

impl MessageQueue {
	pub fn new(sender: futures::channel::mpsc::Sender<Message>) -> Self {
		Self { sender }
	}

	pub fn try_send(
		mut self,
		topic: &str,
		data: Vec<u8>,
	) -> Result<(), futures::channel::mpsc::TrySendError<Message>> {
		let message = Message { topic: topic.to_string(), data };
		self.sender.try_send(message)
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
		Self { sender: tx, receiver: rx, socket }
	}
}

impl MessageQueueWorker {
	pub fn publish(&self, topic: &str, data: &Vec<u8>) -> Result<(), zmq::Error> {
		self.socket.send(topic, zmq::SNDMORE)?;
		self.socket.send(data, 0)?;
		Ok(())
	}

	pub fn queue(&self) -> MessageQueue {
		MessageQueue::new(self.sender.clone())
	}

	pub async fn run(mut self) {
		while let Some(message) = self.receiver.next().await {
			let Message { topic, data } = message;
			let _ = self.publish(topic.as_str(), &data);
		}
	}
}
