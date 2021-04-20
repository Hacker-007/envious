use inkwell::{basic_block::BasicBlock, values::BasicValueEnum};

#[derive(Debug)]
pub struct FunctionContext<'ctx> {
    function_name: usize,
    pub return_blocks: Vec<(BasicBlock<'ctx>, BasicValueEnum<'ctx>)>,
}

impl<'ctx> FunctionContext<'ctx> {
    pub fn new(function_name: usize) -> Self {
        Self {
            function_name,
            return_blocks: Vec::new(),
        }
    }

    pub fn add_return_block(&mut self, block: BasicBlock<'ctx>, value: BasicValueEnum<'ctx>) {
        self.return_blocks.push((block, value));
    }
}