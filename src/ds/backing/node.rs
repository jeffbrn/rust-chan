pub trait NodeData {
	type TVal;

	fn get_val(&self) -> Self::TVal;
	fn set_val(&mut self, new_val: Self::TVal);
}
