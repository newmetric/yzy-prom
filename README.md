
Following depedencies must be added to use the macros.

```bash
cargo add paste lazy-static async_once
cargo add tokio -F macros -F rt-multi-thread -F sync
```

### How to use

```rust:prom.rs
yzy_prom::initialize_yzy_prom!(
    counter = [
        my_app_request_total, "how many request for my app",
        lala, "lala metric detail",
    ],
    counter_vec = [
        my_app_http_request_total, "how many request for my app per endpoint", vec!("path", "status_code"),
    ],
);
```

Define metrics like this. then you can finde out metric functions are auto-generated

```rust:main.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    prom::launch().await; //Launches prometheus server on 127.0.0.1:9090

    prom::increment_my_app_http_request_total_countervec(vec!["/abc", "200"]).await;
}
```


### My vscode complains about macro error

First things first, we've got to update our rust lang & rust analyzer.
Older versions of rust-analyzer don't handle expansion of macros quite well.

```shell
rustup update
```

And also add this on VSCode's settings.json
```json
{
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.diagnostics.disabled": [
        "macro-error",
        "unresolved-macro-call",
        "unresolved-import"
    ],
}
```
