# HttpToolKit Memory Issue.

Install Rust & Cargo via Rustup

Use 8000 as port in HttpToolKit

Issue the following command:

```
$ cargo run --release
```


It sends too many request to example.com, and we can see the memory issue in httptoolkit that never gets recovered. There can be various nuances what memory means like committed memory, cached memory etc. so we need to be careful here.
