use std::fs;
use std::path::Path;

fn main() {
    let openrpc_path = Path::new("../shared/openrpc.json");
    let out_path = Path::new("src/generated.rs");

    println!("cargo:rerun-if-changed={}", openrpc_path.display());
    println!("cargo:rerun-if-changed=build.rs");

    // Read the OpenRPC spec
    let openrpc_content = fs::read_to_string(openrpc_path).expect("Failed to read openrpc.json");
    let openrpc: serde_json::Value =
        serde_json::from_str(&openrpc_content).expect("Failed to parse openrpc.json");

    // Extract the JSON Schema from OpenRPC's components.schemas
    let schemas = openrpc
        .get("components")
        .and_then(|c| c.get("schemas"))
        .expect("OpenRPC must have components.schemas");

    // Create a JSON Schema document with definitions
    let json_schema = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "definitions": schemas
    });

    // Convert $refs from OpenRPC format to JSON Schema format
    let json_schema_str = serde_json::to_string(&json_schema)
        .expect("Failed to serialize schema")
        .replace("#/components/schemas/", "#/definitions/");

    let schema: serde_json::Value =
        serde_json::from_str(&json_schema_str).expect("Failed to parse converted schema");

    // Generate Rust types with typify
    let mut type_space = typify::TypeSpace::default();
    type_space
        .add_root_schema(serde_json::from_value(schema).expect("Failed to convert schema"))
        .expect("Failed to add schema to type space");

    let code = type_space.to_stream().to_string();

    // Format with prettyplease
    let formatted = prettyplease_format(&code).unwrap_or(code);

    fs::write(out_path, formatted).expect("Failed to write generated.rs");
}

fn prettyplease_format(code: &str) -> Option<String> {
    let syntax_tree = syn::parse_file(code).ok()?;
    Some(prettyplease::unparse(&syntax_tree))
}
