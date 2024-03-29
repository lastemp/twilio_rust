# twilio_rust

This is an sdk that will be used by developers to seamlessly integrate with Twilio Gateway (https://www.twilio.com/docs).
Server-side SDKs (or Server-side helper libraries) make it easy for you to use Twilio's REST APIs, generate TwiML, and perform other common server-side programming tasks.

The sdk has below listed dependencies:
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [Reqwest](https://github.com/seanmonstar/reqwest) Rust HTTP Client
- [chrono](https://github.com/chronotope/chrono) provides all functionality needed to do correct operations on dates and times
- [base64](https://github.com/marshallpierce/rust-base64) Decode from Base64 format or encode into it
- [tokio](https://github.com/tokio-rs/tokio) A runtime for writing reliable, asynchronous applications

## installation

```
cargo install --git https://github.com/lastemp/twilio_rust
```

## Usage

Please find below information:

   - See [the examples](./examples/) for full working examples.
   
1. Create a `.env` file in the [the example_1](./examples/example_1/) directory:

   ```ini
   ACCOUNT_SID=***
   AUTH_TOKEN=***
   ```

   Update "ACCOUNT_SID" and "AUTH_TOKEN" with the correct values.   
 