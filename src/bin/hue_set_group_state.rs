extern crate hueclient;
extern crate regex;

use std::env;

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!(
            "usage : {:?} <username> <group_id>,<group_id>,... on|off|[bri]:[hue]:[sat]|[ct]MK:[bri]|[w]K:[bri]|[RR][GG][BB]:[bri]|[x,y]:[bri] [transition_time]",
            args[0]
        );
        return;
    }
    let bridge = hueclient::Bridge::discover_required()
        .await
        .with_user(args[1].to_string());
    let groups: &Vec<usize> = &args[2]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let parsed = hueclient::parse_command(args);

    println!("groups: {:?}", groups);
    for l in groups.iter() {
        println!("{:?}", bridge.set_group_state(*l, &parsed).await);
        std::thread::sleep(::std::time::Duration::from_millis(50))
    }
}
