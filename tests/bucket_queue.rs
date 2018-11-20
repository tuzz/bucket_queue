extern crate bucket_queue;

use bucket_queue::*;
use std::collections::VecDeque;

type Subject<B> = BucketQueue<B>;

mod double_ended {
    use super::*;

    #[test]
    fn it_can_push_and_pop_front_and_back_with_priority() {
        let mut subject = Subject::<VecDeque<&'static str>>::new();

        subject.push_back("first", 0);
        subject.push_back("second", 1);
        subject.push_front("third", 0);
        subject.push_back("fourth", 1);
        subject.push_front("fifth", 1);

        // Current state of bucket queue:
        //   0: third, first
        //   1: fifth, second, fourth

        assert_eq!(subject.pop_back(0), Some("first"));
        assert_eq!(subject.pop_front(0), Some("third"));
        assert_eq!(subject.pop_back(0), None);

        assert_eq!(subject.pop_front(1), Some("fifth"));
        assert_eq!(subject.pop_back(1), Some("fourth"));
        assert_eq!(subject.pop_back(1), Some("second"));
        assert_eq!(subject.pop_front(1), None);

        assert_eq!(subject.pop_front(2), None);
        assert_eq!(subject.pop_back(3), None);
    }

    #[test]
    fn it_can_pop_front_and_back_with_minimum_priority() {
        let mut subject = Subject::<VecDeque<&'static str>>::new();

        subject.push_back("first", 0);
        subject.push_back("second", 1);
        subject.push_front("third", 0);
        subject.push_back("fourth", 1);
        subject.push_front("fifth", 1);

        // Current state of bucket queue:
        //   0: third, first
        //   1: fifth, second, fourth

        assert_eq!(subject.pop_front_min(), Some("third"));
        assert_eq!(subject.pop_back_min(), Some("first"));
        assert_eq!(subject.pop_front_min(), Some("fifth"));
        assert_eq!(subject.pop_back_min(), Some("fourth"));
        assert_eq!(subject.pop_back_min(), Some("second"));
        assert_eq!(subject.pop_front_min(), None);
    }

    #[test]
    fn it_can_pop_front_and_back_with_maximum_priority() {
        let mut subject = Subject::<VecDeque<&'static str>>::new();

        subject.push_back("first", 0);
        subject.push_back("second", 1);
        subject.push_front("third", 0);
        subject.push_back("fourth", 1);
        subject.push_front("fifth", 1);

        // Current state of bucket queue:
        //   0: third, first
        //   1: fifth, second, fourth

        assert_eq!(subject.pop_front_max(), Some("fifth"));
        assert_eq!(subject.pop_back_max(), Some("fourth"));
        assert_eq!(subject.pop_front_max(), Some("second"));
        assert_eq!(subject.pop_back_max(), Some("first"));
        assert_eq!(subject.pop_back_max(), Some("third"));
        assert_eq!(subject.pop_front_max(), None);
    }

    #[test]
    fn it_can_set_the_priority_then_push_and_pop_front_and_back() {
        let mut subject = Subject::<VecDeque<&'static str>>::new();

        subject.bucket(0).push_back("first");
        subject.bucket(1).push_back("second");
        subject.bucket(0).push_front("third");
        subject.bucket(1).push_back("fourth");
        subject.bucket(1).push_front("fifth");

        // Current state of bucket queue:
        //   0: third, first
        //   1: fifth, second, fourth

        assert_eq!(subject.bucket(0).pop_back(), Some("first"));
        assert_eq!(subject.bucket(0).pop_front(), Some("third"));
        assert_eq!(subject.bucket(0).pop_back(), None);

        assert_eq!(subject.bucket(1).pop_front(), Some("fifth"));
        assert_eq!(subject.bucket(1).pop_back(), Some("fourth"));
        assert_eq!(subject.bucket(1).pop_back(), Some("second"));
        assert_eq!(subject.bucket(1).pop_front(), None);

        assert_eq!(subject.bucket(2).pop_front(), None);
        assert_eq!(subject.bucket(3).pop_back(), None);
    }
}

mod first_in_first_out {
    use super::*;

