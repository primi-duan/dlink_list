mod input;
mod dlink_list;

use std::io;

fn main() {
    println!("Hello world by roc-duan!");
    let msg = input::proc_in::help();
    println!("{}", msg);

    let mut d_list = dlink_list::DLINK_LIST::DoubleLinkedList::<i32>::new(0);
    // 从键盘输入直到输入退出
    loop {
        let mut std_in = String::new();
        io::stdin().read_line(&mut std_in).expect("read from stdin error");
        input::proc_in::handle_in(&mut d_list, &mut std_in);
    }
}
