# Lossy Link
Simulate a simple lossy UDP link. The uplink is can also have out of order packets.

## Usage

The simple usage is like this:

```bash
./lossy_link <BIND_ADDRESS> <FORWARD_ADDRESS> [LOSS_RATE] [OUT_OF_ORDER_RATE]
```

For instance, `./lossy_link 127.0.0.1:12345 127.0.0.1:54321 0.5` will start the link simulator, listening on `127.0.0.1:12345` and forwarding whatever packet it receives to `127.0.0.1:54321`. Each packet (either coming from client or server) has a 1 in 2 chance to be dropped. The `OUT_OF_ORDER_RATE` gets its default value of `0.1`. This means that there is a 10% chance that each pair of upstream datagrams gets swapped while being forwarded.

The `LOSS_RATE` can be omitted. Its default value is `0.1`.

The `OUT_OF_ORDER_RATE` can be omitted. Its default value is `0.1`.

## How It Works

It simply proxies the packets between two peers. However, for each packet it draws a random number in `(0, 1]` and if it's less than a probability value it will simply ignore the packet.

For uplink out of order, a channel is created between the main thread and a daemon thread. Each packet received from the client is send in to the channel. In the daemon thread, based on another probability, the thread might wait for another packet and then swaps their order.

It does not support multiplexing. So multiple clients cannot connect to it.