use analysis;
use library;

#[derive(Clone)]
pub struct Parameter {
    pub name: String,
    pub typ: library::TypeId,
    pub transfer: library::Transfer,
    pub caller_allocates: bool,
    pub is_error: bool,
}

impl Parameter {
    pub fn new(orig: &analysis::function_parameters::CParameter) -> Parameter {
        Parameter {
            name: orig.name.clone(),
            typ: orig.typ,
            transfer: orig.transfer,
            caller_allocates: orig.caller_allocates,
            is_error: orig.is_error,
        }
    }
}