    #[test]
    fn it_can_enqueue_and_dequeue_with_priority() {
        let mut subject = Subject::<VecDeque<&'static str>>::new();

        subject.enqueue("first", 0);
        subject.enqueue("second", 1);
        subject.enqueue("third", 0);

        assert_eq!(subject.dequeue(1), Some("second"));
        assert_eq!(subject.dequeue(1), None);

        assert_eq!(subject.dequeue(0), Some("first"));
        assert_eq!(subject.dequeue(0), Some("third"));
        assert_eq!(subject.dequeue(0), None);

        assert_eq!(subject.dequeue(2), None);
        assert_eq!(subject.dequeue(3), None);
    }

    #[test]
    fn it_can_dequeue_with_minimum_priority() {
        let mut subject = Subject::<VecDeque<&'static str>>::new();

        subject.enqueue("first", 0);
        subject.enqueue("second", 1);
        subject.enqueue("third", 0);

        assert_eq!(subject.dequeue_min(), Some("first"));
        assert_eq!(subject.dequeue_min(), Some("third"));
        assert_eq!(subject.dequeue_min(), Some("second"));
        assert_eq!(subject.dequeue_min(), None);
    }

    #[test]
    fn it_can_dequeue_with_maximum_priority() {
        let mut subject = Subject::<VecDeque<&'static str>>::new();

        subject.enqueue("first", 0);
        subject.enqueue("second", 1);
        subject.enqueue("third", 0);

        assert_eq!(subject.dequeue_max(), Some("second"));
        assert_eq!(subject.dequeue_max(), Some("first"));
        assert_eq!(subject.dequeue_max(), Some("third"));
        assert_eq!(subject.dequeue_max(), None);
    }

    #[test]
    fn it_can_set_the_priority_then_enqueue_and_dequeue() {
        let mut subject = Subject::<VecDeque<&'static str>>::new();

        subject.bucket(0).enqueue("first");
        subject.bucket(1).enqueue("second");
        subject.bucket(0).enqueue("third");

        assert_eq!(subject.bucket(1).dequeue(), Some("second"));
        assert_eq!(subject.bucket(1).dequeue(), None);

        assert_eq!(subject.bucket(0).dequeue(), Some("first"));
        assert_eq!(subject.bucket(0).dequeue(), Some("third"));
        assert_eq!(subject.bucket(0).dequeue(), None);

        assert_eq!(subject.bucket(2).dequeue(), None);
        assert_eq!(subject.bucket(3).dequeue(), None);
    }
}

mod last_in_first_out {
    use super::*;

    #[test]
    fn it_can_push_and_pop_with_priority() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        subject.push("first", 0);
        subject.push("second", 1);
        subject.push("third", 0);

        assert_eq!(subject.pop(1), Some("second"));
        assert_eq!(subject.pop(1), None);

        assert_eq!(subject.pop(0), Some("third"));
        assert_eq!(subject.pop(0), Some("first"));
        assert_eq!(subject.pop(0), None);

        assert_eq!(subject.pop(2), None);
        assert_eq!(subject.pop(3), None);
    }

    #[test]
    fn it_can_pop_with_minimum_priority() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        subject.push("first", 0);
        subject.push("second", 1);
        subject.push("third", 0);

        assert_eq!(subject.pop_min(), Some("third"));
        assert_eq!(subject.pop_min(), Some("first"));
        assert_eq!(subject.pop_min(), Some("second"));
        assert_eq!(subject.pop_min(), None);
    }

    #[test]
    fn it_can_pop_with_maximum_priority() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        subject.push("first", 0);
        subject.push("second", 1);
        subject.push("third", 0);

        assert_eq!(subject.pop_max(), Some("second"));
        assert_eq!(subject.pop_max(), Some("third"));
        assert_eq!(subject.pop_max(), Some("first"));
        assert_eq!(subject.pop_max(), None);
    }

    #[test]
    fn it_can_set_the_priority_then_push_and_pop() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        subject.bucket(0).push("first");
        subject.bucket(1).push("second");
        subject.bucket(0).push("third");

        assert_eq!(subject.bucket(1).pop(), Some("second"));
        assert_eq!(subject.bucket(1).pop(), None);

        assert_eq!(subject.bucket(0).pop(), Some("third"));
        assert_eq!(subject.bucket(0).pop(), Some("first"));
        assert_eq!(subject.bucket(0).pop(), None);

        assert_eq!(subject.bucket(2).pop(), None);
        assert_eq!(subject.bucket(3).pop(), None);
    }
}

