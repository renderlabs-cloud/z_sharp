// This is what I want Z# to look like.

public struct Result<T, E> {
	// === 1. STATE (The Raw Data) ===
	// This defines the memory layout. 
	// It maps directly to bytes. No hidden headers.
	private tag: u8,
	private payload: union<T, E>,

	// These look like methods, but they compile to static functions.
	public func is_ok(self) -> bool {
		// Direct access to the raw state defined above
		return self.tag == 0
	}

	public func unwrap(self) -> T {
		if self.is_ok() {
			// Unsafe intrinsic to read union field
			return unsafe_read(self.payload, T) 
		} else {
			// Panic is a std lib feature
			panic!("Called unwrap on Error") 
		};
	}
}