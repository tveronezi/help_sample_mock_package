mod service {
    use std::fmt::Display;

    pub fn do_stuff<T: Display>(value: T) -> String {
        format!("{}", value)
    }
}

pub mod caller {
    use super::service;

    pub fn call_do_stuff(value: &str) -> String {
        service::do_stuff(value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("my stuff".to_string(), super::caller::call_do_stuff("my stuff"));
    }
}
