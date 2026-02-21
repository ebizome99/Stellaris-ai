//! 任务队列
//! 
//! 优先级队列和任务管理

use std::collections::VecDeque;
use parking_lot::Mutex;
use crate::core::types::{TaskId, TaskInfo, TaskStatus, Priority, GenerationParams};
use chrono::Utc;

/// 任务队列
pub struct TaskQueue {
    /// 高优先级队列
    high: Mutex<VecDeque<TaskInfo>>,
    /// 普通优先级队列
    normal: Mutex<VecDeque<TaskInfo>>,
    /// 低优先级队列
    low: Mutex<VecDeque<TaskInfo>>,
    /// 最大容量
    capacity: usize,
}

impl TaskQueue {
    /// 创建新的任务队列
    pub fn new(capacity: usize) -> Self {
        Self {
            high: Mutex::new(VecDeque::new()),
            normal: Mutex::new(VecDeque::new()),
            low: Mutex::new(VecDeque::new()),
            capacity,
        }
    }
    
    /// 添加任务
    pub fn push(&self, params: GenerationParams) -> Option<TaskId> {
        let task_id = TaskId::new();
        
        let task = TaskInfo {
            id: task_id,
            params,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            result: None,
            error: None,
            progress: 0,
            current_step: "等待中".to_string(),
        };
        
        let (total_len, queue) = {
            let high = self.high.lock().len();
            let normal = self.normal.lock().len();
            let low = self.low.lock().len();
            let total = high + normal + low;
            
            match task.params.priority {
                Priority::Realtime | Priority::High => (total, 0),
                Priority::Normal => (total, 1),
                Priority::Low => (total, 2),
            }
        };
        
        if total_len >= self.capacity {
            return None;
        }
        
        match queue {
            0 => self.high.lock().push_back(task),
            1 => self.normal.lock().push_back(task),
            _ => self.low.lock().push_back(task),
        }
        
        Some(task_id)
    }
    
    /// 弹出任务
    pub fn pop(&self) -> Option<TaskInfo> {
        if let Some(task) = self.high.lock().pop_front() {
            return Some(task);
        }
        if let Some(task) = self.normal.lock().pop_front() {
            return Some(task);
        }
        self.low.lock().pop_front()
    }
    
    /// 获取队列长度
    pub fn len(&self) -> usize {
        self.high.lock().len() + self.normal.lock().len() + self.low.lock().len()
    }
    
    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// 更新任务状态
    pub fn update_status(&self, task_id: TaskId, status: TaskStatus) -> bool {
        for queue in [&self.high, &self.normal, &self.low] {
            let mut q = queue.lock();
            if let Some(task) = q.iter_mut().find(|t| t.id == task_id) {
                task.status = status;
                return true;
            }
        }
        false
    }
    
    /// 移除任务
    pub fn remove(&self, task_id: TaskId) -> Option<TaskInfo> {
        if let Some(q) = self.high.lock().iter().position(|t| t.id == task_id) {
            return self.high.lock().remove(q);
        }
        if let Some(q) = self.normal.lock().iter().position(|t| t.id == task_id) {
            return self.normal.lock().remove(q);
        }
        if let Some(q) = self.low.lock().iter().position(|t| t.id == task_id) {
            return self.low.lock().remove(q);
        }
        None
    }
    
    /// 获取所有任务
    pub fn get_all(&self) -> Vec<TaskInfo> {
        let mut tasks = Vec::new();
        tasks.extend(self.high.lock().iter().cloned());
        tasks.extend(self.normal.lock().iter().cloned());
        tasks.extend(self.low.lock().iter().cloned());
        tasks
    }
    
    /// 清空队列
    pub fn clear(&self) {
        self.high.lock().clear();
        self.normal.lock().clear();
        self.low.lock().clear();
    }
}
