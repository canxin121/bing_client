#[macro_export]
macro_rules! vec_string {
    ($($element:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($element.to_string());
            )*
            temp_vec
        }
    };
}
