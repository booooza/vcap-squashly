# vcap-squashly
Turn VCAP_SERVICES into flat env vars.

This is a rewrite of [vcap-squash](https://github.com/cloudfoundry-community/vcap-squash/tree/master) in Rust.

## Download
Visit the [releases](https://github.com/booooza/vcap-squashly/releases/latest) page
and download the binary for your system

## Usage
This application will parse the `VCAP_SERVICES` environment variable and output the unix exports of the flattened version.

Flattened vcap environment variables start with the service name and append `_` for each nested credential.
For example:
```sh
$ VCAP_SERVICES='{ "user-provided": [ {
  "name": "myservice",
  "credentials": {
    "url": "myservice.com",
    "username": "josh",
    "password": "secret",
    "nested": {
      "key": "value",
      "number": 123
    }
  }
} ] }' ./vcap-squashly

export MYSERVICE_URL="myservice.com"
export MYSERVICE_USERNAME="josh"
export MYSERVICE_PASSWORD="secret"
export MYSERVICE_NESTED_KEY="value"
export MYSERVICE_NESTED_NUMBER=123
```

To set your environment variables using this output, use
```sh
eval "$(./vcap-squashly)"
```

### In Cloud Foundry
Add the proper `vcap-squashly` binary to your project root (depending on cf stack)

Create a `.profile.d/setenv.sh` file to push along with your repo
```sh
#!/bin/sh
eval $(./vcap-squashly)
```

## Development

### Test
To run the test suite, use
```sh
$ cargo test
```

Run the test suite in development/watch mode:
```sh
$ cargo install cargo-watch
$ cargo watch -x test
```

### Build local
To build a binary using your Rust env:
```sh
$ cargo build
```
