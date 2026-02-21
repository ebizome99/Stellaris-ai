//! 调度器模块
//! 
//! GPU调度和混合算力调度

pub mod gpu;
pub mod hybrid;
pub mod task_queue;

pub use gpu::GPUScheduler;
pub use hybrid::HybridComputeScheduler;
pub use task_queue::TaskQueue;
