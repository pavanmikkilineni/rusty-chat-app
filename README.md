# rusty-chat-app

This is a simple TCP chat application implemented in Rust, consisting of a server and a client. The server accepts client connections and relays messages between multiple clients using a broadcast channel. Each client connection is handled concurrently in its own task using the `tokio::spawn` function.

## Prerequisites

Before running the application, make sure you have Rust and Cargo installed on your system.

## Running the Server

To start the server, execute the following command in your terminal:

```shell
cargo run --bin server
```

The server will bind to `127.0.0.1:8000` and start listening for client connections.

## Running the Client

To connect to the server as a client, execute the following command in another terminal:

```shell
cargo run --bin client
```

The client will establish a TCP connection to `127.0.0.1:8000` and allow you to send and receive messages.

## How It Works

### Server

The server code creates a TCP listener that binds to `127.0.0.1:8000` and waits for client connections. It also sets up a broadcast channel to relay messages between clients. Each client connection is handled concurrently in its own task using `tokio::spawn`. When a client sends a message, it is broadcasted to all other connected clients except the sender.

### Client

The client code establishes a TCP connection to `127.0.0.1:8000` and sets up separate tasks for reading and writing messages. The reading task continuously reads messages from the server and displays them on the console. The writing task allows the user to input messages from the console and send them to the server.

## Usage

1. Start the server by running `cargo run --bin server` in one terminal.
2. Connect to the server as a client by running `cargo run --bin client` in another terminal.
3. Start sending and receiving messages between the connected clients.

Note: You can run multiple client instances to simulate a multi-client chat environment.

That's it! You now have a basic TCP chat application implemented in Rust.

Feel free to modify and enhance the code to suit your needs. Happy coding!
