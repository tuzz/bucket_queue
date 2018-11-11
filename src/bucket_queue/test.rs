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
    }
}
