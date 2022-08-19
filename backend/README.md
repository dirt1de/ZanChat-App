## Chat service backend

First we need to generate the TLS/SSL key and certificate files. For more details, check out this link: https://web.dev/how-to-use-local-https/

```
brew install mkcert # If you have not mkcert installed

mkcert -install
mkcert localhost
```

After generating the TLS/SSL key and certificate, we need to generate the server codes from the proto file. Make sure you have `Rust` installed

```
cargo build
```

and then execute the following command in your terminal to run the service.

```
cargo run --bin chat_server
```
