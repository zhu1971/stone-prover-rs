// https://github.com/starkware-libs/stone-prover/blob/a78ff37c1402dc9c3e3050a1090cd98b7ff123b3/src/starkware/air/air.h
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// #![feature(generic_const_exprs)]

use std::{collections::HashMap, option::Option, vec::Vec};

use algebra::FieldElementVector;
use ark_ff::Field;

// doc(tk)
// refactor(tk): maybe assoc type in Air
pub trait CompositionPolynomial<F: Field> {}

// doc(tk)
pub trait Air<F: Field> {
  // todo(tk): constructor in trait?
  fn new(trace_length: usize) -> Self
  where Self: Sized {
    assert!(trace_length.is_power_of_two(), "trace length must be power of 2");

    todo!()
  }

  /// Creates a CompositionPolynomial object based on the given (verifier-chosen) coefficients.
  fn create_composition_polynomial(
    &self,
    trace_generator: F,
    random_coefficients: &[F],
    // refactor(tk): dyn bad
  ) -> Box<dyn CompositionPolynomial<F>>;

  // refactor(tk) - public inner?
  fn trace_length(&self) -> usize;

  /// Default to zero
  fn get_composition_polynomial_degree_bound(&self) -> usize;

  /// Number of random coefficients that are chosen by the verifier and affect the constraint.
  /// They are the coefficients of the linear combination of the constraints and must be random in
  /// order to maintain soundness.
  fn num_random_coefficients(&self) -> usize;

  // refactor(tk): redundant?
  fn get_num_constraints(&self) -> usize { self.num_random_coefficients() }

  /// Returns a list of pairs (relative_row, col) that define the neighbors needed for the
  /// constraint.
  fn get_mask() -> AirMask;

  fn num_columns(&self) -> usize;

  /// When the AIR has interaction (i.e. for debugging and testing), clones the AIR and updates its
  /// interaction elements. Returns the cloned AIR. Otherwise, this function shouldn't be used.
  // todo: constructor in trait?
  // refactor(tk): dyn bad
  fn with_interaction_elements(&self, interaction_elms: &FieldElementVector) {
    // panic if called in release
    #[cfg(not(debug_assertions))]
    {
      panic!("Calling WithInteractionElements in an air with no interaction.");
    }
    todo!()
  }

  /// Returns the interaction parameters.
  /// If there is no interaction, returns None.
  fn get_interaction_params(&self) -> Option<InteractionParams>;

  /// If air has interaction, returns the number of columns in the first trace;
  /// otherwise, returns the total number of columns.
  fn get_n_columns_first(&self) -> usize {
    match self.get_interaction_params() {
      Some(params) => params.n_columns_first,
      None => self.num_columns(),
    }
  }
}

/// Helper NewType for the `get_mask` of the Air.
// refactor(tk): newtype->alias?
pub struct AirMask(pub Vec<(i64, u64)>);

/// Stores data relevant to the interaction.
pub struct InteractionParams {
  // Number of columns in first and second trace.
  pub n_columns_first:  usize,
  pub n_columns_second: usize,

  // Number of interaction random elements.
  pub n_interaction_elements: usize,
}