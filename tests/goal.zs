// This is what I want Z# to look like (Still in progress).

type Result<T, E> = struct {
	// The fields of the type.
	private tag: u8;
	private payload: union<T, E>;

	// These look like methods, but they compile to static functions.
	public func is_ok(self): bool {
		// Direct access to the raw state defined above
		return self.tag == 0
	};
}