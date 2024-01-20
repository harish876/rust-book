#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> 
where T: Copy
{
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> 
where T: Copy
{
    fn new() -> Self {
        Self {
            head: None
        }
    }

    fn insert(&mut self, value: T) {
        let old_head = self.head.take();

        match old_head {
            Some(old_head) => {
                let new_head = Node { value, next: Some(old_head) };
                self.head = Some(Box::new(new_head));

            }
            None => {
                self.head = Some(
                    Box::new(Node { value, next: None })
                )
            }
        }
    }

    fn pop(&mut self) -> Option<T> {

        let old_head = self.head.take();
        match old_head {
            Some(old_head) => {
                let val = old_head.value;
                self.head = old_head.next;
                Some(val)
            }
            None => {
                None
            }   
        }

    }

    fn peek(&mut self) -> Option<T> {

        self.head.as_ref().map(|head|{
            head.value
        })
    }

}


#[cfg(test)]

mod test {

    use super::*;

    #[test]
    fn it_linked_list_test(){

        println!("Print test..");
        let mut list = LinkedList::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);
        list.insert(4);


        println!("{:?}", list);

        list.pop();

        println!("{:?}", list);


    }   

}