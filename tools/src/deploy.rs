use crate::{
    compile::compile, mibpf_common::BinaryFileLayout,
    postprocessing::apply_postprocessing, pull::pull, sign::sign,
};

const TEMP_FILE: &str = "program.bin";

pub async fn deploy(
    bpf_source_file: &str,
    out_dir: &str,
    binary_layout: BinaryFileLayout,
    coap_root: &str,
    suit_storage_slot: usize,
    riot_net_if: &str,
    riot_ip: &str,
    host_net_if: &str,
    host_ip: &str,
    board: &str,
    mibpf_root_dir: Option<&str>,
    helper_indices: Vec<u8>,
) -> Result<(), String> {
    let object_file_name = get_object_file_name(bpf_source_file, out_dir)?;
    let suit_manifest = &format!("suit_manifest{}.signed", suit_storage_slot);

    compile(bpf_source_file, Some(TEMP_FILE), out_dir)?;
    apply_postprocessing(&object_file_name, binary_layout, TEMP_FILE, helper_indices)?;
    sign(host_net_if, board, coap_root, TEMP_FILE, suit_storage_slot, mibpf_root_dir)?;
    pull(riot_ip, host_ip, suit_manifest, host_net_if, riot_net_if).await?;

    Ok(())
}

pub fn get_object_file_name(bpf_source_file: &str, out_dir: &str) -> Result<String, String> {
    let base_name = bpf_source_file.split("/").last().unwrap().split(".").nth(0);

    return match base_name {
        Some(name) => Ok(format!("{}/{}.o", out_dir, name)),
        None => Err("File not found: You need to provide the .c source file".to_string()),
    };
}