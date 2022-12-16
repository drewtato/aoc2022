use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
pub struct MinHeap<T: Ord> {
	inner: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> Default for MinHeap<T> {
	fn default() -> Self {
		Self {
			inner: Default::default(),
		}
	}
}

impl<T: Ord> MinHeap<T> {
	pub fn new() -> Self {
		Self {
			inner: Default::default(),
		}
	}

	pub fn len(&self) -> usize {
		self.inner.len()
	}
	pub fn is_empty(&self) -> bool {
		self.inner.is_empty()
	}
	pub fn pop(&mut self) -> Option<T> {
		self.inner.pop().map(|item| item.0)
	}
	pub fn push(&mut self, item: T) {
		self.inner.push(Reverse(item))
	}
	pub fn clear(&mut self) {
		self.inner.clear()
	}
	pub fn peek(&self) -> Option<&T> {
		self.inner.peek().map(|item| &item.0)
	}
}

impl<T: Ord> FromIterator<T> for MinHeap<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		Self {
			inner: FromIterator::from_iter(iter.into_iter().map(|i| Reverse(i))),
		}
	}
}
