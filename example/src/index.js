import zmq from "zeromq";
import { WsProvider, ApiPromise } from "@polkadot/api";

async function run() {
  const sock = new zmq.Subscriber;

  sock.connect("tcp://127.0.0.1:13080");
  sock.subscribe("txpool_add");
  console.log("Subscriber connected to port 13080");

  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  for await (const [topic, data] of sock) {
    const hex = `0x${data.toString("hex")}`;
    const tx = api.tx(hex);
    console.log(tx.toHuman());
    
    const queryInfo = await api.rpc.payment.queryFeeDetails(hex);
    console.log(queryInfo.toJSON());
  }
}

run();