mod min_and_max_priority {
    use super::*;

    #[test]
    fn it_can_return_the_min_and_max_priority() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        subject.push("first", 3);
        subject.push("second", 5);
        subject.push("third", 5);
        subject.push("fourth", 6);
        subject.push("fifth", 7);
        subject.push("sixth", 7);

        // first(3), second(5), third(5), fourth(6), fifth(7), sixth(7)

        assert_eq!(subject.min_priority(), Some(3));
        assert_eq!(subject.max_priority(), Some(7));

        subject.pop_max();

        // first(3), second(5), third(5), fourth(6), fifth(7)

        assert_eq!(subject.min_priority(), Some(3));
        assert_eq!(subject.max_priority(), Some(7));

        subject.pop(5);

        // first(3), second(5), fourth(6), fifth(7)

        assert_eq!(subject.min_priority(), Some(3));
        assert_eq!(subject.max_priority(), Some(7));

        subject.pop_min();

        // second(5), fourth(6), fifth(7)

        assert_eq!(subject.min_priority(), Some(5));
        assert_eq!(subject.max_priority(), Some(7));

        subject.pop_max();

        // second(5), fourth(6)

        assert_eq!(subject.min_priority(), Some(5));
        assert_eq!(subject.max_priority(), Some(6));

        subject.pop_max();

        // second(5)

        assert_eq!(subject.min_priority(), Some(5));
        assert_eq!(subject.max_priority(), Some(5));

        subject.pop_min();

        // <empty>

        assert_eq!(subject.min_priority(), None);
        assert_eq!(subject.max_priority(), None);
    }
}

mod len {
    use super::*;

    #[test]
    fn it_returns_the_number_of_enqueued_items() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        assert_eq!(subject.len(), 0);

        subject.push("first", 0);
        assert_eq!(subject.len(), 1);

        subject.push("second", 1);
        assert_eq!(subject.len(), 2);

        subject.pop_min();
        assert_eq!(subject.len(), 1);

        subject.pop_min();
        assert_eq!(subject.len(), 0);

        subject.pop_min();
        assert_eq!(subject.len(), 0);
    }
}

mod is_empty {
    use super::*;

    #[test]
    fn it_returns_true_if_no_items_are_enqueued() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        assert_eq!(subject.is_empty(), true);

        subject.push("first", 0);
        assert_eq!(subject.is_empty(), false);

        subject.push("second", 1);
        assert_eq!(subject.is_empty(), false);

        subject.pop_min();
        assert_eq!(subject.is_empty(), false);

        subject.pop_min();
        assert_eq!(subject.is_empty(), true);

        subject.pop_min();
        assert_eq!(subject.is_empty(), true);
    }
}

mod clear {
    use super::*;

    #[test]
    fn it_removes_all_items_from_the_bucket_queue() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        subject.push("first", 0);
        subject.push("second", 1);
        subject.push("third", 0);

        subject.clear();

        assert_eq!(subject.len(), 0);
        assert_eq!(subject.is_empty(), true);
        assert_eq!(subject.pop_min(), None);
    }

    #[test]
    fn it_can_clear_deferred_buckets() { // This is equivalent to replacing.
        let mut subject = Subject::<Vec<&'static str>>::new();

        subject.push("first", 0);
        subject.push("second", 1);
        subject.push("third", 0);

        subject.bucket(0).clear();

        assert_eq!(subject.len(), 1);
        assert_eq!(subject.is_empty(), false);

        assert_eq!(subject.pop_min(), Some("second"));
        assert_eq!(subject.pop_min(), None);
    }
}

mod replace {
    use super::*;

    #[test]
    fn it_replaces_a_bucket_in_the_bucket_queue() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        subject.push("first", 0);
        subject.push("second", 1);
        subject.push("third", 0);

        let bucket = vec!["fourth"];
        subject.replace(0, Some(bucket));

        assert_eq!(subject.len(), 2);
        assert_eq!(subject.is_empty(), false);

