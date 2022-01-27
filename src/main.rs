mod utils;

use crate::utils::read_from_toml;
use dirs::config_dir;
use docker_api::{Docker, api::ContainerListOpts};

const NAME: &str =env!("CARGO_PKG_NAME");
const DESCRIPTION: &str =env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str =env!("CARGO_PKG_VERSION");
const AUTHORS: &str =env!("CARGO_PKG_AUTHORS");

#[tokio::main]
async fn main() {
    /*
    let config_path = config_dir().unwrap()
        .join("dockerwarn")
        .join("dockerwarn.conf");
    if !config_path.exists(){
        println!("Configure Docker Warning System");
        return;
    }
    let config = read_from_toml(config_path.to_str().unwrap());
    */
    let docker: Docker = Docker::unix("/var/run/docker.sock");
    let opts = ContainerListOpts::builder().all(true).build();
    match docker.containers().list(&opts).await{
        Ok(containers) => {
            containers.into_iter().for_each(|container|{
                println!(
                    "{}\t{}\t{:?}\t{}\t{}",
                    &container.id[..12],
                    container.image,
                    container.state,
                    container.status,
                    container.names[0]
                );
            })
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
