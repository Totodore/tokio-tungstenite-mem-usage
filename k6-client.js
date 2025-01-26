import ws from "k6/ws";
import { check } from "k6";

export let options = {
  stages: [
    { duration: "30s", target: 5000 },
    { duration: "30s", target: 10000 },
  ],
};

export default function () {
  const url = "ws://127.0.0.1:3000/";

  let receivedEcho = false;
  const res = ws.connect(url, {}, function (socket) {
    socket.on("open", () => socket.send("Hello"));
    socket.on("message", (data) => {
      if (data === "Hello") {
        receivedEcho = true;
      }
    });
    socket.on("close", () => console.log("disconnected"));
  });

  check(receivedEcho, { "received echo": (r) => !!r });
  check(res, { "status is 101": (r) => r && r.status === 101 });
}
