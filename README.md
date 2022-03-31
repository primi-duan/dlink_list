# dlink_list
rust实现的安全双链表，包含从终端解析输入及链表的基本操作
## 使用方法
```
cargo build  
cargo run  

create [cap]           create double-link-list with given capacity.  
append_left [ele]      insert one element at head of list.  
pop_left               pop one element from head.  
peek_left              show head element value.  
append [ele]           insert one element at tail of list.  
pop                    pop one element from tail.  
peek_end               show tail element value.  
clear                  remove all element.  
show                   display all element value.  
exit                   close program.  
```
## 例子
```
Hello world by roc-duan!

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
        
show
------------show list------------
length: 0
capacity: 0
[empty]
------------end show------------

create 3
cap_num is 3
show
------------show list------------
length: 0
capacity: 3
[empty]
------------end show------------

append_left 1
insert value is 1
append 2
insert value is 2
append 10
insert value is 10
append 11
insert value is 11
There is no capacity, 3:3
show
------------show list------------
length: 3
capacity: 3
node value 1 -> node value 2 -> node value 10
------------end show------------

pop_left
get node from head, value is 1
show
------------show list------------
length: 2
capacity: 3
node value 2 -> node value 10
------------end show------------

append_left 5
insert value is 5
show
------------show list------------
length: 3
capacity: 3
node value 5 -> node value 2 -> node value 10
------------end show------------

pop
get node from tail, value is 10
show
------------show list------------
length: 2
capacity: 3
node value 5 -> node value 2
------------end show------------
exit
```
