# MultiStore

## Development

### Running Axum

```sh
cargo run --bin axum-api
```

### Running Lambda

Lambda execution makes use of the [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime).

```sh
cargo lambda watch --bin lambda-api
```

### Accessing the API

```sh
AWS_EC2_METADATA_DISABLED=true AWS_ACCESS_KEY_ID=foo AWS_SECRET_ACCESS_KEY=bar aws s3api --endpoint-url http://localhost:9000/lambda-url/lambda-api --no-cli-pager list-buckets
```
