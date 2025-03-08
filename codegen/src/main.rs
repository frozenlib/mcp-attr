use anyhow::Result;
use mcp_attr_codegen::utils::save;

fn main() -> Result<()> {
    save(
        mcp_attr_codegen::schema::build_schema(
            "./external/modelcontextprotocol-specification/schema/2024-11-05/schema.json",
        )?,
        "./mcp-attr/src/schema.rs",
    )?;
    save(
        mcp_attr_codegen::transitivity2::build_transitivity(&[
            "./mcp-attr/src/schema.rs",
            "./mcp-attr/src/schema_ext.rs",
        ])?,
        "./mcp-attr/src/transitivity.rs",
    )?;
    Ok(())
}
