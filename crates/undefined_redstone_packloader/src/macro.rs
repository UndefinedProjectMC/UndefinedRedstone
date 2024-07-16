#[macro_export]
macro_rules! components_export {
    ($struct_name:ident, $($name:ident = $alias:expr,)*) => {
        // 定义主结构体
        #[allow(non_snake_case)]
        #[derive(Deserialize, Clone)]
        #[serde(untagged)]
        pub enum $struct_name {
            Map(Map<String, Value>)
        }
        impl std::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut f = f.debug_struct(stringify!($struct_name));
                match self {
                    $struct_name::Map(map) => {
                        for (key, value) in map {
                            match key.as_str() {
                                $(
                                    $alias => {
                                        if let Ok(component) = serde_json::from_value::<$name>(value.clone()) {
                                            f.field(stringify!($name), &component);
                                        }
                                    },
                                )*
                                _ => {}
                            }
                        }
                    },
                }
                f.finish()
            }
        }
        impl $struct_name {
            pub fn spawn(self, entity: &mut EntityWorldMut) {
                match self {
                    $struct_name::Map(map) => {
                        for (key, value) in map {
                            match key.as_str() {
                                $(
                                    $alias => {
                                        if let Ok(component) = serde_json::from_value::<$name>(value) {
                                            entity.insert(component);
                                        }
                                    },
                                )*
                                _ => {}
                            }
                        }
                    },
                }
            }
        }
    };
}

#[macro_export]
macro_rules! types_export {
    ($struct_name:ident, $enum_name:ident, $($name:ident = $alias:expr,)*) => {
        // 定义主结构体
        pub enum $enum_name {
            Empty,
            $(
                $name($name),
            )*
        }
        #[allow(non_snake_case)]
        #[derive(Deserialize, Debug)]
        pub struct $struct_name {
            pub format_version: String,
            $(
                #[serde(alias = $alias)]
                pub $name: Option<$name>,
            )*
        }
        impl $struct_name {
            pub fn get(self) -> $enum_name {
                $(
                    if let Some(mut component) = self.$name {
                        component.format_version = self.format_version;
                        return $enum_name::$name(component);
                    }
                )*
                return $enum_name::Empty;
            }
        }
    };
}