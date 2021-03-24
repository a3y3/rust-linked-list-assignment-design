#[cfg(test)]
mod tests {
    use super::super::LinkedList;
    #[derive(Debug, Clone, Eq, PartialEq)]
    struct Holder<T> {
        value: T,
    }
    const VALUES: [&str; 8] = [
        "apple",
        "grape",
        "jackfruit",
        "pineapple",
        "banana",
        "cantaloupe",
        "",
        "melon",
    ];

    enum Error {
        SizeMismatch,
        IncorrectValueAtEnd,
        IncorrectValueAtStart,
        ValueOrderMismatch,
        IncorrectFindOutput,
        IncorrectRemoveOutput,
        FindSizeMismatch
    }

    fn human_readable_err(err: Error) -> &'static str {
        match err {
            Error::SizeMismatch => "Incorrect size of list",
            Error::IncorrectValueAtEnd => "Incorrect value peeked from end of list",
            Error::IncorrectValueAtStart => "Incorrect value peeked from start of list",
            Error::ValueOrderMismatch => 
            "Incorrect value found while popping from start. Perhaps the order in which the list is created is incorrect?",
            Error::IncorrectFindOutput => "Incorrect value found while using find()",
            Error::IncorrectRemoveOutput => "Incorrect value returned from remove()",
            Error::FindSizeMismatch => "Find modifies length of list"
        }
    }


   //test_size
   #[test]
   fn test_size() {
       let mut linked_list = LinkedList::new();
       let mut correct_size = 0;
       
       for value in &VALUES {
           let holder = Holder { value: *value };
           linked_list.push_front(holder);
           correct_size += 1;
           assert_eq!(
               correct_size,
               linked_list.size(),
               "{}", human_readable_err(Error::SizeMismatch));
       }
   }
    //push_front
    #[test]
    fn test_push_front() {
        let mut linked_list = LinkedList::new();
        let mut correct_size = 0;
        assert_eq!(
            correct_size,
            linked_list.size(),
            "{}", human_readable_err(Error::SizeMismatch));
        assert!(linked_list.peek_front().is_none(),"{}", human_readable_err(Error::IncorrectValueAtStart) );
        
        for value in &VALUES {
            let holder = Holder { value: *value };
            linked_list.push_front(holder);
            correct_size += 1;
            assert_eq!(
                correct_size,
                linked_list.size(),
                "{}", human_readable_err(Error::SizeMismatch));
            let first_val = linked_list.peek_front();
            assert_eq!(
                *value, first_val.unwrap().value,
                "{} ", human_readable_err(Error::IncorrectValueAtStart));
        }
    }
     //peek_back
     #[test]
     fn test_peek_back() {
         let mut linked_list = LinkedList::new();
         let mut correct_size = 0;
         assert_eq!(
             correct_size,
             linked_list.size(),
             "{}", human_readable_err(Error::SizeMismatch));
         assert!(linked_list.peek_back().is_none(),"{}", human_readable_err(Error::IncorrectValueAtStart) );
         
         for value in &VALUES {
             let holder = Holder { value: *value };
             linked_list.push_front(holder);
             correct_size += 1;
             assert_eq!(
                 correct_size,
                 linked_list.size(),
                 "{}", human_readable_err(Error::SizeMismatch));
         }
         let array_length= correct_size;
         while linked_list.size() != 0 {
             let last_val = linked_list.peek_back();
             assert_eq!(
                 correct_size,
                 linked_list.size(),
                 "{}", human_readable_err(Error::SizeMismatch)
             );
             assert_eq!(
                 VALUES[array_length-correct_size], last_val.unwrap().value,
                 "{} ", human_readable_err(Error::ValueOrderMismatch)
             );
             linked_list.pop_back();
             correct_size -= 1;
         }
     }
    // push_back
    #[test]
    fn test_push_back() {
        let mut linked_list = LinkedList::new();
        let mut correct_size = 0;
        assert_eq!(
            correct_size,
            linked_list.size(),
            "{}", human_readable_err(Error::SizeMismatch));
        assert!(linked_list.peek_front().is_none(),"{}", human_readable_err(Error::IncorrectValueAtStart) );

        for value in &VALUES {
            let holder = Holder { value: *value };
            linked_list.push_back(holder);
            correct_size += 1;
            assert_eq!(
                correct_size,
                linked_list.size(),
                "{}", human_readable_err(Error::SizeMismatch));
            let last_val = linked_list.peek_back();
            assert_eq!(
                *value, last_val.unwrap().value,
                "{} ", human_readable_err(Error::IncorrectValueAtEnd));
        }
    }

    //pop_front
    #[test]
    fn test_pop_front() {
        let mut linked_list = LinkedList::new();
        let mut correct_size = 0;
        let first_val = linked_list.pop_front();
        assert_eq!(
            correct_size,
            linked_list.size(),
            "{}", human_readable_err(Error::SizeMismatch)
        );
        assert_eq!(
            None, first_val,
            "{} ", human_readable_err(Error::IncorrectValueAtStart)
        );

        for value in &VALUES {
            let holder = Holder { value: *value };
            linked_list.push_front(holder);
            correct_size += 1;
        }
        while linked_list.size() != 0 {
            let first_val = linked_list.pop_front();
            correct_size -= 1;
            assert_eq!(
                correct_size,
                linked_list.size(),
                "{}", human_readable_err(Error::SizeMismatch)
            );
            assert_eq!(
                VALUES[correct_size], first_val.unwrap().value,
                "{} ", human_readable_err(Error::ValueOrderMismatch)
            );
        }
    }
     //peek_front
     #[test]
     fn test_peek_front() {
         let mut linked_list = LinkedList::new();
         let mut correct_size = 0;
         assert_eq!(
             correct_size,
             linked_list.size(),
             "{}", human_readable_err(Error::SizeMismatch));
         assert!(linked_list.peek_front().is_none(),"{}", human_readable_err(Error::IncorrectValueAtStart) );
         
         for value in &VALUES {
             let holder = Holder { value: *value };
             linked_list.push_front(holder);
             correct_size += 1;
             assert_eq!(
                 correct_size,
                 linked_list.size(),
                 "{}", human_readable_err(Error::SizeMismatch));
         }
         while linked_list.size() != 0 {
             let first_val = linked_list.peek_front();
             assert_eq!(
                 correct_size,
                 linked_list.size(),
                 "{}", human_readable_err(Error::SizeMismatch)
             );
             assert_eq!(
                 VALUES[correct_size-1], first_val.unwrap().value,
                 "{} ", human_readable_err(Error::ValueOrderMismatch)
             );
             linked_list.pop_front();
             correct_size -= 1;
         }
     }

    // pop_back
    #[test]
    fn test_pop_back() {
        let mut linked_list = LinkedList::new();
        let mut correct_size = 0;
        let first_val = linked_list.pop_back();
        assert_eq!(
            correct_size,
            linked_list.size(),
            "{}", human_readable_err(Error::SizeMismatch)
        );
        assert_eq!(
            None, first_val,
            "{} ", human_readable_err(Error::IncorrectValueAtStart)
        );

        for value in &VALUES {
            let holder = Holder { value: *value };
            linked_list.push_front(holder);
            correct_size += 1;
        }
        while linked_list.size() != 0 {
            let first_val = linked_list.pop_front();
            correct_size -= 1;
            assert_eq!(
                correct_size,
                linked_list.size(),
                "{}", human_readable_err(Error::SizeMismatch)
            );
            assert_eq!(
                VALUES[correct_size], first_val.unwrap().value,
                "{} ", human_readable_err(Error::ValueOrderMismatch)
            );
        }
    }

    // find
    #[test]
    fn test_find(){
        let mut linked_list = LinkedList::new();
        let found_value = linked_list.find(&Holder{value: "Some value"});
        assert_eq!(false, found_value, "{}", human_readable_err(Error::IncorrectFindOutput));
        let mut correct_size = 0;

        for value in &VALUES {
            let holder = Holder { value: *value };
            linked_list.push_back(holder.clone());
            correct_size += 1;
            assert_eq!(correct_size, linked_list.size(), "{}", human_readable_err(Error::FindSizeMismatch));
            let found_value = linked_list.find(&holder);
            assert_eq!(true, found_value, "{}", human_readable_err(Error::IncorrectFindOutput));
        }
    }

    // remove
    #[test]
    fn test_remove(){
        let mut linked_list = LinkedList::new();
        let mut correct_size = 0;
        let removed = linked_list.remove(&Holder{value: "Some value"});
        assert_eq!(None, removed, "{}", human_readable_err(Error::IncorrectRemoveOutput));
        assert_eq!(correct_size, linked_list.size(), "{}", human_readable_err(Error::SizeMismatch));

        for value in &VALUES {
            let holder = Holder { value: *value };
            linked_list.push_front(holder);
            correct_size += 1;
        }

        for value in &VALUES {
            let holder = Holder { value: *value };
            let removed = linked_list.remove(&holder);
            correct_size -= 1;
            assert_eq!(removed.unwrap(), holder, "{}", human_readable_err(Error::IncorrectRemoveOutput));
            assert_eq!(correct_size, linked_list.size(), "{}", human_readable_err(Error::SizeMismatch));
        }
    }
}
