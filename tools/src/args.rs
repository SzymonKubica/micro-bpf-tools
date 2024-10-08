use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Action {
    /// Compile the eBPF program.
    Compile {
        /// Name of the file containing the eBPF source code.
        #[arg(long)]
        bpf_source_file: String,

        /// Optional override for the name of the file resulting from the compilation
        /// It defaults to <source_file>.bin
        #[arg(long)]
        binary_file: Option<String>,

        /// Directory for the object files
        #[arg(long, default_value_t = String::from("./out"))]
        out_dir: String,
    },
    /// Modifies the ELF file resulting from compilation to make it compatible with the VM.
    Postprocessing {
        /// Name of the file containing the eBPF source code.
        #[arg(long)]
        source_object_file: String,
        /// Name of the binary file to be generated
        #[arg(long)]
        binary_file: Option<String>,
        /// Layout of the binary file that the VM should expect.
        /// Available options: OnlyTextSection, FemtoContainersHeader, ExtendedHeader, RawObjectFile,
        /// Determines which kind of postprocessing is applied to the ELF file.
        #[arg(long, default_value_t = String::from("ExtendedHeader"))]
        binary_layout: String,
        /// Controlls which indices of helpers are made available to the VM
        #[clap(long, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        helper_indices: Vec<u8>,
        /// Controlls the pipeline stage at which the helpers need to be
        /// verified
        #[arg(long, default_value_t = String::from("Runtime"))]
        helper_access_verification: String,
    },
    /// Sign the eBPF binary for SUIT update protocol. Generates  the manifest,
    /// signs it and places all files in the CoAP fileserver root directory.
    Sign {
        /// Network interface of the machine hosting the CoAP fileserver.
        /// Used to find the IPv6 address of the fileserver.
        #[arg(long, default_value_t = String::from("wlan0"))]
        host_network_interface: String,

        /// Name of the target microcontroller board.
        #[arg(long, default_value_t = String::from("nucleo-f439zi"))]
        board_name: String,

        /// Name of the coaproot directory from the CoAP fileserver will serve
        /// the files. The signed binary and manifest will be placed there
        #[arg(long, default_value_t = String::from("coaproot"))]
        coaproot_dir: String,

        /// Name of the binary to sign and place in the CoAP fileserver root
        /// directory.
        #[arg(long, default_value_t = String::from("a.bin"))]
        binary_name: String,

        /// SUIT storage slot (0 or 1) where the signed binary blob is intended
        /// bo be loaded.
        #[arg(long, short, default_value_t = 0)]
        suit_storage_slot: i32,
    },

    /// Sends a request to the RIOT instance to fetch the new signed binary
    /// and load it into the specified SUIT storage slot.
    Pull {
        /// IPv6 address of the RIOT instance.
        #[arg(long, default_value_t = String::from(""))]
        riot_ipv6_addr: String,

        /// IPv6 address of the desktop machine hosting the CoAP fileserver with
        /// the binary blob and SUIT manifest.
        #[arg(long, default_value_t = String::from(""))]
        host_ipv6_addr: String,

        /// Name of the signed SUIT manifest file present in the CoAP fileserver
        /// root directory.
        # [arg(long, default_value_t = String::from("suit_manifest0.signed"))]
        suit_manifest: String,

        /// Network interface of the machine hosting the CoAP fileserver.
        /// Used to find the IPv6 address of the fileserver.
        #[arg(long, default_value_t = String::from("wlan0"))]
        host_network_interface: String,

        /// Network interface of the RIOT instance
        #[arg(long, default_value_t = String::from("5"))]
        riot_network_interface: String,

        /// Target version of the eBPF vm. Available options: Femto-Containers, rBPF
        #[arg(long, default_value_t = String::from("rBPF"))]
        target: String,

        /// Layout of the binary file that the VM should expect.
        /// Available options: OnlyTextSection, FemtoContainersHeader, ExtendedHeader, RawObjectFile,
        #[arg(long, default_value_t = String::from("ExtendedHeader"))]
        binary_layout: String,

        /// SUIT storage slot (0 or 1) where the signed binary blob is intended
        /// bo be loaded.
        #[arg(long, short, default_value_t = 0)]
        suit_storage_slot: i32,

        /// Controlls which indices of helpers are made available to the VM
        #[clap(long, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        helper_indices: Vec<u8>,

        /// Controlls the pipeline stage at which the helpers need to be
        /// verified
        #[arg(long, default_value_t = String::from("Runtime"))]
        helper_access_verification: String,
        #[arg(long, default_value_t = String::from("ExecuteRequest"))]
        helper_access_list_source: String,

        #[arg(long, default_value_t = false)]
        erase: bool,
        /// Controlls whether the uploaded binary is intended for future
        /// jit compilation
        #[arg(long, short, default_value_t = false)]
        jit: bool,
    },
    /// Compiles, signs and initiates firmware pull in one step.
    Deploy {
        /// Name of the file containing the eBPF source code.
        #[arg(long)]
        bpf_source_file: String,

        /// Directory for the object files
        #[arg(long, default_value_t = String::from("./out"))]
        out_dir: String,

        /// Target version of the eBPF vm. Available options: Femto-Containers, rBPF
        #[arg(long, default_value_t = String::from("rBPF"))]
        target: String,

        /// Layout of the binary file that the VM should expect.
        /// Available options: OnlyTextSection, FemtoContainersHeader, ExtendedHeader, RawObjectFile,
        #[arg(long, default_value_t = String::from("ExtendedHeader"))]
        binary_layout: String,

        /// Network interface of the machine hosting the CoAP fileserver.
        /// Used to find the IPv6 address of the fileserver.
        #[arg(long, default_value_t = String::from("wlan0"))]
        host_network_interface: String,

        /// Network interface of the RIOT instance
        #[arg(long, default_value_t = String::from("5"))]
        riot_network_interface: String,

        /// Name of the target microcontroller board.
        #[arg(long, default_value_t = String::from("nucleo-f439zi"))]
        board_name: String,

        /// Name of the coaproot directory from the CoAP fileserver will serve
        /// the files. The signed binary and manifest will be placed there
        #[arg(long, default_value_t = String::from("coaproot"))]
        coaproot_dir: String,

        /// SUIT storage slot (0 or 1) where the signed binary blob is intended
        /// bo be loaded.
        #[arg(long, short, default_value_t = 0)]
        suit_storage_slot: i32,

        /// IPv6 address of the RIOT instance.
        // We set the default value so that the argument doesn't need to be specified when
        // we use the .env configuration.
        #[arg(long, default_value_t = String::from(""))]
        riot_ipv6_addr: String,

        /// IPv6 address of the desktop machine hosting the CoAP fileserver with
        /// the binary blob and SUIT manifest.
        #[arg(long, default_value_t = String::from(""))]
        host_ipv6_addr: String,

        /// Controlls which indices of helpers are made available to the VM
        #[clap(long, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        helper_indices: Vec<u8>,

        /// Controlls the pipeline stage at which the helpers need to be
        /// verified
        #[arg(long, default_value_t = String::from("Runtime"))]
        helper_access_verification: String,
        #[arg(long, default_value_t = String::from("ExecuteRequest"))]
        helper_access_list_source: String,
        #[arg(long, default_value_t = false)]
        erase: bool,
        /// Controlls whether the uploaded binary is intended for future
        /// jit compilation
        #[arg(long, short, default_value_t = false)]
        jit: bool,
    },
    /// Sends a request to the RIOT instance to execute the loaded eBPF bytecode
    /// from a specified SUIT storage slot.
    Execute {
        /// IPv6 address of the RIOT instance.
        #[arg(long, default_value_t = String::from(""))]
        riot_ipv6_addr: String,

        /// Target version of the eBPF vm. Available options: Femto-Containers, rBPF
        #[arg(long, default_value_t = String::from("rBPF"))]
        target: String,

        /// Layout of the binary file that the VM should expect.
        /// Available options: OnlyTextSection, FemtoContainersHeader, ExtendedHeader, RawObjectFile,
        #[arg(long, default_value_t = String::from("ExtendedHeader"))]
        binary_layout: String,

        /// SUIT storage slot (0 or 1) where the signed binary blob is intended
        /// bo be loaded.
        #[arg(long, short, default_value_t = 0)]
        suit_storage_slot: i32,

        /// Network interface of the machine hosting the CoAP fileserver.
        /// Used to find the IPv6 address of the fileserver.
        #[arg(long, default_value_t = String::from("wlan0"))]
        host_network_interface: String,

        /// Which execution model should be used by the vm, avaliable options: ShortLived,
        /// WithAccessToCoapPacket, LongRunning, see [`micro_bpf_common::ExecutionModel`]
        /// for more details.
        #[arg(long, default_value_t = String::from("ShortLived"))]
        execution_model: String,

        /// Controlls which indices of helpers are made available to the VM
        #[clap(long, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        helper_indices: Vec<u8>,
        /// Controlls the pipeline stage at which the helpers need to be
        /// verified
        #[arg(long, default_value_t = String::from("Runtime"))]
        helper_access_verification: String,
        #[arg(long, default_value_t = String::from("ExecuteRequest"))]
        helper_access_list_source: String,

        #[arg(short)]
        jit: bool,
        #[arg(long)]
        jit_compile: bool,
        #[arg(short)]
        benchmark: bool,

    },
}

/// Tools for compiling, signing, loading and executing eBPF programs for
/// micro_bpf.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The command that is to be performed. Available options: Compile, Sign,
    /// Pull, Execute.
    #[command(subcommand)]
    pub command: Action,

    /// Use the .env file to configure missing arguments. Defaults to sourcing
    /// the ".env" file, the loaded file can be overridden by setting the
    /// environment variable "DOTENV".
    #[arg(long, default_value_t = false)]
    pub use_env: bool,
}
