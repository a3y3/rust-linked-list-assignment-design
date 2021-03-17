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

    //push_front


    // push_back
    fn test_push_back() {
        let mut linked_list = LinkedList::new(Holder { value: values[0] });
        let mut correct_size = 1;
        assert_eq!(correct_size, linked_list.size(), "Size mismatch!");
        let last_val = linked_list.peek_back();
        assert_eq!(values[0], last_val.value, "Incorrect value peeked");

        for value in &values {
            let holder = Holder { value: *value };
            linked_list.push_back(holder);
            correct_size += 1;
            assert_eq!(correct_size, linked_list.size(), "Size mismatch!");
            let last_val = linked_list.peek_back();
            assert_eq!(value, last_val.value, "Incorrect value peeked");
        }
    }

    // find
    //pop_front
    // pop_back
}
