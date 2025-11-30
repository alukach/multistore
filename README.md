# MultiStore

MultiStore is an application to easily create an S3-compliant API for one-or-many object store backends.

The system could be utilized to...

1. create custom access credentials (distinct from AWS credentials) to be given out to grant access
2. collect detailed usage metrics
3. gate access to datasets
4. bill users for dataset access

## Development

### Running Hyper API

```sh
cargo run --bin hyper-api
```

### Running Cloudflare Workers API

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
