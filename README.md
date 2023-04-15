# WebSocket-Server-with-Background-Job


This project demonstrates a WebSocket server using Actix Web, Actix Web Actors, and Tokio. The server has an HTTP endpoint that returns a "Hello, world!" JSON message and a WebSocket endpoint that allows clients to connect and send messages. The server will respond to each incoming message with a "Message received" text. Additionally, there is a background job that runs in parallel, printing a message to the console every 10 seconds.

