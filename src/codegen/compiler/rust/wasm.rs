use std::fmt::Write;

use handlebars::{no_escape, Handlebars};
use heck::ToUpperCamelCase;

use crate::{
    codegen::{Compile, IntoWasmCompiler},
    schema::Schema,
};

use super::RustCompiler;

pub struct RustWasmCompiler(RustCompiler);

impl Compile for RustWasmCompiler {
    fn compile_schema(&self, schema: &Schema) -> String {
        let mut code = self.0.compile_schema(schema);

        self.compile_wasm(&mut code, schema);

        code
    }
}

impl RustWasmCompiler {
    fn compile_wasm(&self, code: &mut String, schema: &Schema) {
        use wit_bindgen_gen_core::{Files, Generator};
        use wit_bindgen_gen_rust_wasm::Opts;
        use wit_parser::Interface;

        let name = &schema.aggregate.name;

        #[derive(serde::Serialize)]
        struct TmplData<'a> {
            aggregate: &'a str,
            aggregate_event: String,
            aggregate_command: String,
            aggregate_command_enum: String,
            commands: Vec<TmplCommand<'a>>,
            wasi_wit_code: String,
        }

        #[derive(serde::Serialize)]
        struct TmplCommand<'a> {
            command_name: &'a str,
            command_name_variant: String,
            params: Vec<&'a str>,
        }

        let wasi_wit_code = {
            let iface =
                Interface::parse("domain", include_str!("../templates/domain.wit")).unwrap();
            let opts = Opts::default();
            let mut gen = opts.build();
            let mut files = Files::default();
            gen.generate_all(&[], &[iface], &mut files);

            files
                .iter()
                .map(|(_, contents)| String::from_utf8(contents.to_vec()).unwrap())
                .collect::<Vec<_>>()
                .join("\n")
        };

        let data = TmplData {
            aggregate: name,
            aggregate_event: format!("{name}Event"),
            aggregate_command: format!("{name}Command"),
            aggregate_command_enum: format!("{name}CommandEnum"),
            commands: schema
                .aggregate
                .commands
                .iter()
                .map(|(command_name, command)| TmplCommand {
                    command_name,
                    command_name_variant: command_name.to_upper_camel_case(),
                    params: command
                        .params
                        .iter()
                        .map(|param| param.name.as_str())
                        .collect(),
                })
                .collect(),
            wasi_wit_code,
        };

        let tmpl_str = include_str!("../templates/rust-wasi.hbs");
        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(no_escape);
        handlebars.set_strict_mode(true);
        handlebars
            .register_template_string("rust-wasi", tmpl_str)
            .unwrap();

        let wasi_code = handlebars.render_template(tmpl_str, &data).unwrap();

        writeln!(code, "{wasi_code}");
    }
}

impl IntoWasmCompiler for RustCompiler {
    type Compiler = RustWasmCompiler;

    fn into_wasm_compiler(self) -> Self::Compiler {
        RustWasmCompiler(self)
    }
}
