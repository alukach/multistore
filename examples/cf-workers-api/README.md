# Cloudflare Workers Example

Run the project on the [Cloudflare Workers](https://workers.cloudflare.com/) runtime.

**Upsides:**

- Serverless architecture
- Minimal cold-start[^cold-starts]
- Generous free tier (100K req/day, 1K req/min)[^worker-limits]
- No wall-clock timeout[^worker-limits]
- No additional data transfer (egress) or throughput (bandwidth) fees[^pricing]

**Downsides:**

- Low CPU timeout (10 ms)[^worker-limits]
- Max `POST` body payload of 100 MB[^request-limits]
- If making requests to AWS S3 buckets from an AWS environment within the same region, AWS egress fees are not avoided

## Strategy

Due to Cloudflare Worker's CPU limit, we cannot convert the JS `ReadableStream` to a Rust `ByteStream`. As such, we must make the readable stream available to be sent as the response body. However, the `reqwest` -> `object_store` -> `s3s` pipeline does not accomodate passing along the raw `ReadableStream`. As such, we must provide a custom `HttpConnector` to `object_store`, wherein we will manually insert the `ReadableStream` into a global variable when we first receive the response from the underlying Object Store. We can then insert that stream as a value into our outgoing response.

## Development

### `ring` compilation

**Problem:**

```
[custom build] warning: ring@0.17.14: error: unable to create target: 'No available targets are compatible with triple "wasm32-unknown-unknown"'
[custom build] warning: ring@0.17.14: 1 error generated.
```

**Fix:**

```sh
export PATH="/opt/homebrew/opt/llvm/bin/:$PATH"
export CC=/opt/homebrew/opt/llvm/bin/clang
export AR=/opt/homebrew/opt/llvm/bin/llvm-ar
```

[^cold-starts]: https://blog.cloudflare.com/eliminating-cold-starts-with-cloudflare-workers/
[^worker-limits]: https://developers.cloudflare.com/workers/platform/limits/#worker-limits
[^pricing]: https://developers.cloudflare.com/workers/platform/pricing/
[^request-limits]: https://developers.cloudflare.com/workers/platform/limits/#request-limits
