## To reproduce the bug

Make a `GET` request on the `lock` endpoint.
This will lock the `wait` endpoint, it won't answer until someone hit the `unlock` endpoint.

```rust
#[get("/wait")]
async fn wait() -> impl Responder {
    println!("Wait called");
    while LOCKED.load(Ordering::Relaxed) {
        println!("Waiting");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    println!("Waiting was a success");
    HttpResponse::Ok().body("Hello world!")
}
```

Wait until your client disconnect because of a timeout, or ctrl+c your client. 

On the terminal running actix-web you should see that the while loop is still running. **It shouldn't**.

If you call the `unlock` endpoint you'll see that the loop and we get an answer.
