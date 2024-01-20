//limit checker based on some value and threshold

pub trait Messenger {
    fn send(&self, message: &str);
}

struct LimitTracker<'a,T: Messenger> {
    messenger: &'a T,
    current_value: usize,
    limit: usize,
}

impl<'a,T> LimitTracker<'a,T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, current_value: usize, limit: usize) -> Self {
        Self {
            messenger,
            current_value,
            limit,
        }
    }

    fn check_limit(&self) {
        let threshold = self.current_value as f64 / self.limit as f64;

        if threshold >= 1.0 {
            self.messenger.send(&format!(
                "Limit Exceeded for the current user with threshold {}",
                threshold
            ));
        } else if threshold >= 0.90 {
            self.messenger.send(&format!("Limit has exceeded 90%"));
        } else if threshold >= 0.75 {
            self.messenger.send(&format!("Warning...Limit has exceeded 75%"));
        }
    }
}

#[cfg(test)]

mod test {
    use super::*;
    use std::cell::RefCell; 

    struct MockMessenger {
        messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            Self { messages: RefCell::new(vec![]) }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.messages.borrow_mut().push(message.to_string());
        }
    }

    #[test]
    fn test_one() {
        let mock_messenger = MockMessenger::new();
        let limit_tracker = LimitTracker::new(&mock_messenger, 80, 100);
        limit_tracker.check_limit();

        assert_eq!(mock_messenger.messages.borrow().len(),1);
    }
}
