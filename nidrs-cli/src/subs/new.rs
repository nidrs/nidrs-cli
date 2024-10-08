use std::{
    any,
    io::{BufRead, Write},
    path::PathBuf,
};

use serde::de::Error;
use toml::Table;

use crate::shared::exec_cmd;

/// eg: nid new . --name=project_name --template=template_name --yes
#[derive(clap::Parser, Debug)]
pub struct New {
    target: String,

    #[clap(short, long)]
    name: Option<String>,

    #[clap(short, long, default_value = "quickstart")]
    template: Option<String>,

    #[clap(short, long)]
    yes: bool,
}

impl New {
    pub fn run(&self) {
        // 1. check init or create dir
        // 2. git clone template
        // 3. replace template name
        // 4. git init
        let we = WorkEnv::new(std::env::current_dir().unwrap(), true).init();

        let is_init = self.target == "."; // is init mode
        let (project_path, project_name) = if let Some(name) = &self.name {
            let name = name.to_string();
            let path = if is_init {
                we.base_dir().clone()
            } else {
                std::path::PathBuf::from(&self.target)
            };
            (path.join(&name), name)
        } else {
            let path = if is_init {
                we.base_dir().clone()
            } else {
                // 获取绝对路径
                we.base_dir().clone().join(&self.target)
            };
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            (path, name)
        };

        // https://github.com/nidrs/quickstart-template
        let template = self.template.as_ref().unwrap();
        let template_url = format!("https://github.com/nidrs/{template}-template");

        // println!(
        //     "project_name: {:?}",
        //     (&project_path, &project_name, &template_url)
        // );
        // println!("debug: {:?}", self);
        if !is_init {
            let is_exists = project_path.exists();

            let mut remove_yes = false;

            if is_exists {
                if !self.yes {
                    let confirmation = dialoguer::Confirm::new()
                        .with_prompt(format!("[nid] The `{project_name}` directory is about to be overwritten, Do you want to continue?"))
                        .interact()
                        .unwrap();

                    if confirmation {
                        remove_yes = true;
                    }
                } else {
                    remove_yes = true;
                }

                if remove_yes {
                    // remove project_path
                    let _ = std::fs::remove_dir_all(&project_path);
                }
            }
            if is_exists && !remove_yes {
                println!("[nid] The `{project_name}` directory already exists, please change the project name or delete the directory.");
                return;
            }
        }

        // git clone
        exec_cmd(
            "git",
            std::process::Command::new("git")
                .arg("clone")
                .arg("--progress")
                .arg(&template_url)
                .arg(&project_path),
        )
        .unwrap();

        // remove .git
        let git_path = project_path.join(".git");

        let _ = std::fs::remove_dir_all(git_path);

        // replace template name
        // each all files
        let mut files = vec![];
        let mut dirs = vec![project_path.clone()];
        while let Some(dir) = dirs.pop() {
            let entries = std::fs::read_dir(&dir).unwrap();
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path.clone());
                } else {
                    files.push(path);
                }
            }
        }

        for file in files {
            let content = std::fs::read_to_string(&file).unwrap();
            let content = content.replace("TEMPLATE_NAME", &project_name);
            std::fs::write(&file, content).unwrap();
        }

        if !we.is_init_git() {
            // git init and print
            std::process::Command::new("git")
                .arg("init")
                .current_dir(&project_path)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .output()
                .expect("failed to execute process");
        }

        println!("[nid] success!")
    }
}

#[derive(Debug)]
struct WorkEnv {
    cargo_toml: Option<PathBuf>,
    base_dir: PathBuf,
    is_init_git: bool,
    is_root: bool,
    workspaces: Vec<WorkEnv>,
}

impl WorkEnv {
    fn new(base_dir: PathBuf, is_root: bool) -> Self {
        let cargo_toml = base_dir.join("Cargo.toml");
        let is_init_git = base_dir.join(".git").exists();
        let workspaces = vec![];

        Self {
            cargo_toml: Some(cargo_toml),
            base_dir,
            workspaces,
            is_init_git,
            is_root: is_root,
        }
    }

    fn init(mut self) -> Self {
        let cargo_toml = self.read_cargo_toml();
        let workspace = cargo_toml.get("workspace");
        if let Some(workspace) = workspace {
            if let Some(workspace) = workspace.as_table() {
                let members = workspace.get("members");
                if let Some(members) = members {
                    if let Some(members) = members.as_array() {
                        for member in members {
                            let member = member.as_str().unwrap();
                            let member_path = self.base_dir.join(member);
                            let we = WorkEnv::new(member_path, false);
                            self.mut_workspace().push(we.init());
                        }
                    }
                }
            }
        }

        return self;
    }

    fn base_dir(&self) -> &PathBuf {
        &self.base_dir
    }

    fn is_root(&self) -> bool {
        self.is_root
    }

    fn is_init_git(&self) -> bool {
        self.is_init_git
    }

    fn workspace(&self) -> &Vec<WorkEnv> {
        self.workspaces.as_ref()
    }

    fn mut_workspace(&mut self) -> &mut Vec<WorkEnv> {
        self.workspaces.as_mut()
    }

    fn read_cargo_toml(&self) -> Table {
        let cargo_toml = self.cargo_toml.as_ref().unwrap();
        let content = std::fs::read_to_string(cargo_toml).unwrap_or("".to_string());
        let table: Table = toml::from_str(&content).unwrap();
        table
    }

    fn write_cargo_toml(&self, table: Table) {
        let cargo_toml = self.cargo_toml.as_ref().unwrap();
        let content = toml::to_string(&table).unwrap();
        std::fs::write(cargo_toml, content).unwrap();
    }
}
