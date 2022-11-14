use glob::glob;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn cmd(cmd: &str, quiet: bool) {
    let mut c = Command::new("python");
    if quiet == true {
        c.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    } else {
        c.stdout(Stdio::null()).stderr(Stdio::null());
    }

    for arg in cmd.split_whitespace() {
        c.arg(arg);
    }

    match c.output() {
        Ok(result) => println!("{}", String::from_utf8(result.stdout).unwrap()),
        Err(e) => println!(
            "Error has occured running command: python {} : {:?}",
            cmd, e
        ),
    }
}

fn generate_protobuf_files() {
    println!("Generating protobuf codes for pybess...");
    // std::io::stdout().flush().unwrap();
    gen_one_set_of_files("protobuf", "pybess/builtin_pb");
    gen_one_set_of_files("protobuf/tests", "pybess/builtin_pb");
    // for path in plugins {
    //     gen_one_set_of_files(std::path::Path::new(&format!("{}/{}",path, "protobuf")),"pybess/plugin_pb");
    // }
}
fn gen_one_set_of_files(srcdir: &str, outdir: &str) {
    // run grpc_tools.protoc on *.proto in srcdir, with python output to outdir"
    // python -m grpc_tools.protoc -I. --python_out=. --grpc_python_out=. helloworld.proto
    // let stddir = "protobuf";
    let files = glob(
        std::path::Path::new(&format!("{}/{}", srcdir, "*.proto"))
            .to_str()
            .unwrap(),
    )
    .unwrap();
    for file in files {
        match file {
            Ok(proto_file) => {
                let cmd_args= &format!("-m grpc_tools.protoc --proto_path={srcdir} --python_out={outdir} --grpc_python_out={outdir} {}", proto_file.display());
                cmd(cmd_args, false);
            }
            Err(e) => println!("{:?}", e),
        }
    }
    // Note: when run as, e.g.
    // grpc_tools.protoc protobuf/ports/port_msg.proto \
    //        --proto_path=protobuf \
    //        --python_out=pybess/builtin_pb ...
    // protoc writes its output to:
    //    pybess/builtin_pb/ports/port_msg.proto
    // which is automatically where we want it.
    let files = glob(
        std::path::Path::new(&format!("{}/{}/{}", srcdir, "ports", "*.proto"))
            .to_str()
            .unwrap(),
    )
    .unwrap();
    for file in files {
        match file {
            Ok(proto_file) => {
                let cmd_args= &format!("-m grpc_tools.protoc --proto_path={srcdir} --python_out={outdir} --grpc_python_out={outdir} {}", proto_file.display());
                cmd(cmd_args, false);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // build_data::set_GIT_BRANCH();
    // build_data::set_GIT_COMMIT();
    // build_data::set_GIT_DIRTY();
    // build_data::set_SOURCE_TIMESTAMP();
    // build_data::no_debug_rebuilds();
    build_data::set_RUSTC_VERSION();
    // build_data::set_RUSTC_VERSION_SEMVER();

    generate_protobuf_files();
    tonic_build::compile_protos("protobuf/service.proto")?;
    Ok(())
}
