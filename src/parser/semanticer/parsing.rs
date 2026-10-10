use crate::{
    model::Spanned,
    parser::{
        nparser::model::NParsedNode,
        semanticer::{
            model::{CommandId, FileId, SemanticCommandRepresent, SemanticedBuffer, Semanticer},
            schema::CommandSchema,
        },
    },
};

impl Semanticer {
    pub fn parse<'src>(&mut self, source: &'src str, file_id: FileId) -> SemanticedBuffer {
        let mut buffer = SemanticedBuffer::new_with_file_id(file_id);
        'wh: while let Some(node) = self.peek() {
            if let NParsedNode::Command(command) = node.content {
                if let Some(text) = command.name.span.text(source) {
                    if let Some(schema) = self.schemas.get(text) {
                        let mut command_rpre = SemanticCommandRepresent {
                            id: CommandId {
                                id: buffer.commands.len() as u32,
                            },
                            span: command.name.span,
                            args: Vec::new(),
                            kind: schema.kind.clone(),
                        };
                    } else {
                        continue 'wh;
                    };
                }
            }
        }
        todo!()
    }

    pub fn peek(&self) -> Option<Spanned<NParsedNode>> {
        self.buffer.nodes.get(self.cursor.offset).cloned()
    }
}
