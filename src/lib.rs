mod service {
    use rand::Rng;
    use std::fmt::Display;

    pub trait StuffDoer {
        fn do_stuff<T: Display>(&self, value: T) -> String {
            // imagine this function resulting in side-effects.
            let mut rng = rand::thread_rng();
            let random_stuff: u8 = rng.gen();
            format!("{} {}", value, random_stuff)
        }

        fn do_other_stuff<T: Display>(&self, value: T) -> String {
            format!("alt-{}", value)
        }
    }
}

pub mod caller {
    use super::service;
    use std::sync::Arc;

    pub fn call_do_stuff<T: service::StuffDoer>(doer: Arc<T>, value: &str) -> String {
        let my_stuff = doer.do_stuff(value);
        format!("{} [from caller]", my_stuff)
    }

    pub fn call_do_other_stuff<T: service::StuffDoer>(doer: Arc<T>, value: &str) -> String {
        format!("{} [from caller]", doer.do_other_stuff(value))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;
    use std::sync::Arc;

    struct Mock {}
    impl super::service::StuffDoer for Mock {
        fn do_stuff<T: Display>(&self, value: T) -> String {
            format!("{} from mock", value)
        }
    }

    #[test]
    fn it_works() {
        let doer = Arc::new(Mock {});
        assert_eq!(
            "alt-my stuff [from caller]".to_string(),
            super::caller::call_do_other_stuff(doer.clone(), "my stuff")
        );
        assert_eq!(
            "my stuff from mock [from caller]".to_string(),
            super::caller::call_do_stuff(doer, "my stuff")
        );
    }
}
