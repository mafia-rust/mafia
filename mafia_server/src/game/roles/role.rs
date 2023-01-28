
pub trait Role {
    type RoleData: RoleData;

    fn get_name(&self) -> &'static str;
    fn get_description(&self) -> &'static str;
    // fn get_alignment(&self) -> Alignment;
    // fn get_team(&self) -> Team;
}

pub trait RoleData {
    fn new() -> Self where Self: Sized;
}

macro_rules! create_role {
    (
        $name:ident 
        
        $description:literal

        // night target function

        // day target function

        // ...

        role-specific data: {
            $($data_ident:ident: $data_type:ty = $data_default:expr),*
        }
    ) => {
        struct $name {}

        #[allow(non_upper_case_globals)]
        static mut $name: $name = $name {};

        impl Role for $name {
            type RoleData = AdditionalRoleData;

            fn get_name(&self) -> &'static str {
                stringify!($name)
            }
        
            fn get_description(&self) -> &'static str {
                $description
            }
        }

        struct AdditionalRoleData {
            $($data_ident: $data_type),*
        }

        impl RoleData for AdditionalRoleData {
            fn new() -> Self {
                AdditionalRoleData {
                    $($data_ident: $data_default),*
                }
            }
        }
    };
}