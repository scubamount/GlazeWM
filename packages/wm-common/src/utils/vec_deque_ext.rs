use std::collections::VecDeque;

pub trait VecDequeExt<T>
where
  T: PartialEq,
{
  /// Shifts a value to a specified index in a `VecDeque`.
  ///
  /// Inserts at index if value doesn't already exist in the `VecDeque`.
  fn shift_to_index(&mut self, target_index: usize, item: T);
}

impl<T> VecDequeExt<T> for VecDeque<T>
where
  T: PartialEq,
{
  fn shift_to_index(&mut self, target_index: usize, value: T) {
    if let Some(index) = self.iter().position(|e| e == &value) {
      self.remove(index);

      // Adjust for when the target index becomes out of bounds because of
      // the removal above.
      self.insert(target_index.clamp(0, self.len()), value);
    }
  }
}
