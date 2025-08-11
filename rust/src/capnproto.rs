use capnp::message::{Builder, ReaderOptions};
use capnp::serialize;

use crate::constants;
use crate::message_rust_capnp::{
    message, message_data, FarcasterNetwork, HashScheme, MessageType, SignatureScheme,
};

/// Encodes a nested Farcaster message using Cap'n Proto.
pub fn encode() -> Vec<u8> {
    // --- 1. Build the inner MessageData ---
    let mut message_data_builder = Builder::new_default();
    {
        let mut msg_data = message_data_builder.init_root::<message_data::Builder>();
        msg_data.set_timestamp(constants::SAMPLE_TIMESTAMP);
        msg_data.set_type(MessageType::CastAdd);
        msg_data.set_network(FarcasterNetwork::Devnet);
        msg_data.set_fid(&constants::SAMPLE_FID);

        // Build the CastAddBody inside the MessageData's body union
        let body = msg_data.init_body();
        let mut cast_add_body = body.init_cast_add_body();
        cast_add_body.set_text(constants::SAMPLE_TEXT);

        cast_add_body.reborrow().init_embeds(0);
        cast_add_body.reborrow().init_mentions(0);

        // Build the parent CastId within the CastAddBody's parent union
        let parent = cast_add_body.init_parent();
        let mut cast_id = parent.init_cast_id();
        cast_id.set_fid(&constants::SAMPLE_FID);
        cast_id.set_ts_hash(&constants::SAMPLE_TS_HASH);
    }

    // Serialize the inner MessageData to a byte vector
    let mut message_data_bytes = Vec::new();
    serialize::write_message(&mut message_data_bytes, &message_data_builder).unwrap();

    // --- 2. Build the outer Message ---
    let mut message_builder = Builder::new_default();
    {
        let mut msg = message_builder.init_root::<message::Builder>();
        msg.set_data(&message_data_bytes);
        msg.set_hash(&constants::SAMPLE_HASH);
        msg.set_hash_scheme(HashScheme::Blake3);
        msg.set_signature(&constants::SAMPLE_SIGNATURE);
        msg.set_signature_scheme(SignatureScheme::Ed25519);
        msg.set_signer(&constants::SAMPLE_SIGNER);
    }

    // --- 3. Serialize the final outer Message ---
    let mut final_bytes = Vec::new();
    serialize::write_message(&mut final_bytes, &message_builder).unwrap();
    final_bytes
}

/// Decodes a nested Farcaster message from a byte slice using Cap'n Proto.
pub fn decode(buf: &[u8]) {
    // --- 1. Read the outer Message ---
    let message_reader = serialize::read_message(buf, ReaderOptions::new()).unwrap();
    let msg = message_reader.get_root::<message::Reader>().unwrap();

    // --- 2. Extract and read the inner MessageData ---
    let data_bytes = msg.get_data().unwrap();
    let inner_message_reader = serialize::read_message(data_bytes, ReaderOptions::new()).unwrap();
    let msg_data = inner_message_reader
        .get_root::<message_data::Reader>()
        .unwrap();

    // --- 3. Verify the decoded content ---
    match msg_data.get_body().which() {
        // Case 1: Everything succeeded.
        Ok(message_data::body::Which::CastAddBody(Ok(cast_add_body))) => {
            if cast_add_body.get_text().unwrap() != constants::SAMPLE_TEXT {
                panic!(
                    "Unexpected decoded text. Got '{:?}', expected '{}'",
                    cast_add_body.get_text().unwrap(),
                    constants::SAMPLE_TEXT
                );
            }
        }
        // Case 2: The union is `Void`.
        Ok(message_data::body::Which::Void(_)) => {
            panic!("Expected message body to be CastAddBody, but got Void")
        }
        // This handles when the union tag is `CastAddBody`, but reading its content fails.
        Ok(message_data::body::Which::CastAddBody(Err(e))) => {
            panic!("Failed to read CastAddBody content: {:?}", e)
        }
        // Case 3: Reading the union tag itself failed.
        Err(e) => panic!("Error reading message body: {:?}", e),
    }
}
