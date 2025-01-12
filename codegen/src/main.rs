use anyhow::Result;
use mcp_attrs_codegen::utils::save;

fn main() -> Result<()> {
    save(
        mcp_attrs_codegen::schema::build_schema(
            "./external/modelcontextprotocol-specification/schema/2024-11-05/schema.json",
        )?,
        "./mcp-attr/src/schema.rs",
    )?;
    save(
        mcp_attrs_codegen::transitivity2::build_transitivity(&[
            "./mcp-attr/src/schema.rs",
            "./mcp-attr/src/schema_ext.rs",
        ])?,
        "./mcp-attr/src/transitivity.rs",
    )?;
    Ok(())
}
