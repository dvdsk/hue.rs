extern crate hueclient;
use std::env;

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage : {:?} <username>", args[0]);
        return;
    }
    let bridge = hueclient::Bridge::discover_required().await.with_user(args[1].to_string());
    match bridge.get_all_scenes().await {
        Ok(scenes) => {
            println!("id name");
            for ref l in scenes.iter() {
                println!("{:2} {:40}", l.id, l.scene.name,);
            }
        }
        Err(err) => {
            println!("Error: {}", err);
            ::std::process::exit(2)
        }
    }
}
