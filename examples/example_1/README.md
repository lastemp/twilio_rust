# example_1

This is a full working example which uses the [twilio_rust sdk](https://github.com/lastemp/twilio_rust).
The API endpoints provided by Twilio Gateway includes; SMS, Whatsapp, USSD, Voice etc (https://www.twilio.com/docs). 

The example has below listed dependencies:
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [tokio](https://github.com/tokio-rs/tokio) A runtime for writing reliable, asynchronous applications
- [twilio_rust](https://github.com/lastemp/twilio_rust) an sdk to seamlessly integrate with Twilio Gateway

## Usage

All the following commands assume that your current working directory is _this_ directory. I.e.:

```console
$ pwd
.../example_1
```

1. Create a `.env` file in this this directory:

   ```ini
   ACCOUNT_SID=***
   AUTH_TOKEN=***
   ```

   Update "ACCOUNT_SID" and "AUTH_TOKEN" with the correct values.   

1. Using a different terminal execute requests by un-commenting code for the spefific function on main.rs. For example:

   ```rust
	dotenv().ok();
    let account_sid = env::var("ACCOUNT_SID").expect("ACCOUNT_SID is not set in .env file");
    let auth_token = env::var("AUTH_TOKEN").expect("AUTH_TOKEN is not set in .env file");
    // PART A: TESTS
    // sms
    let x = create_sms::create_sms_message(account_sid, auth_token);

    // whatsapp
    // let x = create_whatsapp::create_whatsapp_message(account_sid, auth_token);

    // let x = fetch_message::fetch_message(account_sid, auth_token);
    // let x = fetch_multiple_messages::fetch_multiple_message(account_sid, auth_token);

    // let x = fetch_media_message::fetch_media_message(account_sid, auth_token);
    // let x = fetch_multiple_media_message::fetch_multiple_media_message(account_sid, auth_token);

    // let x = update_message::update_message(account_sid, auth_token);

    // let x = delete_message::delete_message(account_sid, auth_token);
    // let x = delete_media_message::delete_media_message(account_sid, auth_token);

    x.await;
	}
   ```

1. Run the application:

   ```sh
   cargo run
   ```
