use stale_read_expert::{SessionContext, RequestType, router::handle_request};
use futures::future::join_all;

fn setup() {
    stale_read_expert::init_cluster();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_basic_read_after_write() {
    setup();
    let ctx = SessionContext::new();
    let key = format!("t1:{}", std::time::SystemTime::now().elapsed().unwrap().as_nanos());
    let write_req = RequestType::Write { key: key.clone(), value: "success".to_string() };
    handle_request(write_req, &ctx).await;
    let read_req = RequestType::Read { key };
    let read_resp = handle_request(read_req, &ctx).await;
    assert_eq!(read_resp.value, Some("success".to_string()));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_read_after_write_with_yield() {
    setup();
    let ctx = SessionContext::new();
    let key = format!("t2:{}", std::time::SystemTime::now().elapsed().unwrap().as_nanos());
    let write_req = RequestType::Write { key: key.clone(), value: "yield_value".to_string() };
    handle_request(write_req, &ctx).await;
    tokio::task::yield_now().await;
    tokio::task::yield_now().await;
    let read_req = RequestType::Read { key };
    let read_resp = handle_request(read_req, &ctx).await;
    assert_eq!(read_resp.value, Some("yield_value".to_string()));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_read_after_write_spawn() {
    setup();
    let ctx = SessionContext::new();
    let key = format!("t3:{}", std::time::SystemTime::now().elapsed().unwrap().as_nanos());
    let write_req = RequestType::Write { key: key.clone(), value: "spawned".to_string() };
    handle_request(write_req, &ctx).await;
    let ctx_clone = ctx.clone();
    let key_clone = key.clone();
    let handle = tokio::spawn(async move {
        let read_req = RequestType::Read { key: key_clone };
        handle_request(read_req, &ctx_clone).await
    });
    let read_resp = handle.await.unwrap();
    assert_eq!(read_resp.value, Some("spawned".to_string()));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_read_after_write_spawn_blocking() {
    setup();
    let ctx = SessionContext::new();
    let key = format!("t4:{}", std::time::SystemTime::now().elapsed().unwrap().as_nanos());
    let write_req = RequestType::Write { key: key.clone(), value: "blocked".to_string() };
    handle_request(write_req, &ctx).await;
    let ctx_clone = ctx.clone();
    let key_clone = key.clone();
    let read_resp = tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            let read_req = RequestType::Read { key: key_clone };
            handle_request(read_req, &ctx_clone).await
        })
    }).await.unwrap();
    assert_eq!(read_resp.value, Some("blocked".to_string()));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_writes_and_reads() {
    setup();
    let ctx = SessionContext::new();
    let base = std::time::SystemTime::now().elapsed().unwrap().as_nanos();
    let write_tasks: Vec<_> = (0..5u64).map(|i| {
        let ctx = ctx.clone();
        let key = format!("t5:{}:{}", base, i);
        tokio::spawn(async move {
            let req = RequestType::Write { key: key.clone(), value: format!("v{}", i) };
            handle_request(req, &ctx).await
        })
    }).collect();
    join_all(write_tasks).await;
    tokio::time::sleep(tokio::time::Duration::from_millis(60)).await;
    for i in 0..5 {
        let key = format!("t5:{}:{}", base, i);
        let read_req = RequestType::Read { key };
        let read_resp = handle_request(read_req, &ctx).await;
        assert_eq!(read_resp.value, Some(format!("v{}", i)));
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_session_isolation() {
    setup();
    let ctx1 = SessionContext::new();
    let ctx2 = SessionContext::new();
    let key = format!("t6:{}", ctx1.id());
    let req1 = RequestType::Write { key, value: "from1".to_string() };
    handle_request(req1, &ctx1).await;
    assert!(!ctx2.requires_primary());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_heavy_concurrency() {
    setup();
    let ctx = SessionContext::new();
    ctx.record_write();
    let ctx_clone = ctx.clone();
    let handle1 = tokio::spawn(async move {
        tokio::task::yield_now().await;
        ctx_clone.requires_primary()
    });
    let ctx_clone2 = ctx.clone();
    let handle2 = tokio::spawn(async move {
        tokio::task::yield_now().await;
        ctx_clone2.has_writes()
    });
    assert!(handle1.await.unwrap());
    assert!(handle2.await.unwrap());
}
