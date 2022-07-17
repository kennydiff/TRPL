// K_22715 创建 tests/common/mod.rs ，而不是创建 tests/common.rs
// K_22715 这是一种 Rust 的命名规范，这样命名告诉 Rust 不要将 common 看作一个集成测试文件。 
pub fn setup() {
    // setup code specific to your library's tests would go here
}
