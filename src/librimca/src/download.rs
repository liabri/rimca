use crate::Instance;

pub struct Download<T> {
	pub instance: Instance,
	pub inner: T,
}