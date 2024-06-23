use std::sync::LazyLock;

pub type Static<T> = LazyLock<T>;
