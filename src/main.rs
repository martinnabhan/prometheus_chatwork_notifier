use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Input {
    version: String,
    #[serde(rename = "groupKey")]
    group_key: String,
    //  status: "<resolved|firing>",
    receiver: String,
    // groupLabels: <object>,
    // commonLabels: <object>,
    // commonAnnotations: <object>,
    // externalURL: String,
    // alerts: [
    //   {
    //     status: "<resolved|firing>",
    //     labels: <object>,
    //     annotations: <object>,
    //     startsAt: "<rfc3339>",
    //     endsAt: "<rfc3339>",
    //     generatorURL: String
    //   },
    // ]
}

#[derive(Serialize)]
struct Output {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(handler);

    Ok(())
}

fn handler(_e: Input, _c: Context) -> Result<Output, HandlerError> {
    Ok(Output {
        message: "OK".to_string(),
    })
}