        assert_eq!(subject.pop_min(), Some("fourth"));
        assert_eq!(subject.pop_min(), Some("second"));
        assert_eq!(subject.pop_min(), None);
    }

    #[test]
    fn it_can_remove_a_bucket_by_replacing_it_with_none() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        subject.push("first", 0);
        subject.push("second", 1);
        subject.push("third", 0);

        subject.replace(0, None);

        assert_eq!(subject.len(), 1);
        assert_eq!(subject.is_empty(), false);

        assert_eq!(subject.pop_min(), Some("second"));
        assert_eq!(subject.pop_min(), None);
    }

    #[test]
    fn it_returns_the_bucket_of_replaced_items() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        subject.push("first", 0);
        subject.push("second", 1);
        subject.push("third", 0);

        let bucket = subject.replace(0, None);
        assert_eq!(bucket.unwrap(), &["first", "third"]);

        let bucket = subject.replace(0, None);
        assert_eq!(bucket, None);
    }

    #[test]
    fn it_can_replace_a_bucket_that_doesnt_exist_yet() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        subject.push("first", 0);
        subject.push("second", 1);
        subject.push("third", 0);

        let replacement = vec!["fourth"];
        let replaced = subject.replace(3, Some(replacement));

        assert_eq!(replaced, None);

        assert_eq!(subject.len(), 4);
        assert_eq!(subject.is_empty(), false);

        assert_eq!(subject.pop_min(), Some("third"));
        assert_eq!(subject.pop_min(), Some("first"));
        assert_eq!(subject.pop_min(), Some("second"));
        assert_eq!(subject.pop_min(), Some("fourth"));
        assert_eq!(subject.pop_min(), None);
    }
}

mod deferrals {
    use super::*;

    #[test]
    fn it_adds_to_bucket_zero_if_there_are_no_buckets() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        subject.min_bucket().push("first");
        subject.max_bucket().push("second");

        assert_eq!(subject.min_priority(), Some(0));
        assert_eq!(subject.max_priority(), Some(0));
    }

    #[test]
    #[should_panic]
    fn it_prevents_adding_to_the_bucket_more_than_once() {
        let mut subject = Subject::<Vec<&'static str>>::new();
        let mut bucket = subject.bucket(0);

        bucket.push("first");
        bucket.push("second"); // The second push should panic.
    }

    #[test]
    #[should_panic]
    fn it_prevents_removing_from_the_bucket_more_than_once() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        subject.push("first", 0);
        subject.push("second", 0);

        let mut bucket = subject.bucket(0);

        bucket.pop();
        bucket.pop(); // The second pop should panic.
    }

    #[test]
    fn it_can_ask_questions_of_buckets_more_than_once() {
        let mut subject = Subject::<Vec<&'static str>>::new();

        subject.push("first", 0);

        let bucket = subject.bucket(0);

        assert_eq!(bucket.len_bucket(), 1);
        assert_eq!(bucket.is_empty_bucket(), false);
        assert_eq!(bucket.len_bucket(), 1);
    }
}

mod nested_bucket_queue {
    use super::*;

    #[test]
    fn it_supports_using_a_bucket_queue_as_a_type_of_bucket() {
        let mut subject = Subject::<Subject<Vec<&'static str>>>::new();

        subject.bucket_for_adding(0).push("first", 0);
        subject.bucket_for_adding(0).push("second", 1);
        subject.bucket_for_adding(1).push("third", 0);

        assert_eq!(subject.len(), 3);

        let first = subject.bucket_for_removing(0).unwrap().pop_min();
        assert_eq!(first, Some("first"));
        assert_eq!(subject.len(), 2);

        let max = subject.max_priority().unwrap();
        let bucket = subject.bucket_for_removing(max).unwrap();
        assert_eq!(bucket.pop_max(), Some("third"));
    }

    #[test]
    fn it_supports_accessing_nested_buckets_via_deferrals() {
        let mut subject = Subject::<Subject<Vec<&'static str>>>::new();

        subject.bucket(0).push("first", 0);
        subject.bucket(0).push("second", 1);
        subject.bucket(1).bucket(0).push("third");

        assert_eq!(subject.len(), 3);

        assert_eq!(subject.bucket(0).pop_min(), Some("first"));
        assert_eq!(subject.len(), 2);

        assert_eq!(subject.max_bucket().pop_max(), Some("third"));
        assert_eq!(subject.len(), 1);

        assert_eq!(subject.min_bucket().pop_min(), Some("second"));
        assert_eq!(subject.len(), 0);
    }

