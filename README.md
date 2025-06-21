# MultiStore

## Development

### Running Hyper API

```sh
cargo run --bin hyper-api
```

### Running Workers API

```sh
npx wrangler dev --cwd examples/cf-workers-api
```

### Running Lambda API

Lambda execution makes use of the [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime).

```sh
cargo lambda watch --bin lambda-api
```

### Accessing the API

```sh
export AWS_MAX_ATTEMPTS=1
export AWS_EC2_METADATA_DISABLED=true
export ENDPOINT_URL=http://localhost:9000/lambda-url/lambda-api
export AWS_ACCESS_KEY_ID=foo 
export AWS_SECRET_ACCESS_KEY=bar
```

```sh
aws \
--endpoint-url ${ENDPOINT_URL} \
--no-cli-pager \
s3api list-buckets
```
