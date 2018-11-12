use super::*;

type Subject<B> = BucketQueue<B>;

use std::collections::VecDeque;

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

mod nested_bucket_queue {
    use super::*;

    #[test]
    fn it_supports_using_a_bucket_queue_as_a_type_of_bucket() {
        let mut subject = Subject::<Subject<Vec<&'static str>>>::new();

        subject.bucket_for_adding(0).push("first", 0);
        subject.bucket_for_adding(0).push("second", 1);
        subject.bucket_for_adding(1).push("third", 0);

        assert_eq!(subject.len(), 3);

        let bucket = subject.bucket_for_removing(0).unwrap();
        assert_eq!(bucket.pop_min(), Some("first"));
        assert_eq!(subject.len(), 2);

        let max = subject.max_priority().unwrap();
        let bucket = subject.bucket_for_removing(max).unwrap();
        assert_eq!(bucket.pop_max(), Some("third"));
    }
}
