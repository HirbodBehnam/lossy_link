# Lossy Link
Simulate a simple lossy UDP link.

## How It Works

It simply proxies the packets between two peers. However, for each packet it draws a random number in `(0, 1]` and if it's less than a probability value it will simply ignore the packet.

It does not support multiplexing. So multiple clients cannot connect to it.

## Usage

The simple usage is like this:

```bash
./lossy_link <BIND_ADDRESS> <FORWARD_ADDRESS> [LOSS_RATE]
```

For instance, `./lossy_link 127.0.0.1:12345 127.0.0.1:54321 0.5` will start the link simulator, listening on `127.0.0.1:12345` and forwarding whatever packet it receives to `127.0.0.1:54321`. Each packet (either coming from client or server) has a 1 in 2 chance to be dropped.

The `LOSS_RATE` can be omitted. Its default value is `0.1`.