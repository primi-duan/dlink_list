pub mod DLINK_LIST {
    use std::cell::{RefCell, Ref};
    use std::rc::Rc;
    use std::fmt::Display;
    use std::fmt;

    /*
        前后继指针类型：Option<Rc<RefCell<Node_st<T>>>>
        首先prev,next是可有可无的，所以是Option类型
        由于prev和next是对其他节点的引用，所以没有对应节点的所有权，采用Rc共享所有权（Rc表示不可变的shared_ptr）
        因为双向链表的节点变动会牵涉prev和next字段的变动，但是Rc是不可变的（就算node设置为mut也不行），所以需要RefCell包裹一下变成可变的。一般Rc都和RefCell配合使用的
    */
    #[derive(Debug)]
    struct Node_st<T: Display> {
        element: T,
        prev: Option<Rc<RefCell<Node_st<T>>>>,
        next: Option<Rc<RefCell<Node_st<T>>>>,
    }

    impl <T: Display> Node_st<T> {
        fn make(value: T) -> Rc<RefCell<Node_st<T>>> { //外部不可调用，私有方法，返回一个节点的共享引用
            let one_node = Node_st {
                element: value,
                prev: None,
                next: None,
            };
            return Rc::new(RefCell::new(one_node));
        }
    }
    
    #[derive(Debug)]
    pub struct DoubleLinkedList<T:Display> {
        capacity: i32,
        len: i32,
        head: Option<Rc<RefCell<Node_st<T>>>>,
        tail: Option<Rc<RefCell<Node_st<T>>>>,
    }

    impl <T: Display> fmt::Display for DoubleLinkedList<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "------------show list------------\n");
            write!(f, "length: {}\n", self.len);
            write!(f, "capacity: {}\n", self.capacity);
            if self.len == 0 {
                write!(f, "[empty]");
                return write!(f, "\n------------end show------------\n");
            }

            // as_ref 是转引用函数, 在不改变被转换对象(具有所有权)的基础上产生一个引用对象
            let tmp_head = self.head.as_ref().unwrap().clone();
            write!(f, "node value {}", tmp_head.borrow().element);
            let mut next = tmp_head;
            for _ in 1 .. self.len {
                write!(f, " -> ");
                let next_t = next.borrow().next.as_ref().unwrap().clone();
                write!(f, "node value {}", next_t.borrow().element);
                next = next_t;
            }
            
            return write!(f, "\n------------end show------------\n");
        }
    }

    impl<T: Display> DoubleLinkedList<T> {
        pub fn new(capacity: i32) -> Self { // 静态方法，Self等价于DoubleLinkedList<T>
            if capacity < 0 {
                println!("input capacity error:{}", capacity);
                panic!();
            }
            let ret = DoubleLinkedList {
                capacity: capacity,
                head: None,
                tail: None,
                len: 0,
            };
            return ret; // or 去掉上行的逗号，直接当做返回
        }

        // append_left
        pub fn insert_at_head(&mut self, value: T) {
            if self.capacity == 0 {
                println!("List not init! please input 'create [cap]'");
                return;
            }
            if self.len >= self.capacity {
                println!("There is no capacity, {}:{}", self.capacity, self.len);
                return;
            }
            let new_node = Node_st::make(value); //得到节点的一个共享引用

            // 因为match会导致所有权转移,head属于self不能转移所有权，因此使用Option的take将内部值取出,self.head变成None
            let tmp_head = self.head.take();
            match tmp_head {
                Some(n_rc) => {
                    // 链表非空
                    // borror_mut是RefCell的方法，让内部的值变为可变
                    // Rc让一个值有多个共享所有者，调用clone产生一个指针指向该值
                    n_rc.borrow_mut().prev = Some(new_node.clone());
                    new_node.borrow_mut().next = Some(n_rc);
                    self.head = Some(new_node);
                },
                None => {
                    // 链表为空,直接挂
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
            }
            self.len += 1;
        }

        // pop_left
        pub fn pop_from_head(&mut self) -> Option<T> {
            if self.len == 0 {
                println!("There is no node in list!");
                return None;
            }
            // 采用Option::map的方式取出返回值,map中使用匿名函数
            let pop_value = self.head.take().map(|old_node_rc| {
                // 判断第一个节点有没有next节点
                // 如果有则断开并把head指向next，如果没有则把self.head和tail一起变成None
                match old_node_rc.borrow_mut().next.take() {
                    Some(next_rc) => {
                        // 解出old_node_rc其余引用
                        next_rc.borrow_mut().prev = None;
                        self.head = Some(next_rc);
                    },
                    None => {
                        // head在take时已经变为None
                        self.tail = None;
                    }
                }
                
                // node_rc是Rc类型，表示智能指针，共享了所有权.
                // 但是pop则表示把node从双向链表中删除，即所有权转移走
                // 我们又不知道有没有其他地方共享了所有权，所以使用Rc::try_unwrap，在运行时判断尝试解出内部的RefCell
                // ok()表示把Result类型转成Option类型，再使用Option的unwrap解出RefCell<Node>
                // into_inner是将RefCell<Node>解出Node，最终所有权被释放出来了
                Rc::try_unwrap(old_node_rc).ok().unwrap().into_inner().element
            });

            self.len -= 1;
            return pop_value;
        }

        // peak_left
        pub fn peek_at_head(&self) -> Option<Ref<T>> {
            if self.len == 0 {
                println!("There is no node in list!");
                return None;
            }
            // Ref:Wraps a borrowed reference to a value in a RefCell box
            // example:
            // let c = RefCell::new((5, 'b'));
            // let b1: Ref<(u32, char)> = c.borrow();
            // let b2: Ref<u32> = Ref::map(b1, |t| &t.0);
            // assert_eq!(*b2, 5)
            let head_ele = self.head.as_ref().map( |node_rc| {
                let b_node = node_rc.borrow();
                let ele = Ref::map(b_node, |n| {
                    &n.element
                });
                return ele
            });
            return head_ele;
        }

        // append
        pub fn insert_at_tail(&mut self, value: T) {
            if self.capacity == 0 {
                println!("List not init! please input 'create [cap]'");
                return;
            }
            if self.len >= self.capacity {
                println!("There is no capacity, {}:{}", self.capacity, self.len);
                return;
            }

            let new_node = Node_st::make(value);

            // 因为match会导致所有权转移,head属于self不能转移所有权，因此使用Option的take将内部值取出,self.tail变成None
            let tmp_tail = self.tail.take();
            match tmp_tail {
                Some(n_rc) => {
                    // 链表非空
                    n_rc.borrow_mut().next = Some(new_node.clone());
                    new_node.borrow_mut().prev = Some(n_rc);
                    self.tail = Some(new_node);
                },
                None => {
                    // 链表为空,直接挂
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
            }
            self.len += 1;
        }

        // pop
        pub fn pop_from_tail(&mut self) -> Option<T>{
            if self.len == 0 {
                println!("There is no node in list!");
                return None;
            }

            // 采用Option::map的方式取出返回值,map中使用匿名函数
            let pop_value = self.tail.take().map(|old_node_rc| {
                // 判断最后一个节点有没有pre节点
                // 如果有则断开并把tail指向pre，如果没有则把self.head和tail一起变成None
                match old_node_rc.borrow_mut().prev.take() {
                    Some(pre_rc) => {
                        // 解出old_node_rc其余引用
                        pre_rc.borrow_mut().next = None;
                        self.tail = Some(pre_rc);
                    },
                    None => {
                        // tail在take时已经变为None
                        self.head = None;
                    }
                }

                // old_node_rc是Rc类型，表示智能指针，共享了所有权.
                // 但是pop则表示把node从双向链表中删除，即所有权转移走
                // 我们又不知道有没有其他地方共享了所有权，所以使用Rc::try_unwrap，在运行时判断尝试解出内部的RefCell并释放所有权
                // ok()表示把Result类型转成Option类型，再使用Option的unwrap解出RefCell<Node>
                // into_inner是将RefCell<Node>解出Node，最终所有权被释放出来了
                Rc::try_unwrap(old_node_rc).ok().unwrap().into_inner().element
            });

            self.len -= 1;
            return pop_value;
        }

        // peak_end
        pub fn peek_at_tail(&self) -> Option<Ref<T>> {
            if self.len == 0 {
                println!("There is no node in list!");
                return None;
            }
            // Ref:Wraps a borrowed reference to a value in a RefCell box
            // example:
            // let c = RefCell::new((5, 'b'));
            // let b1: Ref<(u32, char)> = c.borrow();
            // let b2: Ref<u32> = Ref::map(b1, |t| &t.0);
            // assert_eq!(*b2, 5)
            let tail_ele = self.tail.as_ref().map( |node_rc| {
                let b_node = node_rc.borrow();
                let ele = Ref::map(b_node, |n| {
                    &n.element
                });
                return ele
            });
            return tail_ele;
        }
    }
}