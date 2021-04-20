use inkwell::{basic_block::BasicBlock, values::BasicValueEnum};

#[derive(Debug)]
pub struct FunctionContext<'ctx> {
    function_name: usize,
    pub return_blocks: Vec<(BasicBlock<'ctx>, Option<BasicValueEnum<'ctx>>)>,
    pub return_block: BasicBlock<'ctx>,
}

impl<'ctx> FunctionContext<'ctx> {
    pub fn new(function_name: usize, return_block: BasicBlock<'ctx>) -> Self {
        Self {
            function_name,
            return_blocks: Vec::new(),
            return_block,
        }
    }

    pub fn add_return_block(
        &mut self,
        block: BasicBlock<'ctx>,
        value: Option<BasicValueEnum<'ctx>>,
    ) {
        self.return_blocks.push((block, value));
    }
}
