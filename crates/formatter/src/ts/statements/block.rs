use rslint_parser::ast::BlockStmt;

use crate::{
	format_elements, group_elements, join_elements, soft_indent, soft_line_break_or_space, token,
	FormatElement, ToFormatElement,
};

impl ToFormatElement for BlockStmt {
	fn to_format_element(&self) -> FormatElement {
		group_elements(format_elements![
			token("{"),
			soft_indent(join_elements(
				soft_line_break_or_space(),
				self.stmts().map(|stmt| stmt.to_format_element())
			)),
			token("}")
		])
	}
}