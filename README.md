# http_client_demo
不同 Rust http 客户端的使用和测试
- hyper
- reqwest: 在 hyper 之上
- ylong: OH 下的 http 客户端

Other:
- curl-rust
- actix-web-client(awc)：在 hyper 之上
- isahc：基于 libcurl
- surf：基于 libcurl
- ureq：只支持同步、只使用 safe rust

两个不同的 mock 工具：
- https://lib.rs/crates/httpc-test：模拟HTTP客户端来进行测试，建立在 reqwest 上
- https://docs.rs/httptest/latest/httptest：模拟服务器，建立在 hyper 上

Todo: 
- [ ] add benchmark framework
- [ ] 功能支持的比较
