use std::{
    any,
    collections::HashMap,
    io::{BufRead, Write},
    path::PathBuf,
};

use crate::shared::exec_cmd;

/// eg: nid openapi "http://localhost:3000" --yes
#[derive(clap::Parser, Debug)]
pub struct Openapi {
    #[clap(default_value = "http://localhost:3000")]
    serve: String,

    #[clap(short, long, default_value = "./node_modules/@nidist/api-client")]
    out_dir: Option<String>,

    /// default: ts
    #[clap(short, long)]
    lang: Option<String>,

    #[clap(short, long)]
    yes: bool,
}

impl Openapi {
    pub fn run(&self) {
        let openapi_url = format!("{}/api-docs/openapi.json", self.serve);
        let openapi_json = OpenapiBuilder::new(&openapi_url);
        // println!("{:?}", openapi_json.openapi);

        let out_dir = self.out_dir.as_ref().unwrap();
        let out_dir_path = PathBuf::from(out_dir);
        let client_dir_path = out_dir_path.join("client");
        let client_index_file = client_dir_path.join("index.ts");

        if out_dir_path.exists() {
            if self.yes {
                println!("[Openapi] remove {:?}", out_dir_path);
                let _ = std::fs::remove_dir_all(&out_dir_path);
            } else {
                let mut input = String::new();
                print!(
                    "[Openapi] {:?} is exists, overwrite? (y/n): ",
                    out_dir_path.display()
                );
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut input).unwrap();
                if input.contains("y") {
                    println!("[Openapi] remove {:?}", out_dir_path);
                    let _ = std::fs::remove_dir_all(&out_dir_path);
                } else {
                    return;
                }
            }
        }

        let template_url = "https://github.com/nidrs/tempalte-client-js";
        // git clone
        exec_cmd(
            "Git",
            std::process::Command::new("git")
                .arg("clone")
                .arg("--progress")
                .arg(&template_url)
                .arg(&out_dir_path),
        )
        .unwrap();

        // remove .git
        let git_path = out_dir_path.join(".git");

        let _ = std::fs::remove_dir_all(git_path);

        let mut file = std::fs::File::create(&client_index_file).unwrap();
        file.write_all(openapi_json.to_ts().as_bytes()).unwrap();

        exec_cmd(
            "Build(0)",
            std::process::Command::new("npm")
                .arg("install")
                .current_dir(&client_dir_path),
        )
        .unwrap();

        exec_cmd(
            "Build(1)",
            std::process::Command::new("npm")
                .arg("run")
                .arg("build")
                .current_dir(&client_dir_path),
        )
        .unwrap();

        // exec_cmd(
        //     "Link",
        //     std::process::Command::new("npm")
        //         .arg("link")
        //         .current_dir(&client_dir_path),
        // )
        // .unwrap();

        println!("[Openapi] build api client sdk: {:?}", out_dir_path);
    }
}

pub struct OpenapiBuilder {
    pub openapi: serde_json::Value,
}

impl OpenapiBuilder {
    pub fn new(url: &str) -> Self {
        let openapi = reqwest::blocking::get(url).unwrap().text().unwrap();
        let openapi = serde_json::from_str(&openapi).unwrap();
        Self { openapi }
    }

    pub fn to_ts(&self) -> String {
        let mut controllers = HashMap::new();

        for (path, item_path) in self.openapi["paths"].as_object().unwrap() {
            if let Some(item) = item_path.as_object() {
                for (method, item) in item {
                    if let Some(opr) = item.as_object() {
                        let x_router = opr["x-router"].as_str().unwrap();
                        let x_controller = opr["x-controller"].as_str().unwrap();

                        let controller = controllers.entry(x_controller).or_insert_with(|| {
                            let mut controller = HashMap::new();
                            controller.insert(x_router, (method, path, opr));
                            controller
                        });

                        controller.insert(x_router, (method, path, opr));
                    }
                }
            }
        }

        // println!("{:#?}", controllers);

        let mut ts =
            "// @ts-nocheck eslint-disable prettier-ignore\nimport { reqHandler, resHandler } from \"@nidrs/openapi-client-js\";\n\n".to_string();

        for (controller, router) in &controllers {
            ts.push_str(&format!(
                "/* prettier-ignore */\nexport class {} {{\n",
                controller
            ));
            ts.push_str("  constructor(private api: Api) {}\n");
            for (router, opr) in router {
                let method = opr.0;
                let path = opr.1;
                ts.push_str(&format!("  async {}(dto:any) {{\n", router));
                ts.push_str(&format!(
                    "    return resHandler(await this.api.request(reqHandler(dto, '{method}', '{path}', this.api.openapi)))\n"
                ));
                ts.push_str("  }\n");
            }
            ts.push_str("}\n");
        }

        ts.push_str("/* prettier-ignore */\nexport class Api {\n");
        for (controller, _) in controllers {
            let key = to_camel_case(controller).replace("Controller", "");
            ts.push_str(&format!("  {} = new {}(this);\n", key, controller));
        }
        ts.push_str(&format!("  openapi = {};\n", self.openapi));
        ts.push_str("  constructor(public request: any) {}\n");
        ts.push_str("}\n");

        ts
    }
}

/// 大驼峰 转 小驼峰
fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            result.push(c.to_ascii_lowercase());
        } else if c == '_' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
    }

    result
}