    #[test]
    fn it_supports_replacing_nested_buckets_via_deferrals() {
        let mut subject = Subject::<Subject<Vec<&'static str>>>::new();

        subject.bucket(0).push("first", 0);
        subject.bucket(0).push("second", 1);
        subject.bucket(0).push("third", 1);
        subject.bucket(1).push("fourth", 0);

        let bucket = subject.bucket(0).replace(1, None);

        assert_eq!(subject.len(), 2);
        assert_eq!(bucket.unwrap(), &["second", "third"]);

        let bucket = subject.bucket(0).replace(1, None);
        assert_eq!(bucket, None);

        let replacement = vec!["fifth"];
        subject.bucket(2).replace(2, Some(replacement));
        assert_eq!(bucket, None);

        assert_eq!(subject.min_bucket().pop_min(), Some("first"));
        assert_eq!(subject.min_bucket().pop_min(), Some("fourth"));
        assert_eq!(subject.min_bucket().pop_min(), Some("fifth"));
        assert_eq!(subject.min_bucket().pop_min(), None);
    }

    #[test]
    fn it_can_be_arbitrarily_nested() {
        let mut subject = Subject::<Subject<Subject<Vec<&'static str>>>>::new();

        subject.bucket(0).bucket(1).push("first", 2);
        subject.bucket(3).bucket(4).bucket(5).push("second");
        subject.bucket(3).bucket(6).bucket(7).push("third");

        assert_eq!(subject.len(), 3);

        assert_eq!(subject.max_priority(), Some(3));
        assert_eq!(subject.min_bucket().min_priority(), Some(1));
        assert_eq!(subject.max_bucket().min_bucket().max_priority(), Some(5));

        assert_eq!(subject.max_bucket().min_bucket().pop_min(), Some("second"));
        assert_eq!(subject.max_bucket().min_bucket().pop_min(), Some("third"));
        assert_eq!(subject.max_bucket().min_bucket().pop_min(), Some("first"));
    }

    #[test]
    fn it_can_replace_arbitrarily_nested_buckets_via_deferrals() {
        let mut subject = Subject::<Subject<Subject<Vec<&'static str>>>>::new();

        subject.bucket(0).bucket(1).bucket(0).push("first");
        subject.bucket(0).bucket(1).bucket(0).push("second");
        subject.bucket(0).bucket(1).bucket(1).push("third");
        subject.bucket(0).bucket(2).bucket(0).push("fourth");
        subject.bucket(0).bucket(2).bucket(0).push("fifth");
        subject.bucket(1).bucket(0).bucket(0).push("sixth");

        let bucket = subject.bucket(0).bucket(1).replace(0, None);
        assert_eq!(bucket.unwrap(), &["first", "second"]);
        assert_eq!(subject.len(), 4);
        assert_eq!(subject.bucket(0).len(), 3);

        let bucket = subject.bucket(0).bucket(1).replace(0, None);
        assert_eq!(bucket, None);

        let mut replacement = Subject::<Vec<&'static str>>::new();
        replacement.bucket(1).push("seventh");
        replacement.bucket(2).push("eighth");

        let bucket = subject.bucket(1).replace(0, Some(replacement)); // TODO switch args
        assert_eq!(bucket.unwrap().pop_min(), Some("sixth"));

        assert_eq!(subject.min_bucket().min_bucket().pop_min(), Some("third"));
        assert_eq!(subject.min_bucket().min_bucket().pop_min(), Some("fifth"));
        assert_eq!(subject.min_bucket().min_bucket().pop_min(), Some("fourth"));
        assert_eq!(subject.min_bucket().min_bucket().pop_min(), Some("seventh"));
        assert_eq!(subject.min_bucket().min_bucket().pop_min(), Some("eighth"));
        assert_eq!(subject.min_bucket().min_bucket().pop_min(), None);
    }
}
