fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_list = &["proto/textchat.proto"];

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .build_server(true)
        .out_dir("src")
        .compile(proto_list, &["./proto/"])
        .unwrap();

    Ok(())
}
