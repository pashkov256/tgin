## Load testing (Webhook only)
To validate performance, I developed a specialized benchmark tool in Rust. It sends fake Webhook requests to the bot and measures the full Round-Trip Time (RTT) to receive a response.
To achieve this, the bot is configured to send its replies (sendMessage) not to the real Telegram API, but back to the benchmark server.

***Note: Currently supports Webhook updates. Long Polling support is coming soon.***

The benchmark sends a fixed RPS (Requests Per Second) for a specified duration, then waits 5 seconds to collect trailing responses before calculating statistics.


### How to prove
```
make bench-direct
```
```
make bench-tgin
```

### Configuration
I ran two comparison tests:

- Direct: All requests go to a single Python "Echo Bot".

- Tgin: Requests go to the tgin load balancer, which distributes traffic between two replicas Python "Echo Bot" using the Round-Robin strategy.

## Results

### Direct
```
🚀 Starting Benchmark
🎯 Target: http://10.5.0.11:8080/webhook
⚡ RPS: 300
⏱ Duration: 10s
📡 Mock Server Port: 8090
🏁 Sending finished. Waiting 5s for trailing responses...

==========================================
📊 BENCHMARK RESULTS
==========================================
Requests Sent:     3001
Responses Recv:    726
Errors:            277
Loss Rate:         75.81%
------------------------------------------
LATENCY (Round-Trip Time):
  Min:    1.24 ms
  Mean:   2407.23 ms
  p50:    1435.65 ms
  p95:    5971.97 ms
  p99:    11911.17 ms
  Max:    11952.13 ms
==========================================
```


### Tgin
```
🚀 Starting Benchmark
🎯 Target: http://10.5.0.2:3000/webhook
⚡ RPS: 300
⏱ Duration: 10s
📡 Mock Server Port: 8090
🏁 Sending finished. Waiting 5s for trailing responses...

==========================================
📊 BENCHMARK RESULTS
==========================================
Requests Sent:     3001
Responses Recv:    3001
Errors:            0
Loss Rate:         0.00%
------------------------------------------
LATENCY (Round-Trip Time):
  Min:    1.08 ms
  Mean:   14.80 ms
  p50:    7.66 ms
  p95:    46.66 ms
  p99:    68.61 ms
  Max:    170.50 ms
==========================================
```

### What we can see
The benchmark results clearly demonstrate the bottleneck of a monolithic architecture and the efficiency of horizontal scaling with tgin:

1. Reliability (0% vs 277 Errors):
    In the Direct scenario, the single Python bot failed to handle the TCP connection influx, resulting in 277 connection errors and a 75.8% loss rate. 
    With tgin, the error rate dropped to 0. The Rust-based architecture successfully accepted all incoming connections, buffering them efficiently before distribution.

2. Throughput (4x Increase):

    Direct: The single bot effectively processed only ~72 RPS (726 responses / 10s).

    Tgin: The cluster processed the full 300 RPS load without dropping a single packet.
    By adding just one extra worker node managed by tgin, we achieved 100% throughput reliability.

3. Latency (160x Improvement):
    The most dramatic difference is in response time.

    Direct: Average latency was 2.4 seconds, with spikes up to 12 seconds due to queue blocking.

    Tgin: Average latency dropped to 14.8 ms.
    Tgin effectively acted as a shock absorber, distributing the load so that neither of the two worker bots was overwhelmed.