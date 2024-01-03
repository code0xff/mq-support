import zmq from "zeromq";

async function run() {
  const sock = new zmq.Subscriber;

  sock.connect("tcp://127.0.0.1:13080");
  sock.subscribe("test-topic");
  console.log("Subscriber connected to port 13080");

  for await (const [topic, msg] of sock) {
    console.log("received a message related to:", topic.toString(), "containing message:", msg);
  }
}

run();