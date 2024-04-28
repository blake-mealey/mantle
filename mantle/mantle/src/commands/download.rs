use yansi::Paint;

use rbx_mantle::{
    config::{load_project_config, StateConfig},
    state::{get_state, save_state},
};

pub async fn run(project: Option<&str>, key: Option<&str>) -> i32 {
    logger::start_action("Download state file:");
    let config_file = match load_project_config(project) {
        Ok(v) => v,
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };

    // config_file.header.

    if !matches!(config_file.header.state, StateConfig::Remote(_)) {
        logger::end_action(Paint::red("Project is not configured with remote state"));
        return 1;
    }

    let state = match get_state(&config_file.project_path, &config_file.header).await {
        Ok(v) => v,
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };

    let state_config = match key {
        Some(key) => StateConfig::LocalKey(key.to_owned()),
        None => StateConfig::Local,
    };
    match save_state(&config_file.project_path, &state_config, &state).await {
        Ok(_) => {}
        Err(e) => {
            logger::end_action(Paint::red(e));
            return 1;
        }
    };
    logger::end_action("Succeeded");

    0
}
