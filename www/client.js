const ws = new WebSocket('ws://localhost:8181');
console.log("opening websocket...");
ws.onopen = function (event) {
    console.log("Success!");
  ws.send('something');
}
