use sc_mq::MessageQueueWorker;

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let worker = MessageQueueWorker::new("tcp://127.0.0.1:13080", 32);
	let queue = worker.queue();

	tokio::spawn(async move {
		loop {
			let now = chrono::Local::now().to_string();
			if let Err(e) = queue.clone().try_send("test-topic", now.as_bytes().to_vec()) {
				eprintln!("{:?}", e);
			}
			tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
		}
	});

	worker.run().await;

	Ok(())
}
