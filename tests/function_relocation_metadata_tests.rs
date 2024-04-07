mod common;

use common::{load_env, test_execution, test_execution_accessing_coap_pkt};
use internal_representation::BinaryFileLayout;

// This module contains end-to-end integration tests of the compile-upload-
// execute workflow of the eBPF programs on microcontrollers. It is recommended
// that the tests are run using a native RIOT instance running on the host
// desktop machine.
//
// The tests are set up in a way that each test file contains the expected return
// value on the first line in the source file. This testsuite extracts that information
// and compares it to the actual output returned in the response from the RIOT
// instance running the mibpf server.
//
// TODO: write up setup instructions
// TODO: allow for specifying environment externally
// TOD: update this doc to make it specific to the tested binary file layout.

#[tokio::test]
async fn printf() {
    test_function_relocation_metadata("printf.c").await;
}

#[tokio::test]
async fn bpf_fetch() {
    test_function_relocation_metadata("bpf_fetch.c").await;
}

#[tokio::test]
async fn bpf_store() {
    test_function_relocation_metadata("bpf_store.c").await;
}

#[tokio::test]
async fn bpf_strlen() {
    test_function_relocation_metadata("bpf_strlen.c").await;
}

#[tokio::test]
async fn bpf_fmt_s16_dfp() {
    test_function_relocation_metadata("bpf_fmt_s16_dfp.c").await;
}

#[tokio::test]
async fn bpf_fmt_u32_dec() {
    test_function_relocation_metadata("bpf_fmt_u32_dec.c").await;
}

// TODO: figure out why this doesn't work
#[ignore]
#[tokio::test]
async fn pc_relative_calls() {
    test_function_relocation_metadata("pc_relative_calls.c").await;
}

#[tokio::test]
async fn inlined_calls() {
    test_function_relocation_metadata("inlined_calls.c").await;
}

#[tokio::test]
async fn fletcher_32_checksum() {
    test_function_relocation_metadata("fletcher32_checksum.c").await;
}

// TODO: figure out why this doesn't work, the problem is some add with overflow.
#[ignore]
#[tokio::test]
async fn gcoap_response_format() {
    test_function_relocation_metadata_accessing_coap_pkt("gcoap_response_format.c").await;
}

/// Runs a test which deploys an eBPF script which is prepared to be compatible
/// with [`BinaryFileLayout::RawObjectFile`], the tested implementation on the
/// microcontroller resolves relocations once the program is loaded into memory,
/// then it patches the bytecode and executes the final resulting binary. The
/// return value of the program is returned in the CoAP response that is sent
/// following the request that was sent by this testsuite to the server running
/// on the target device.
///
/// It is important to note that those tests should serve as end-to-end sanity
/// checks rather than a full proof of correctness of the system. Since we can
/// only check for successful execution of the requests we make and then compare
/// the return value of the program to the initial expectation, we cannot guarantee
/// that all parts of the execution of the program were successful. For instance,
/// when testing programs that rely on the bpf_printf helper for logging, the
/// shell output of the tested riot instance should be examined to see if the
/// printed logs are what we would expect them to be.
///
/// Another limitation is that this testsuite was built with the native RIOT
/// instance in mind, which runs as a simulation on the host machine. Because
/// of this, they aren't able to test whether the GPIO-related helpers work
/// as expected.
///
/// Furthermore, because of the design of this testsuite, we can only test programs
/// that terminate quickly enough so that the microcontroller can send the CoAP
/// response with the return value of the program within the request timeout.
async fn test_function_relocation_metadata(test_program: &str) {
    let env = load_env();
    test_execution(
        test_program,
        BinaryFileLayout::FunctionRelocationMetadata,
        &env,
    )
    .await;
}

/// Tests execution of a given eBPF program which is expected to have access to
/// the incoming network packet that requested the execution of the VM. It
/// then tests whether the response received matches the one specified on the
/// first line of the test file.
async fn test_function_relocation_metadata_accessing_coap_pkt(test_program: &str) {
    let env = load_env();
    test_execution_accessing_coap_pkt(
        test_program,
        BinaryFileLayout::FunctionRelocationMetadata,
        &env,
    )
    .await;
}
