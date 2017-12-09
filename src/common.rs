
macro_rules! type_states {
    ($type:ident, ($($id:ident),*)) => {
        pub trait $type {}
        
        $(
            pub struct $id {}
            impl $type for $id {}
        )*
    };
}

macro_rules! type_group {
    ($type:ident, ($($id:ident),*)) => {
        pub trait $type {}
        
        $(
            impl $type for $id {}
        )*
    };
}