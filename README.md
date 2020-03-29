# rust_bin
![CI](https://github.com/gsquire/rust_bin/workflows/CI/badge.svg?branch=master&event=push)

This is a stateless HTTP application used to try out Google's [Cloud Run](https://cloud.google.com/run)
environment. It is a simple service similar to [httpbin](https://httpbin.org/) with far fewer
features.

## Disclosure
I am in no way affiliated with Google and just wanted to give this service a spin. Being able to use
any language and Linux OS without having to worry about provisioning was appealing to me and I wanted
to test it out.

There is no guarantee that the sample URL will always be available as well. If it becomes costly, then
I will delete the project but code will remain in this repository.

## Trying it Out
If you want to test a running instance, you can see the API by hitting the index page
[here](https://rustbin-j5pswym3zq-uw.a.run.app/). It will display a few routes that you can then
use `curl` or your favorite HTTP library to run requests against.

Sample Request:

```sh
# If you have `jq` installed:
curl -s https://rustbin-j5pswym3zq-uw.a.run.app/user-agent | jq .

# Response:
{
  "user-agent": "curl/7.54.0"
}
```

## Setup
If you wish to run the service locally, you can use the specified `Dockerfile` and build it with the
following command:

```sh
docker build -t rustbin:latest .
```

```sh
docker run -it -e PORT=8080 -p 8080:8080 rustbin:latest
```

If you have a nightly Rust compiler locally you can build and test the service via these commands:

```sh
cargo +nightly build && cargo +nightly test
```

## Guide
I followed [this guide](https://cloud.google.com/run/docs/quickstarts/build-and-deploy) to configure
the `Dockerfile` and deploy it to GCP.

## License
MIT
