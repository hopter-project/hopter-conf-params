#![no_std]
/* ############################ */
/* ### Stack Configurations ### */
/* ############################ */

/// The boundary of the contiguous stack that its top should not grow beyond.
pub const CONTIGUOUS_STACK_BOUNDARY: u32 = 0x2000_0010;

/// The bottom of the congituous stack.
pub const CONTIGUOUS_STACK_BOTTOM: u32 = 0x2000_1000;

/// Whether dynamic extension of the stack is allowed.
pub const ALLOW_DYNAMIC_STACK: bool = true;

/// The address in memory where the active stacklet boundary is stored.
pub const STACKLET_BOUNDARY_MEM_ADDR: u32 = 0x2000_0000;

/// The extra size added to a stacklet allocation request in addition to the
/// allocation size requested by the function prologue.
pub const STACKLET_ADDITION_ALLOC_SIZE: usize = 64;

/// The stack size of the idle task when it is just created. If
/// [`ALLOW_DYNAMIC_STACK`] is set to true, this value can be kept to 0 so
/// that the stack will be allocated completely dynamically.
pub const IDLE_TASK_INITIAL_STACK_SIZE: usize = 0;

/// The stack size of the main task when it is just created. If
/// [`ALLOW_DYNAMIC_STACK`] is set to true, this value can be kept to 0 so
/// that the stack will be allocated completely dynamically.
pub const MAIN_TASK_INITIAL_STACK_SIZE: usize = 0;

/* ########################### */
/* ### Heap Configurations ### */
/* ########################### */

/// The ending address of the heap region.
pub const HEAP_END: u32 = 0x2002_0000;

/// Free memory chunks use 16-bit links to form linked lists. Since memory
/// chunks are 4-byte aligned, the linkes can represent a range of 2^18 bytes.
/// The represented range is
/// `[MEM_CHUNK_LINK_OFFSET, MEM_CHUNK_LINK_OFFSET + 2^18)`.
pub const MEM_CHUNK_LINK_OFFSET: u32 = 0x2000_0000;

/* ################################ */
/* ### Interrupt Configurations ### */
/* ################################ */

/// The numerical stepping between two adjacent IRQ priority levels.
/// The interrupt priority registers on Cortex-M reserves 8-bits for each
/// interrupt, but for most MCU implementations only a few significant bits
/// are used. For example, if only the top 5 significant bits are used, then
/// the numerical granularity will be 8. If the top 4 significant bits are used,
/// then the numerical granularity will be 16. See Nested Vectored Interrupt
/// Controller (NVIC) in Cortex-M for details.
pub const IRQ_PRIORITY_GRANULARITY: u8 = 16;

/// The maximum priority of an interrupt. It has the smallest numerical value.
pub const IRQ_MAX_PRIORITY: u8 = 7 * IRQ_PRIORITY_GRANULARITY;

/// The higher priority of an interrupt. Defined for convenience.
pub const IRQ_HIGH_PRIORITY: u8 = 8 * IRQ_PRIORITY_GRANULARITY;

/// The normal priority of an interrupt. Defined for convenience.
pub const IRQ_NORMAL_PRIORITY: u8 = 9 * IRQ_PRIORITY_GRANULARITY;

/// The lower priority of an interrupt. Defined for convenience.
pub const IRQ_LOW_PRIORITY: u8 = 10 * IRQ_PRIORITY_GRANULARITY;

/// The minimum priority of an interrupt. It has the largest numerical value.
pub const IRQ_MIN_PRIORITY: u8 = 11 * IRQ_PRIORITY_GRANULARITY;

/// The priority of SysTick interrupt.
pub const SYSTICK_PRIORITY: u8 = IRQ_LOW_PRIORITY;

/// Hopter globally enables or disables interrupts by configuring the BASEPRI
/// register. IRQs with higher priority (smaller numerical value) than BASEPRI
/// are enabled. This value should be set to be lower than all IRQ priority
/// levels.
pub const IRQ_ENABLE_BASEPRI_PRIORITY: u8 = 14 * IRQ_PRIORITY_GRANULARITY;

/// Hopter globally enables or disables interrupts by configuring the BASEPRI
/// register. IRQs with lower or equal priority (greater or qeual numerical
/// value) than BASEPRI are disabled. This value should be set to be higher or
/// equal than all IRQ priority levels.
pub const IRQ_DISABLE_BASEPRI_PRIORITY: u8 = 6 * IRQ_PRIORITY_GRANULARITY;

/// When the interrupt is not globally masked, i.e. the normal case, the SVC
/// is set to a priority lower than all IRQs, so that IRQs can nest above an
/// active SVC and get served promptly.
pub const SVC_NORMAL_PRIORITY: u8 = 12 * IRQ_PRIORITY_GRANULARITY;

/// When the interrupt is globally masked, SVC still need to be allowed because
/// growing segmented stacks depend on it. During the period that the interrupt
/// is globally masked, the priority of SVC is raised to keep it higher than
/// BASEPRI.
pub const SVC_RAISED_PRIORITY: u8 = 0 * IRQ_PRIORITY_GRANULARITY;

/// PendSV is used to implement context switch. Since an SVC may tail chain a
/// PendSV to perform context switch, PendSV's priority must be lower than SVC.
pub const PENDSV_PRIORITY: u8 = 13 * IRQ_PRIORITY_GRANULARITY;

/* ########################### */
/* ### Task Configurations ### */
/* ########################### */

/// The maximum number of tasks. Must be a power of 2.
pub const MAX_TASK_NUMBER: usize = 16;

/// Whether a ready higher priority task should cause a lower priority running
/// task to yield.
pub const ALLOW_TASK_PREEMPTION: bool = true;

/// Maximum priority number. Lower numerical numbers represent higher priorities.
/// Allowed priority range: 0 to (TASK_PRIORITY_LEVELS - 1).
pub const TASK_PRIORITY_LEVELS: u8 = 16;

/// The priority of the idle task. Typically this is set to the lowest allowed
/// priority.
pub const IDLE_TASK_PRIORITY: u8 = TASK_PRIORITY_LEVELS - 1;

/// The task priority of the main task.
pub const MAIN_TASK_PRIORITY: u8 = 0;

/// A panicked task will get its priority reduced to the unwind priority,
/// which is very low but still higher than idle priority.
pub const UNWIND_PRIORITY: u8 = TASK_PRIORITY_LEVELS - 3;
