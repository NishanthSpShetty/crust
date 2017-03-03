#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub enum DocType {
    NO_STRICT,
    STRICT,
    STRUCT_INIT,
    DEFAULT,
}

impl DocType {
    pub fn get_doc(self) -> &'static str {
        match self {
            DocType::STRICT => {
                "\n/*Crust with Strict Mode enabled, declares all variables as immutable.\n * If \
                 you are mutating the below variable anywhere in program, please change the \
                 declaration statement as\n * let mut var_name:type=init_val;\n **/\n"
            }
            DocType::NO_STRICT => {
                "\n/*Avoid using mutable variables unless it is necessary to do so\n */\n"
            }
            DocType::STRUCT_INIT => {
                "\n/* Declaration of a structure should be completed with initialization of it's \
                 fields\n * Parser may miss the generation of initialization statements.\n * It \
                 should be in the following format\n * let variable:struct_name = struct_name { \
                 member1:value1, member2:value2,..}\n */"
            }
            _ => "//Doc Not Found. Please Report bug",
        }
    }
}
