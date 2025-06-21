## Strategy

Due to Cloudflare Worker's CPU limit, we cannot convert the JS `ReadableStream` to a Rust `ByteStream`. As such, we must make the readable stream available to be sent as the response body. However, the `reqwest` -> `object_store` -> `s3s` pipeline does not accomodate passing along the raw `ReadableStream`. As such, we must provide a custom `HttpConnector` to `object_store`, wherein we will manually insert the `ReadableStream` into a global variable when we first receive the response from the underlying Object Store. We can then insert that stream as a value into our outgoing response.

## Tips

```sh
export PATH="/opt/homebrew/opt/llvm/bin/:$PATH"
export CC=/opt/homebrew/opt/llvm/bin/clang
export AR=/opt/homebrew/opt/llvm/bin/llvm-ar
```
