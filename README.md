# rico.rs
A rico.com.vc http client.
# rico.rs
A [rico.com.vc](https://www.rico.com.vc/) http client written in rust.

It's pretty straight forward to use. Just create a client by calling the login function.

```rust
use rico_rs::RicoClient;

#[tokio::main]
async fn main() {
    let c = RicoClient::login("<username>".into(), "password".into())
        .await
        .unwrap();
   
    let c_info = c.customer_info().await.unwrap();
    println!("{:?}", c_info);
    
    let summary = c.summary().await.unwrap();
    println!("{:?}", summary);
}
```

Be aware that for now only the customer_info and the summary API's are implemented.

For info on what data this API's fetch, please see the files:
* [customer_info.rs](src/customer_info.rs)
* [summary.rs](src/summary.rs)