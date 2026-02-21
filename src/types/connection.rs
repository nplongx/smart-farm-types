use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ConnectionConfig {
    Mqtt {
        mqtt_topic_sub: Option<String>,
        mqtt_topic_pub: Option<String>,
        json_key: Option<String>,
        payload_on: Option<String>,
        payload_off: Option<String>,
    },

    Gpio {
        pin: u8,
        // active_low: bool,
    },
}
