use crate::{
	lexer::capture::{Chain, CaptureResult, CaptureResultsMap},
	modification::gluea::{LuaRc},
};

use ::mlua_magic_macros;

/// The [`AbstractSyntaxTree`] is a tree of [`Node`]s.
#[derive(Clone)]
#[mlua_magic_macros::structure]
pub struct AbstractSyntaxTree {
	pub nodes: Vec<Node>,
}

impl AbstractSyntaxTree {
	pub fn new(capture_results: Vec<CaptureResult>) -> Self {
		let nodes: Vec<Node> = Vec::new();

		return Self { nodes: nodes };
	}
}

mlua_magic_macros::compile!(type_path = AbstractSyntaxTree, fields = true);

#[derive(Clone)]
#[mlua_magic_macros::enumeration]
pub enum Node {
	Branch(AbstractSyntaxTree),
	Leaf(CaptureResultsMap /*Option<CORAL::Block>*/),
}

mlua_magic_macros::compile!(type_path = Node, variants = true);

#[derive(Clone)]
#[mlua_magic_macros::structure]
pub struct Accumulator {
	pub(self) index: usize,
	pub(self) size: usize,
	pub(self) tree_ref: LuaRc<AbstractSyntaxTree>,
}

mlua_magic_macros::compile!(type_path = Accumulator, fields = true, methods = true);

#[mlua_magic_macros::implementation]
impl Accumulator {
	/// Creates a new [`Accumulator`] from an [`AbstractSyntaxTree`] and a index.
	/// The starting size is `0`.
	pub fn of(tree: AbstractSyntaxTree, index: usize) -> Self {
		return Self {
			index: index,
			size: 0,
			tree_ref: LuaRc::new(tree),
		};
	}

	/// Expands the [`Accumulator`] by `1`.
	pub fn next(&mut self) -> () {
		self.size += 1;
	}

	/// Returns the [`Node`] at the specified index or [`None`] if the index is out of bounds.
	pub fn get(&self, index: usize) -> Option<Node> {
		if index >= self.size {
			return None;
		};

		let node: Option<Node> = (*self.tree_ref).nodes.get(index).cloned(); // As always, we must clone for safety.

		return node;
	}
}
