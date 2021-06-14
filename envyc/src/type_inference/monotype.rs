use std::rc::Rc;

use crate::interner::Interner;

#[derive(Debug, PartialEq, Eq)]
pub enum Monotype {
    Int,
    Float,
    Boolean,
    Char,
    Void,
    Existential(usize),
    Function {
        parameters: Vec<MonotypeRef>,
        ret: MonotypeRef,
    },
    Custom(usize),
}

impl Monotype {
    pub fn to_string(&self, interner: &Interner<String>) -> String {
        match self {
            Monotype::Int => "Int".to_string(),
            Monotype::Float => "Float".to_string(),
            Monotype::Boolean => "Boolean".to_string(),
            Monotype::Char => "Char".to_string(),
            Monotype::Void => "Void".to_string(),
            Monotype::Existential(id) => format!("µ{}", *id),
            Monotype::Function { parameters, ret } => {
                let formatted_parameters = parameters
                    .iter()
                    .map(|parameter| parameter.to_string(interner))
                    .collect::<Vec<_>>();

                format!(
                    "({}) :: {}",
                    formatted_parameters.join(","),
                    ret.to_string(interner)
                )
            }
            &Monotype::Custom(id) => interner.get(id).to_string(),
        }
    }
}

pub type MonotypeRef = Rc<Monotype>;
