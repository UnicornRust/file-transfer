use std::env;
//
// 获取本机的 hostname, ip
pub mod hostname_ip;
// 显示文件信息
pub mod display;
// 根据 ip 和端口号发送文件和文件夹
pub mod sender;
// 接收文件和文件夹
pub mod receiver;
// 输出帮助信息
pub mod help;

fn main() {
    // 获取参数
    let args = env::args().collect::<Vec<String>>();
    let args = args.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    match args[1] {
        "sender" => sender::send(&args[2..]),
        "receiver" => receiver::receive(&args[2..]),
        "help" => help::help(),
        _ => panic!("sub command must be :: sender | receiver | help"),
    }
}
