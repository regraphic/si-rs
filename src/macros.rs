#[macro_export]
macro_rules! preset {
    ($name:ident ($img:ident, $($key:ident : $type:ty),*) {$( $body:stmt );* $(;)?}) => {
            let $name = SiPreset::new(Box::new(|img, vals: HashMap<String, Box<dyn std::any::Any>>| {
                $(
                    let $key = match vals.get(stringify!($key)) {
                        Some($key) => {
                            // Do type checking
                            if $key.type_id() == std::any::TypeId::of::<$type>() {
                                // Downcast it
                                $key.downcast_ref::<$type>().unwrap()
                            } else {
                                panic!("Expected type: {:?}\nFound type: {:?}", std::any::TypeId::of::<$type>(), $key.type_id());
                            }
                        }
                        None => panic!("No {} provided", stringify!($key))
                    };
                )*;
                let $img = img.clone();
                $($body)*
            }
            ));
    }
}

#[macro_export]
macro_rules! anymap {
    {$($key:ident : $val:expr),*} => {
        {
            let mut map = HashMap::new();
            $(
                map.insert(stringify!($key).to_string(), Box::new($val) as Box<dyn std::any::Any>);
            )*
            map
        }
    };
}
