mod defer_movements;
mod from_ast_to_ir;
mod fuse_add;
mod fuse_movements;
mod mul_loop_optimization;
mod unroll_zero_loops;

pub use {
    defer_movements::defer_movements, from_ast_to_ir::from_ast_to_ir, fuse_add::fuse_add,
    fuse_movements::fuse_movements, mul_loop_optimization::mul_loop_optimization,
    unroll_zero_loops::unroll_zero_loops,
};
