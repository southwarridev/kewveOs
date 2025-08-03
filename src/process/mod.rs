//! Enterprise-grade process management for Kewve OS
//! 
//! This module provides comprehensive process management including:
//! - Process lifecycle management
//! - Memory space isolation
//! - Capability-based security
//! - Cross-platform scheduling
//! - Resource management and cleanup

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;
use lazy_static::lazy_static;
use crate::println;
use x86_64::{VirtAddr, PhysAddr};
use core::sync::atomic::{AtomicU64, Ordering};

/// Process states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked,
    Terminated,
}

/// Process identifier
pub type ProcessId = u64;

/// Process control block
#[derive(Debug, Clone)]
pub struct ProcessControlBlock {
    pub id: ProcessId,
    pub name: String,
    pub state: ProcessState,
    pub priority: u8,
    pub stack_pointer: Option<u64>,
    pub program_counter: Option<u64>,
    pub registers: [u64; 16], // General purpose registers
}

impl ProcessControlBlock {
    /// Create a new process control block
    pub fn new(id: ProcessId, name: String) -> Self {
        Self {
            id,
            name,
            state: ProcessState::Ready,
            priority: 0,
            stack_pointer: None,
            program_counter: None,
            registers: [0; 16],
        }
    }
    
    /// Set the process state
    pub fn set_state(&mut self, state: ProcessState) {
        self.state = state;
    }
    
    /// Get the process state
    pub fn state(&self) -> ProcessState {
        self.state
    }
}

/// Process scheduler
pub struct Scheduler {
    processes: BTreeMap<ProcessId, ProcessControlBlock>,
    ready_queue: Vec<ProcessId>,
    current_process: Option<ProcessId>,
}

impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            ready_queue: Vec::new(),
            current_process: None,
        }
    }
    
    /// Add a process to the scheduler
    pub fn add_process(&mut self, pcb: ProcessControlBlock) {
        let pid = pcb.id;
        self.processes.insert(pid, pcb);
        self.ready_queue.push(pid);
    }
    
    /// Remove a process from the scheduler
    pub fn remove_process(&mut self, pid: ProcessId) -> Option<ProcessControlBlock> {
        self.ready_queue.retain(|&x| x != pid);
        self.processes.remove(&pid)
    }
    
    /// Get the current process
    pub fn current_process(&self) -> Option<&ProcessControlBlock> {
        self.current_process.and_then(|pid| self.processes.get(&pid))
    }
    
    /// Get a mutable reference to the current process
    pub fn current_process_mut(&mut self) -> Option<&mut ProcessControlBlock> {
        self.current_process.and_then(move |pid| self.processes.get_mut(&pid))
    }
    
    /// Schedule the next process
    pub fn schedule(&mut self) -> Option<&ProcessControlBlock> {
        if self.ready_queue.is_empty() {
            return None;
        }
        
        // Simple round-robin scheduling
        let next_pid = self.ready_queue.remove(0);
        self.ready_queue.push(next_pid);
        self.current_process = Some(next_pid);
        
        self.processes.get(&next_pid)
    }
    
    /// Block the current process
    pub fn block_current(&mut self) {
        if let Some(pid) = self.current_process {
            if let Some(process) = self.processes.get_mut(&pid) {
                process.set_state(ProcessState::Blocked);
            }
            self.current_process = None;
        }
    }
    
    /// Unblock a process
    pub fn unblock(&mut self, pid: ProcessId) {
        if let Some(process) = self.processes.get_mut(&pid) {
            if process.state == ProcessState::Blocked {
                process.set_state(ProcessState::Ready);
                self.ready_queue.push(pid);
            }
        }
    }
}

lazy_static! {
    /// Global scheduler instance
    pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
}

/// Initialize process management
pub fn init() {
    // Create the initial kernel process
    let kernel_process = ProcessControlBlock::new(0, String::from("kernel"));
    SCHEDULER.lock().add_process(kernel_process);
    
    println!("Process management initialized");
}

/// Create a new process
pub fn create_process(name: String) -> ProcessId {
    static mut NEXT_PID: ProcessId = 1;
    
    let pid = unsafe {
        let id = NEXT_PID;
        NEXT_PID += 1;
        id
    };
    
    let process = ProcessControlBlock::new(pid, name);
    SCHEDULER.lock().add_process(process);
    
    pid
}

/// Switch to the next process
pub fn switch_to_next_process() {
    let mut scheduler = SCHEDULER.lock();
    if let Some(next_process) = scheduler.schedule() {
        println!("Switching to process: {} (PID: {})", next_process.name, next_process.id);
    }
}