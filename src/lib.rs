mod service {
    use std::fmt::Display;

    pub trait StuffDoer {
        fn do_stuff<T: Display>(&self, value: T) -> String {
            format!("{}", value)
        }
    }
}

pub mod caller {
    use super::service;

    pub fn call_do_stuff<T: service::StuffDoer>(doer: T, value: &str) -> String {
        doer.do_stuff(value)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    struct Mock {}
    impl super::service::StuffDoer for Mock {
        fn do_stuff<T: Display>(&self, value: T) -> String {
            format!("{} from mock", value)
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(
            "my stuff from mock".to_string(),
            super::caller::call_do_stuff(Mock {}, "my stuff")
        );
    }
}
