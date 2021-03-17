#[cfg(test)]
mod tests {
    use super::super::LinkedList;
    struct Holder<T> {
        value: T,
    }
    const values: [&str; 8] = [
        "apple",
        "banana",
        "grape",
        "jackfruit",
        "pineapple",
        "cantaloupe",
        "",
        "melon",
    ];

    enum Error {
        SizeMismatch,
        IncorrectValueAtEnd,
        IncorrectValueAtStart,
        ValueOrderMismatch,
    }
    fn human_readable_err(err: Error) -> &'static str {
        match err {
            Error::SizeMismatch => "Incorrect size of list",
            Error::IncorrectValueAtEnd => "Incorrect value peeked from end of list",
            Error::IncorrectValueAtStart => "Incorrect value peeked from start of list",
            Error::ValueOrderMismatch => 
            "Incorrect value found while popping from start. Perhaps the order in which the list is created is incorrect?"
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
            format!("{}", human_readable_err(Error::SizeMismatch))
        );
        let first_val = linked_list.peek_front();
        assert_eq!(
            None, first_val,
            format!("{} ", human_readable_err(Error::IncorrectValueAtStart))
        );

        for value in &values {
            let holder = Holder { value: *value };
            linked_list.push_front(holder);
            correct_size += 1;
            assert_eq!(
                correct_size,
                linked_list.size(),
                format!("{}", human_readable_err(Error::SizeMismatch))
            );
            let first_val = linked_list.peek_front();
            assert_eq!(
                *value, first_val.unwrap().value,
                format!("{} ", human_readable_err(Error::IncorrectValueAtStart))
            );
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
            format!("{}", human_readable_err(Error::SizeMismatch))
        );
        let last_val = linked_list.peek_back();
        assert_eq!(
            None, last_val,
            format!("{} ", human_readable_err(Error::IncorrectValueAtEnd))
        );

        for value in &values {
            let holder = Holder { value: *value };
            linked_list.push_back(holder);
            correct_size += 1;
            assert_eq!(
                correct_size,
                linked_list.size(),
                format!("{}", human_readable_err(Error::SizeMismatch))
            );
            let last_val = linked_list.peek_back();
            assert_eq!(
                *value, last_val.unwrap().value,
                format!("{} ", human_readable_err(Error::IncorrectValueAtEnd))
            );
        }
    }

    //pop_front
    fn test_pop_front() {
        let mut linked_list = LinkedList::new();
        let mut correct_size = 0;
        let first_val = linked_list.pop_front();
        assert_eq!(
            correct_size,
            linked_list.size(),
            format!("{}", human_readable_err(Error::SizeMismatch))
        );
        assert_eq!(
            None, first_val,
            format!("{} ", human_readable_err(Error::IncorrectValueAtStart))
        );

        for value in &values {
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
                format!("{}", human_readable_err(Error::SizeMismatch))
            );
            assert_eq!(
                values[correct_size], first_val.unwrap().value,
                format!("{} ", human_readable_err(Error::ValueOrderMismatch))
            );
        }
    }

    // pop_back
    fn test_pop_back() {
        let mut linked_list = LinkedList::new();
        let mut correct_size = 0;
        let first_val = linked_list.pop_back();
        assert_eq!(
            correct_size,
            linked_list.size(),
            format!("{}", human_readable_err(Error::SizeMismatch))
        );
        assert_eq!(
            None, first_val,
            format!("{} ", human_readable_err(Error::IncorrectValueAtStart))
        );

        for value in &values {
            let holder = Holder { value: *value };
            linked_list.push_front(holder);
            correct_size += 1;
        }
        let mut i = 0;
        while linked_list.size() != 0 {
            let first_val = linked_list.pop_front();
            correct_size -= 1;
            assert_eq!(
                correct_size,
                linked_list.size(),
                format!("{}", human_readable_err(Error::SizeMismatch))
            );
            assert_eq!(
                values[i], first_val.unwrap().value,
                format!("{} ", human_readable_err(Error::ValueOrderMismatch))
            );
            i += 1;
        }
    }
    // find
    
}
