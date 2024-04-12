use std::process::Command;

use mibpf_common::ExecutionModel;
use log::debug;

use crate::mibpf_common::{
    BinaryFileLayout, TargetVM, VMConfiguration, VMExecutionRequestMsg,
};

pub async fn execute(
    riot_ipv6_addr: &str,
    target: TargetVM,
    binary_layout: BinaryFileLayout,
    suit_storage_slot: usize,
    host_network_interface: &str,
    execution_model: ExecutionModel,
    helper_indices: &[u8],
) -> Result<String, String> {
    let request = VMExecutionRequestMsg {
        configuration: VMConfiguration::new(target, binary_layout, suit_storage_slot).encode(),
        available_helpers: encode(helper_indices),
    };

    let url = match execution_model {
        ExecutionModel::ShortLived => format!(
            "coap://[{}%{}]/vm/exec",
            riot_ipv6_addr, host_network_interface
        ),
        ExecutionModel::WithAccessToCoapPacket => {
            format!(
                "coap://[{}%{}]/vm/exec/coap-pkt",
                riot_ipv6_addr, host_network_interface
            )
        }
        ExecutionModel::LongRunning => format!(
            "coap://[{}%{}]/vm/spawn",
            riot_ipv6_addr, host_network_interface
        ),
        ExecutionModel::Benchmark => format!(
            "coap://[{}%{}]/vm/bench",
            riot_ipv6_addr, host_network_interface
        ),
    };

    debug!("Sending a request to the url: {}", url);

    let payload = serde_json::to_string(&request).unwrap();
    debug!("Request payload:\n{}", payload);

    // We use the aiocoap-client here as opposed to the rust coap library because
    // that one didn't support overriding the network interface in the ipv6 urls
    let Ok(output) = Command::new("aiocoap-client")
        .arg("-m")
        .arg("POST")
        .arg(url.clone())
        .arg("--payload")
        .arg(&payload)
        .output()
    else {
        return Err(format!("Failed to send request payload: {}", payload));
    };

    let response = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to parse the response: {}", e))?;

    Ok(response)
}

fn encode(available_indices: &[u8]) -> [u8; 3] {
    let mut encoding = [0; 3];
    for i in available_indices {
        // The first 8 helpers are configured by the first u8, the next
        // by the second one and so on.
        let bucket = (i / 8) as usize;
        encoding[bucket] |= 1 << (i % 8);
    }
    return encoding;
}