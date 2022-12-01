use std::array;

use std::iter::Flatten;

use crate::Grid;

pub type Neighbors<'a, T> = NeighborsExtra<'a, 3, 3, T>;

pub type NeighborsExtra<'a, const Y: usize, const X: usize, T> = [[Option<&'a T>; X]; Y];

pub trait GetNeighbors {
	type Neighbor;

	fn neighbors(&self, y: usize, x: usize) -> Neighbors<Self::Neighbor> {
		self.neighbors_extra(y, x)
	}

	fn neighbors_extra<const Y_LEN: usize, const X_LEN: usize>(
		&self,
		y: usize,
		x: usize,
	) -> NeighborsExtra<Y_LEN, X_LEN, Self::Neighbor> {
		self.neighbors_extra_offset(y, x, (Y_LEN as isize - 1) / -2, (X_LEN as isize - 1) / -2)
	}

	fn neighbors_extra_offset<const Y_LEN: usize, const X_LEN: usize>(
		&self,
		y: usize,
		x: usize,
		off_y: isize,
		off_x: isize,
	) -> NeighborsExtra<Y_LEN, X_LEN, Self::Neighbor>;

	fn neighbors_iter(&self, y: usize, x: usize) -> NeighborIter<Self::Neighbor, 3, 3> {
		self.neighbors(y, x).into_iter().flatten().flatten()
	}

	fn neighbors_extra_iter<const Y_LEN: usize, const X_LEN: usize>(
		&self,
		y: usize,
		x: usize,
	) -> NeighborIter<Self::Neighbor, Y_LEN, X_LEN> {
		self.neighbors_extra::<Y_LEN, X_LEN>(y, x)
			.into_iter()
			.flatten()
			.flatten()
	}

	fn neighbors_extra_offset_iter<const Y_LEN: usize, const X_LEN: usize>(
		&self,
		y: usize,
		x: usize,
		off_y: isize,
		off_x: isize,
	) -> NeighborIter<Self::Neighbor, Y_LEN, X_LEN> {
		self.neighbors_extra_offset::<Y_LEN, X_LEN>(y, x, off_y, off_x)
			.into_iter()
			.flatten()
			.flatten()
	}
}

type NeighborIter<'a, T, const Y: usize, const X: usize> =
	Flatten<Flatten<std::array::IntoIter<[Option<&'a T>; X], Y>>>;

impl<T> GetNeighbors for Grid<T> {
	type Neighbor = T;

	fn neighbors_extra_offset<const Y_LEN: usize, const X_LEN: usize>(
		&self,
		y: usize,
		x: usize,
		off_y: isize,
		off_x: isize,
	) -> NeighborsExtra<Y_LEN, X_LEN, Self::Neighbor> {
		array::from_fn(|dy| {
			let ny = y + dy;
			let ny = (ny as isize + off_y) as usize;
			array::from_fn(|dx| {
				let nx = x + dx;
				let nx = (nx as isize + off_x) as usize;
				self.get(ny).and_then(|row| row.get(nx))
			})
		})
	}
}
