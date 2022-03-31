pub mod proc_in {
    use std::process::exit;
    use crate::dlink_list;

    pub fn help() -> String {
        format!("
create [cap]           create double-link-list with given capacity
append_left [ele]      insert one element at head of list
pop_left               pop one element from head
peek_left              show head element value
append [ele]           insert one element at tail of list
pop                    pop one element from tail
peek_end               show tail element value
clear                  remove all element
show                   display all element value
exit                   close program
        ")
    }

    pub fn handle_in(d_list: &mut dlink_list::DLINK_LIST::DoubleLinkedList::<i32>, std_in: &mut String) {
        let inputs: Vec<&str> = std_in.split(" ").collect();
        //println!("{:?}", inputs);
        match inputs[0] {
            "create" => {
                match inputs.get(1) {
                    Some(cap) => {
                        let cap_num: i32 = cap.trim().parse().expect("not a number");
                        println!("cap_num is {}", cap_num);
                        *d_list = dlink_list::DLINK_LIST::DoubleLinkedList::<i32>::new(cap_num);
                    },
                    None => {
                        println!("Incomplete input\r\n{}", help());
                    }
                }
            },
            "append_left" => {
                match inputs.get(1) {
                    Some(ele) => {
                        let value: i32 = ele.trim().parse().expect("not a number");
                        println!("insert value is {}", value);
                        d_list.insert_at_head(value);
                    },
                    None => {
                        println!("Incomplete input\r\n{}", help());
                    }
                }
            },
            "pop_left\n" => {
                match d_list.pop_from_head() {
                    Some(v) => {
                        println!("get node from head, value is {}", v);
                    },
                    None => {
                        println!("list is NULL");
                    }
                }
                //println!("get node from head, value is {:?}", d_list.pop_from_head());
            },
            "peek_left\n" => {
                match d_list.peek_at_head() {
                    Some(v) => {
                        println!("list head_node value is {}", v);
                    },
                    None => {
                        println!("list is NULL");
                    }
                }
            },
            "append" => {
                match inputs.get(1) {
                    Some(ele) => {
                        let value :i32 = ele.trim().parse().expect("not a number");
                        println!("insert value is {}", value);
                        d_list.insert_at_tail(value);
                    },
                    None => {
                        println!("Incomplete input\r\n{}", help());
                    }
                }
            },
            "pop\n" => {
                match d_list.pop_from_tail() {
                    Some(v) => {
                        println!("get node from tail, value is {}", v);
                    },
                    None => {
                        println!("list is NULL");
                    }
                }
            },
            "peek_end\n" => {
                match d_list.peek_at_tail() {
                    Some(v) => {
                        println!("list tail_node value is {}", v);
                    },
                    None => {
                        println!("list is NULL");
                    }
                }
            },
            "show\n" => {
                println!("{}", d_list);
            },
            "exit\n" => {
                println!("exit!!");
                exit(0);
            },
            _ => {
                println!("other input:{:?}\r\n{}", inputs, help());
            }
        }
    }
}