use std::path::PathBuf;

/// eg: nid new . --name=project_name --template=template_name --yes
#[derive(clap::Parser, Debug)]
pub struct NewCommand {
    target: String,

    #[clap(short, long)]
    name: Option<String>,

    #[clap(short, long, default_value = "quickstart")]
    template: Option<String>,

    #[clap(short, long)]
    yes: bool,
}

impl NewCommand {
    pub fn run(&self) {
        // 1. check init or create dir
        // 2. git clone template
        // 3. replace template name
        // 4. git init
        let cur_dir = std::env::current_dir().unwrap();
        let is_init = self.target == ".";
        let (project_path, project_name) = if let Some(name) = &self.name {
            let name = name.to_string();
            let path = if is_init {
                cur_dir
            } else {
                std::path::PathBuf::from(&self.target)
            };
            (path.join(&name), name)
        } else {
            let path = if is_init {
                cur_dir
            } else {
                // 获取绝对路径
                cur_dir.join(&self.target)
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

        let is_exists = project_path.exists();

        let mut remove_yes = false;

        if is_exists {
            if !self.yes {
                let confirmation = dialoguer::Confirm::new()
                    .with_prompt(format!("The `{project_name}` directory is about to be overwritten, Do you want to continue?"))
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
            println!("The `{project_name}` directory already exists, please change the project name or delete the directory.");
            return;
        }

        // git clone
        std::process::Command::new("git")
            .arg("clone")
            .arg(&template_url)
            .arg(&project_path)
            .output()
            .expect("failed to execute process");

        // remove .git
        let git_path = project_path.join(".git");

        let _ = std::fs::remove_dir_all(git_path);

        println!("success!")
    }
}

struct CurrentDirEnv {
    cargo_toml: Option<PathBuf>,
    src_dir: Option<PathBuf>,
    base_dir: PathBuf,
    workspaces: Option<Vec<PathBuf>>,
    is_init_git: bool,
}

impl CurrentDirEnv {
    fn new() -> Self {
        let base_dir = std::env::current_dir().unwrap();
        let cargo_toml = base_dir.join("Cargo.toml");
        let src_dir = base_dir.join("src");
        let is_init_git = base_dir.join(".git").exists();
        let workspaces = None;

        Self {
            cargo_toml: Some(cargo_toml),
            src_dir: Some(src_dir),
            base_dir,
            workspaces,
            is_init_git,
        }
    }

    fn read_cargo_toml(&self) -> String {
        let cargo_toml = self.cargo_toml.as_ref().unwrap();
        let content = std::fs::read_to_string(cargo_toml).unwrap();
        content
    }
}
