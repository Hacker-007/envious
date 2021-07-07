use std::{collections::{HashMap, HashSet}, rc::Rc};

use super::{constraints::Constraint, monotype::{Monotype, MonotypeRef}};

#[derive(Debug)]
pub struct Substitution {
    solutions: HashMap<usize, MonotypeRef>
}

impl Substitution {
    pub fn new() -> Self {
        Self {
            solutions: HashMap::new(),
        }
    }

    pub fn apply_constraints(&self, constraints: HashSet<Constraint>) -> HashSet<Constraint> {
        constraints.iter()
                .map(|constraint| self.apply_constraint(constraint))
                .collect()
    }

    pub fn compose(&self, other: Substitution) -> Substitution {
        let mut substituted_self = self.solutions.iter().map(|(key, value)| (*key, other.apply_type(value.clone()))).collect::<HashMap<_, _>>();
        substituted_self.extend(other.solutions);
        Substitution {
            solutions: substituted_self,
        }
    }

    fn apply_constraint(&self, constraint: &Constraint) -> Constraint {
        match constraint {
            Constraint::Equal(left, right) => {
                Constraint::Equal(self.apply_type(left.clone()), self.apply_type(right.clone()))
            }
        }
    }

    fn apply_type(&self, ty: MonotypeRef) -> MonotypeRef {
        self.solutions.iter()
            .fold(ty, |result, (id, solution_ty)| {
                self.substitute(result, *id, solution_ty.clone())
            })
    }

    fn substitute(&self, ty: MonotypeRef, id: usize, replacement: MonotypeRef) -> MonotypeRef {
        match &*ty {
            Monotype::Int
            | Monotype::Float
            | Monotype::Boolean
            | Monotype::Char
            | Monotype::Void
            | Monotype::Custom(_) => ty,
            Monotype::Function { parameters, ret } => {
                Rc::new(Monotype::Function {
                    parameters: parameters.iter().map(|parameter| self.substitute(parameter.clone(), id, replacement.clone())).collect::<Vec<_>>(),
                    ret: self.substitute(ret.clone(), id, replacement),
                })
            }
            &Monotype::Existential(second_var_id) => if second_var_id == id { replacement } else { ty }
        }
    }
}
