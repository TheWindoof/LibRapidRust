//! Diving deeper into mathematics, huh? In here you'll find mathematical sets - Sometimes pretty handy!
/// Brings mathematical sets into Rust.
#[derive(Debug, Clone)]
pub struct VecSet<'a, T> {
    elements: Vec<T>,
    parent: Option<&'a VecSet<'a, T>>,
}

// Main impl
impl<'a, T: Copy + Ord> VecSet<'a, T> {
    /// Creates a new `VecSet`.
    /// # Arguments
    /// * `values` - The values for the `VecSet`.
    /// # Returns
    /// A new `VecSet`.
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// 
    /// let set = VecSet::new(&vec![0, 1, 2, 3, 4, 5]);
    /// assert_eq!(set.elements(), &vec![0, 1, 2, 3, 4, 5]);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(values: &[T]) -> VecSet<'a, T> {        
        let mut res: VecSet<T> = VecSet { elements: values.to_vec(),
                                          parent: None };
        res.elements.sort_unstable();
        res.elements.dedup();
        res
    }
    /// Get the empty set, ∅.
    /// # Returns
    /// A `VecSet<T>` with no elements.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::{VecSet, set};
    /// let v: Vec<u8> = Vec::new();
    /// 
    /// assert_eq!(v, VecSet::empty_set().elements());
    /// ```
    #[inline]
    #[must_use]
    pub fn empty_set() -> VecSet<'a, T> {
        VecSet { elements: Vec::new(), parent: None }
    }
    /// Creates a new `VecSet` using a parent-`VecSet` to which it applies a closure (A rule which is applied to each element and checks if this rule holds true).
    /// # Arguments
    /// * `parent` - The `VecSet` from which the new `VecSet` emerges.
    /// * `f` - The closure after which the new `VecSet` is created.
    /// # Returns
    /// A child `VecSet`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// let test1:       VecSet<u8> = VecSet::new(&vec![0,1,2,3,4]);
    /// let from_parent: VecSet<u8> = VecSet::<u8>::new_subset(&test1, |x| x % 2 == 0);
    /// assert_eq!(from_parent, VecSet::new(&vec![0,2,4]));
    /// assert_eq!(test1.elements(), &vec![0,1,2,3,4])
    /// ```
    #[must_use]
    pub fn new_subset<F: Fn(T) -> bool>(parent: &'a VecSet<T>, f: F) -> VecSet<'a, T> {
        let mut res: VecSet<T> = VecSet { elements: Vec::with_capacity(parent.cardinality()),
                                          parent: Some(parent) };
        for elem in parent {
            if f(elem) {
                res.elements.push(elem);
            }
        }
        res
    }
    /// Does a mathematical union on two VecSets.
    /// \
    /// In other words, it creates a new set of all values that are either in `self` or `other` or both.
    /// `self ∪ other`.
    /// # Arguments
    /// * `self` - The first set.
    /// * `other` - The second set.
    /// # Returns
    /// A new `VecSet<T>`: `self ∪ other`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// use lib_rapid::compsci::general::BinaryInsert;
    /// let s:  VecSet<i32> = VecSet::new(&vec![0,1,2,3,4,5,6,7,8,9,10]);
    /// let s1: VecSet<i32> = VecSet::new(&vec![11,12,13,13,11,0,0,0]);
    /// 
    /// let c:  VecSet<i32> = s.union(&s1);
    /// assert_eq!(c, set!(0,1,2,3,4,5,6,7,8,9,10,11,12,13));
    /// ```
    #[must_use = "This returns the result of the operation, without modifying the original."]
    pub fn union(&self, other: &VecSet<T>) -> VecSet<T> {
        let mut res: VecSet<T> = VecSet {elements: Vec::with_capacity(self.cardinality() +
                                                                      other.cardinality()),
                                         parent:   None };

        res.elements.extend_from_slice(&self.elements);
        res.elements.extend_from_slice(&other.elements);

        res.elements.sort_unstable(); 
        res.elements.dedup();
        res
    }
    /// Does a mathematical intersection on two sets.
    /// \
    /// In other words: It creates a new set of all values that are present in both sets.
    /// # Arguments
    /// * `self` - The first set.
    /// * `other` - The second set.
    /// # Returns
    /// A new `VecSet<T>`: `self ∩ other`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// use lib_rapid::compsci::general::BinaryInsert; // Used for "set!"
    /// 
    /// let s:  VecSet<i32> = VecSet::new(&vec![0,1,2,3,4,5,6,7,8,9,10,11]);
    /// let s2: VecSet<i32> = VecSet::new(&vec![0,1,2,3,11,0,0,0]);
    /// 
    /// let c:  VecSet<i32> = s.intersection(&s2);
    /// assert_eq!(c, set!(0, 1, 2, 3, 11));
    /// ```
    #[must_use = "This returns the result of the operation, without modifying the original."]
    pub fn intersection(&self, other: &VecSet<T>) -> VecSet<T> {
        let mut res: VecSet<T> = self.clone();
        res.elements.retain(|x| other.elements.binary_search(x).is_ok());

        res
    }
    /// Checks for disjointness between `self` and `other`.
    /// \
    /// Two sets are disjoint if and only if `self` has no elements of `other`.
    /// The two sets `A := {1, 2, 3}` and `B := {1, 5, 6}` are *not* disjoint, because they
    /// share the common element `1`.
    /// # Arguments
    /// * `other` - The other set to be checked for disjointness.
    /// # Returns
    /// A `bool`
    /// # Examples
    ///```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let s1 = set!(0, 1, 2, 3, 4, 5, 6, 7, 8);
    /// let s2 = set!(0, 1, 2, 3, 4, 5, 6);
    /// let s3 = set!(8, 9, 10, 11, 12, 13, 14);
    /// let s4 = set!(9, 10, 11, 12, 13, 14, 15);
    /// 
    /// assert_eq!(false, s1.is_disjoint_with(&s2));
    /// assert_eq!(false, s1.is_disjoint_with(&s3));
    /// assert_eq!(true, s1.is_disjoint_with(&s4));
    /// ```
    #[must_use]
    pub fn is_disjoint_with(&self, other: &VecSet<T>) -> bool {
        for i in self {
            if other.has_element(i)
            { return false; }
        }
        true
    }
    /// Checks for mathematical difference between two sets - `A \ B`.
    /// \
    /// The difference between two sets is the set of all elements that are present in a set `A`,
    /// but not in the set `B`. For example, the difference between `A := {1, 5, 6}` and `B := {2, 3, 4}` is `{5, 6}`, because `5` and `6` are the elements not present in `B`.
    /// # Arguments
    /// * `other` - The other set to be checked for.
    /// # Returns
    /// A `VecSet<T>`.
    /// # Examples
    ///```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let s1 = set!(1, 2, 3, 4, 5);
    /// let s2 = set!(3, 4, 5);
    /// 
    /// assert_eq!(set!(1, 2), s1.difference_with(&s2));
    ///```
    #[must_use = "This returns the result of the operation, without modifying the original."]
    pub fn difference_with(&self, other: &VecSet<T>) -> VecSet<T> {
        let mut res_vec: Vec<T> = Vec::with_capacity(std::cmp::max(self.cardinality(), other.cardinality()));
        for i in self {
            if !other.has_element(i)
            { res_vec.push(i); }
        }

        VecSet { elements: res_vec,
                 parent: None }
    }
    /// Gets the cartesian product of two sets in `O(n·m)`.
    /// \
    /// The cartesian product of two sets is the set of all tuples of the form (a, b), such
    /// that every unique permutation is captured. Let `A := {1, 2}` and `B := {3, 4}`. Then, the cartesian product `C` is `C := {(1, 3), (1, 4), (2, 3), (2, 4)}`.
    /// # Arguments
    /// * `other` - `&VecSet<T>`
    /// # Returns
    /// A `VecSet<(T, T)>`.
    ///```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let s1 = set!(1, 2, 3);
    /// let s2 = set!(1, 2);
    /// 
    /// assert_eq!(set!((1, 1), (1, 2), (2, 1), (2, 2), (3, 1), (3, 2)), s1.cartesian_product(&s2));
    ///```
    #[must_use = "This returns the result of the operation, without modifying the original."]
    pub fn cartesian_product(&self, other: &VecSet<T>) -> VecSet<(T, T)> {
        let mut res_vec = Vec::with_capacity(self.cardinality() * other.cardinality());
        for i in self {
            for j in other {
                res_vec.push((i, j));
            }
        }
        VecSet { elements: res_vec, parent: None }
    }
    /// Gets the symmetric difference.
    /// \
    /// The symmetric difference is the set of all
    /// elements that are either in `self` or `other`, but not in both.
    /// # Arguments
    /// * `other` - A `&VecSet<T>`.
    /// # Examples
    ///```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let s1 = set!(1, 2, 3, 4, 5);
    /// let s2 = set!(3, 4);
    /// 
    /// assert_eq!(set!(1, 2, 5), s1.symmetric_difference_with(&s2));
    ///```
    #[must_use = "This returns the result of the operation, without modifying the original."]
    pub fn symmetric_difference_with(&self, other: &VecSet<T>) -> VecSet<T> {
        let diff1 = self.difference_with(&other);
        let diff2 = other.difference_with(&self).clone();

        VecSet {elements: diff1.union(&diff2).elements,
                parent:   None }
    }
    /// Lets you check for an element in a set.
    /// # Arguments
    /// * `elem` - The element to check for.
    /// # Returns
    /// A boolean value which determines if `elem ∈ self`. 
    /// 
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let set = set!(0, 1, 2, 3, 4, 5, 6);
    /// 
    /// assert_eq!(false, set.has_element(7));
    /// assert_eq!(false, set.has_element(-1));
    /// assert_eq!(true, set.has_element(1));
    /// ```
    #[inline]
    #[must_use]
    pub fn has_element(&self, elem: T) -> bool {
        self.elements.binary_search(&elem).is_ok()
    }
    /// Lets you insert an element into a set. Does not insert already present values.
    /// \
    /// Mathematically speaking, it takes the set `A` and a set `B`, where the set B is defined as
    /// `B := {b}`, where `b` is the new value. `A` is then redefined as `A = A_{pre} ∪ B`.
    /// # Arguments
    /// * `elem` - The element to insert.
    /// # Returns
    /// Nothing.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// let mut s: VecSet<i32> = VecSet::new(&vec![0,1,2,3,4,5,6,7,8,9,10]);
    /// 
    /// s.insert(5);
    /// assert_eq!(s.elements(), &vec![0,1,2,3,4,5,6,7,8,9,10]);
    /// ```
    pub fn insert(&mut self, elem: T) {
        self.elements.binary_insert_no_dup(elem)
    }
    /// Lets you check wether a set has a parent (emerged from another set) or not.
    /// # Returns
    /// A boolean value which determines if the set is a subset of any other set.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let set = set!(0, 1, 2, 3, 4, 5, 6);
    /// let subset = VecSet::new_subset(&set, |x| x % 2 == 0);
    /// 
    /// assert_eq!(true, subset.has_emerged());
    /// assert_eq!(false, set.has_emerged());
    /// ```
    #[inline]
    #[must_use]
    pub fn has_emerged(&self) -> bool {
        self.parent.is_some()
    }
    /// Gets you the optional superset from which the Set emerged.
    /// # Returns
    /// A `Option<&VecSet<T>>`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let set = set!(0, 1, 2, 3, 4, 5, 6);
    /// let subset = VecSet::new_subset(&set, |x| x % 2 == 0);
    /// 
    /// assert_eq!(&set, subset.get_parent().unwrap());
    /// ```
    #[inline]
    #[must_use]
    pub fn get_parent(&self) -> Option<&VecSet<T>> {
        self.parent
    }
    /// Determines whether `self` is a subset of `other`, unconditional from whether `self` emerged from `other`.
    /// # Arguments
    /// * `other` - A `VecSet<T>`.
    /// # Returns
    /// A `bool`
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let set  = set!(0, 1, 2, 3, 4, 5, 6);
    /// let set2 = set!(0, 1, 2, 3, 4);
    /// 
    /// assert_eq!(false, set.is_subset_of(&set2));
    /// assert_eq!(true, set2.is_subset_of(&set));
    /// ```
    #[must_use]
    pub fn is_subset_of(&self, other: &VecSet<T>) -> bool {
        for i in self {
            if !other.has_element(i)
            { return false; }
        }
        true
    }
    /// Gets the cardinality of a set.
    /// \
    /// The cardinality of a set is the count of all elements.
    /// # Returns
    /// A `usize`: `|self|`.
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let set = set!(0, 1, 2, 3, 4, 5, 6);
    /// 
    /// assert_eq!(7, set.cardinality());
    /// ```
    #[inline]
    #[must_use]
    pub fn cardinality(&self) -> usize {
        self.elements.len()
    }
    /// Lets you set the elements of a set.
    /// # Arguments
    /// * `vals` - The Vec to change the values to.
    /// # Returns
    /// Nothing. 
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let mut set = set!(0, 1, 2, 3, 4, 5, 6);
    /// set.set_elements(&vec![0, 2, 4, 6]);
    /// 
    /// assert_eq!(&vec![0, 2, 4, 6], set.elements());
    /// ```
    #[inline]
    pub fn set_elements(&mut self, vals: &[T]) {
        self.elements = vals.to_vec();
        self.elements.sort_unstable();
    }
    /// Lets you get the elements of a set.
    /// # Arguments
    /// * none
    /// # Returns
    /// A `&[T]` containing all elements. 
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// use lib_rapid::math::sets::vec_sets::set;
    /// 
    /// let mut set = set!(0, 1, 2, 3, 4, 5, 6);
    /// 
    /// assert_eq!(&vec![0, 1, 2, 3, 4, 5, 6], set.elements());
    /// ```
    #[inline]
    #[must_use]
    pub fn elements(&self) -> &[T] {
        &self.elements
    }
}

