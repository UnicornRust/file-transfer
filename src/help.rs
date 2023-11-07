pub const DEFAULT_PORT: u16 = 3001;
pub const BUFFER_LENGTH: usize = 1024 * 1024 * 20;
/// 当前模块主要是该工具的帮助信息，与一些默认的配置
pub fn help() {
    println!("------------------------------------------------------------------");
    println!("transfer");
    println!("sub command could be");
    println!("receiver:");
    println!("       --port | -p: use this argument to instend of default listen port 3001");
    println!("sender: ");
    println!("       --ip-port | -i: specify server ip and port which you are going to sender file to");
    println!("                   eg: --ip-port=192.168.1.1:9000");
    println!("       --ip : specify server ip which you are going to sender file to, use default port 3001");
    println!("                   eg: --ip-port 192.168.1.1");
    println!("help:");
    println!("       command usage help");
}
