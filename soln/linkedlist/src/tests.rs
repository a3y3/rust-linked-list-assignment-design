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
    }
    fn human_readable_err(err: Error) -> &'static str {
        match err {
            Error::SizeMismatch => "Incorrect size of list",
            Error::IncorrectValueAtEnd => "Incorrect value peeked from end of list",
            Error::IncorrectValueAtStart => "Incorrect value peeked from start of list",
        }
    }

    //push_front
    #[test]
    fn test_push_front() {
        let mut linked_list = LinkedList::new(Holder { value: values[0] });
        let mut correct_size = 1;
        assert_eq!(
            correct_size,
            linked_list.size(),
            format!("{}", human_readable_err(Error::SizeMismatch))
        );
        let first_val = linked_list.peek_front();
        assert_eq!(
            values[0], first_val.value,
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
            let last_val = linked_list.peek_back();
            assert_eq!(
                *value, last_val.value,
                format!("{} ", human_readable_err(Error::IncorrectValueAtStart))
            );
        }
    }

    // push_back
    #[test]
    fn test_push_back() {
        let mut linked_list = LinkedList::new(Holder { value: values[0] });
        let mut correct_size = 1;
        assert_eq!(
            correct_size,
            linked_list.size(),
            format!("{}", human_readable_err(Error::SizeMismatch))
        );
        let last_val = linked_list.peek_back();
        assert_eq!(
            values[0], last_val.value,
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
                *value, last_val.value,
                format!("{} ", human_readable_err(Error::IncorrectValueAtEnd))
            );
        }
    }

    //pop_front
    // pop_back
    // find
}
