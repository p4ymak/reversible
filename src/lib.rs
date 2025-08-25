#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[cfg(test)]
mod tests;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub struct Reversible<T: Default + Debug + Clone> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    data: T,
    #[cfg_attr(feature = "serde", serde(skip))]
    edit: Option<T>,
}
impl<T: Default + Debug + Clone> AsRef<T> for Reversible<T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}
impl<T: Default + Debug + Clone> AsMut<T> for Reversible<T> {
    fn as_mut(&mut self) -> &mut T {
        if self.edit.is_none() {
            self.force_edit();
        }
        self.edit.as_mut().expect("self.edit is always Some")
    }
}
impl<T: Default + Debug + Clone> std::fmt::Debug for Reversible<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}
impl<T: Default + Debug + Clone> Default for Reversible<T> {
    fn default() -> Self {
        Reversible {
            data: T::default(),
            edit: None,
        }
    }
}
impl<T: Default + Debug + Clone> From<T> for Reversible<T> {
    fn from(value: T) -> Self {
        Reversible::new(value)
    }
}

impl<T: Default + Debug + Clone> Reversible<T> {
    pub fn new(data: T) -> Self {
        Reversible { data, edit: None }
    }
    /// Reference to Edited Value
    pub fn as_ref_edit(&self) -> &T {
        match &self.edit {
            Some(edit) => edit,
            None => &self.data,
        }
    }

    /// Save Edited Value
    /// # Example
    /// ```
    /// let mut rev = reversible::Reversible::new(4);
    /// *rev.as_mut() = 13;
    /// assert_eq!(rev.as_ref(), &4);
    /// assert_eq!(rev.as_mut(), &mut 13);
    /// rev.save();
    /// assert_eq!(rev.as_ref(), &13);
    /// assert_eq!(rev.as_ref(), rev.as_ref_edit());
    /// ```
    pub fn save(&mut self) {
        if let Some(edit) = self.edit.take() {
            self.data = edit;
        }
    }

    /// Revert Edited Value
    /// # Example
    /// ```
    /// let mut rev = reversible::Reversible::new(4);
    /// *rev.as_mut() = 13;
    /// assert_eq!(rev.as_ref(), &4);
    /// assert_eq!(rev.as_mut(), &mut 13);
    /// rev.revert();
    /// assert_eq!(rev.as_mut(), &mut 4);
    /// assert_eq!(rev.as_ref(), rev.as_ref_edit());
    /// ```
    pub fn revert(&mut self) {
        self.edit = None;
    }

    /// Does it have Edited Value
    /// # Example
    /// ```
    /// let mut rev = reversible::Reversible::new(4);
    /// assert_eq!(rev.is_edit(), false);
    /// *rev.as_mut() = 13;
    /// assert!(rev.is_edit());
    /// ```
    pub fn is_edit(&self) -> bool {
        self.edit.is_some()
    }

    /// Force to have Edited Value same as Saved Value
    /// # Example
    /// ```
    /// let mut rev = reversible::Reversible::new(4);
    /// assert_eq!(rev.is_edit(), false);
    /// rev.force_edit();
    /// assert!(rev.is_edit());
    /// ```
    pub fn force_edit(&mut self) {
        self.edit = Some(self.data.clone());
    }
}
