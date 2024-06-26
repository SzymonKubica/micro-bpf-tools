use std::process::Command;

use log::{debug, error};

pub fn sign(
    host_network_interface: &str,
    board_name: &str,
    coaproot_dir: &str,
    binary_name: &str,
    suit_storage_slot: usize,
    mibpf_root_dir: Option<&str>,
) -> Result<(), String> {
    place_binary_in_coap_root(coaproot_dir, binary_name);

    let file_name = binary_name.split("/").last().unwrap();

    let mibpf_home = match mibpf_root_dir {
        Some(dir) => dir,
        // The default assumes that the tools script is run from the root of the mibpf repo.
        None => ".",
    };

    let Ok(output) = Command::new("bash")
        .env("RIOT_HOME", &format!("{}/RIOT", mibpf_home))
        .arg(&format!("{}/scripts/sign-binary.sh", mibpf_home))
        .arg(host_network_interface)
        .arg(board_name)
        .arg(coaproot_dir)
        // The file should have been copied to coaproot by now.
        .arg(format!("{}/{}", coaproot_dir, file_name))
        .arg(suit_storage_slot.to_string())
        .output()
    else {
        return Err("Failed to sign the binary".to_string());
    };

    debug!(
        "sign.sh output: \n{}",
        String::from_utf8(output.stdout).unwrap()
    );
    if output.stderr.len() > 0 {
        error!(
            "Error while signing: \n{}",
            String::from_utf8(output.stderr).unwrap()
        );

    }

    Ok(())
}

fn place_binary_in_coap_root(coaproot_dir: &str, binary_name: &str) {
    let _ = Command::new("mv")
        .arg(binary_name)
        .arg(coaproot_dir)
        .spawn()
        .expect("Failed to copy the binary file.")
        .wait();
}
