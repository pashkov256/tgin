use axum::{extract::State, routing::post, Json, Router};
use axum::body::Bytes; 
use dashmap::DashMap;
use hdrhistogram::Histogram;
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH}; 
use tokio::sync::Mutex;
use tokio::time::interval;
use uuid::Uuid;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "http://127.0.0.1:3000/webhook")]
    target: String,

    #[arg(short, long, default_value_t = 1000)]
    rps: u64,

    #[arg(short, long, default_value_t = 10)]
    duration: u64,

    #[arg(short, long, default_value_t = 8090)]
    port: u16,
}

struct BenchState {
    pending: DashMap<String, Instant>,
    
    histogram: Mutex<Histogram<u64>>,

    sent_count: AtomicUsize,
    received_count: AtomicUsize,
    errors_count: AtomicUsize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let state = Arc::new(BenchState {
        pending: DashMap::new(),
        histogram: Mutex::new(Histogram::<u64>::new(3).unwrap()), 
        sent_count: AtomicUsize::new(0),
        received_count: AtomicUsize::new(0),
        errors_count: AtomicUsize::new(0),
    });

    println!("🚀 Starting Benchmark");
    println!("🎯 Target: {}", args.target);
    println!("⚡ RPS: {}", args.rps);
    println!("⏱ Duration: {}s", args.duration);
    println!("📡 Mock Server Port: {}", args.port);

    let server_state = state.clone();
    let app = Router::new()
        .route("/bot:token/sendMessage", post(handle_send_message))
        .with_state(server_state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], args.port));
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    tokio::time::sleep(Duration::from_millis(500)).await;

    let client = Client::builder()
        .pool_max_idle_per_host(1000) 
        .build()
        .unwrap();

    let gen_state = state.clone();
    let target_url = args.target.clone();
    let rps = args.rps;
    let duration_sec = args.duration;

    let generator_handle = tokio::spawn(async move {
        let start_test = Instant::now();
        let interval_micros = 1_000_000 / rps; 
        let mut ticker = interval(Duration::from_micros(interval_micros));

        while start_test.elapsed().as_secs() < duration_sec {
            ticker.tick().await; 

            let uuid = Uuid::new_v4().to_string();
            let url = target_url.clone();
            let s = gen_state.clone();
            let c = client.clone();

            s.pending.insert(uuid.clone(), Instant::now());
            s.sent_count.fetch_add(1, Ordering::Relaxed);

            tokio::spawn(async move {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let payload = json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 123,
                        "date": timestamp, 
                        "chat": { "id": 1, "type": "private" },
                        "from": { "id": 1, "is_bot": false, "first_name": "Bench" },
                        "text": uuid 
                    }
                });

                if c.post(&url).json(&payload).send().await.is_err() {
                    s.errors_count.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });

    let stats_handle = tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
        }
    });

    generator_handle.await.unwrap();
    
    println!("🏁 Sending finished. Waiting 5s for trailing responses...");
    tokio::time::sleep(Duration::from_secs(5)).await;

    print_report(&state).await;
}

async fn handle_send_message(
    State(state): State<Arc<BenchState>>,
    body: Bytes, 
) -> Json<Value> {
    
    let payload: Value = if let Ok(json) = serde_json::from_slice(&body) {
        json
    } else if let Ok(form) = serde_urlencoded::from_bytes(&body) {
        form
    } else {
        json!({})
    };

    if let Some(text) = payload.get("text").and_then(|t| t.as_str()) {
        if let Some((_, start_time)) = state.pending.remove(text) {
            let elapsed = start_time.elapsed().as_micros() as u64;
            
            let mut hist = state.histogram.lock().await;
            let _ = hist.record(elapsed);
            
            state.received_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Json(json!({
        "ok": true,
        "result": {
            "message_id": 123,
            "date": timestamp,
            "chat": {
                "id": 1,
                "type": "private",
                "first_name": "BenchUser"
            },
            "from": {
                "id": 999,
                "is_bot": true,
                "first_name": "TginBenchBot"
            },
            "text": "Reply from bench" 
        }
    }))
}

async fn print_report(state: &BenchState) {
    let sent = state.sent_count.load(Ordering::Relaxed);
    let received = state.received_count.load(Ordering::Relaxed);
    let errors = state.errors_count.load(Ordering::Relaxed);
    let hist = state.histogram.lock().await;

    println!("\n==========================================");
    println!("📊 BENCHMARK RESULTS");
    println!("==========================================");
    println!("Requests Sent:     {}", sent);
    println!("Responses Recv:    {}", received);
    println!("Errors:            {}", errors);
    println!("Loss Rate:         {:.2}%", 100.0 * (sent - received) as f64 / sent as f64);
    println!("------------------------------------------");
    println!("LATENCY (Round-Trip Time):");
    println!("  Min:    {:.2} ms", hist.min() as f64 / 1000.0);
    println!("  Mean:   {:.2} ms", hist.mean() / 1000.0);
    println!("  p50:    {:.2} ms", hist.value_at_quantile(0.5) as f64 / 1000.0);
    println!("  p95:    {:.2} ms", hist.value_at_quantile(0.95) as f64 / 1000.0);
    println!("  p99:    {:.2} ms", hist.value_at_quantile(0.99) as f64 / 1000.0);
    println!("  Max:    {:.2} ms", hist.max() as f64 / 1000.0);
    println!("==========================================");
}