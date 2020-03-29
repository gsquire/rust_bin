# rust_bin
This is a stateless HTTP application used to try out Google's [Cloud Run](https://cloud.google.com/run)
environment. It is a simple service similar to [httpbin](https://httpbin.org/) with far fewer
features.

## Disclosure
I am in no way affiliated with Google and just wanted to give this service a spin. Being able to use
any language and Linux OS without having to worry about provisioning was appealing to me and I wanted
to test it out.

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