/// Creates a new `VecSet` more elegantly from values.
/// # Returns
/// A new `VecSet`.
/// # Examples
/// ```
/// use lib_rapid::set;
/// use lib_rapid::math::sets::vec_sets::VecSet; 
/// 
/// let set: VecSet<i32> = set!(0,1,2,3,4,5,6,-1);
/// ```
#[macro_export]
macro_rules! set {
    ( $( $a:expr ),* ) => {
        {
            use lib_rapid::compsci::general::BinaryInsert;
            let mut res_vec = Vec::with_capacity(20);
            $(
                res_vec.binary_insert_no_dup($a);
            )*
            VecSet::new(&res_vec)
        }
    };
}
pub use set;

use crate::compsci::general::BinaryInsert;

impl<T: ToString + Copy + Ord> VecSet<'_, T> {
    /// Lets you print a set with all its parents recursively.
    /// # Returns
    /// Nothing. 
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// let s:  VecSet<i32> = VecSet::new(&vec![0,1,2,3,4,5,6,7,8,9,10]);
    /// let s1: VecSet<i32> = VecSet::new_subset(&s, |x| x % 2 == 0);
    /// let s2: VecSet<i32> = VecSet::new_subset(&s1, |x| x == 4);
    /// 
    /// s2.full_println(); // Prints this set and the superset, see to_full_string.
    /// println!("{}", s2); // Only prints this set
    /// ```
    pub fn full_println(&self) {
        println!("{}", self.rec_to_string(&mut String::new()));
    }
    /// Converts a set with all subsets to a string.
    /// # Returns
    /// A String containing the result. 
    /// # Examples
    /// ```
    /// use lib_rapid::math::sets::vec_sets::VecSet;
    /// let s:  VecSet<i32> = VecSet::new(&vec![0,1,2,3,4,5,6,7,8,9,10]);
    /// let s1: VecSet<i32> = VecSet::new_subset(&s, |x| x % 2 == 0);
    /// let s2: VecSet<i32> = VecSet::new_subset(&s1, |x| x == 4);
    /// assert_eq!(s2.to_full_string(), "{ 4 } ⊆ { 0; 2; 4; 6; 8; 10 } ⊆ { 0; 1; 2; 3; 4; 5; 6; 7; 8; 9; 10 }".to_string());
    /// ```
    #[must_use]
    pub fn to_full_string(&self) -> String {
        self.rec_to_string(&mut String::new())
    }

    fn rec_to_string(&self, string: &mut String) -> String {
        string.push_str(&self.to_string()); // The child-set at the bottom
        if let Some(x) = self.parent { string.push_str(" ⊆ "); // Add subset-character
        x.rec_to_string(string); } // Recursively append parent sets
        string.to_string()
    }
}

// Indexing for Sets
impl<T> std::ops::Index<usize> for VecSet<'_, T> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

// Implement Printing
impl<T: ToString + Copy + Ord> std::fmt::Display for VecSet<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res: String = String::from('{');
        for elem in self.elements() {
            res.push(' ');
            res.push_str(&elem.to_string());
            res.push(';');
        }
        res.pop();
        write!(f, "{} }}", res)
    }
}

// Implement Equality
impl<T: PartialEq> PartialEq for VecSet<'_, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl<T> IntoIterator for VecSet<'_, T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}
impl<T: Copy + Clone + Ord> IntoIterator for &VecSet<'_, T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements().to_owned().into_iter()
    }
}