
#[derive(Clone, Copy)]
pub struct Task {
    pub id: u32,
}

const QUEUE_SIZE: usize = 8;

pub struct ReadyQueue {
    tasks: [Option<Task>; QUEUE_SIZE],
    head: usize,
    tail: usize,
    count: usize,
}

impl ReadyQueue {
    pub const fn new() -> Self {
        Self {
            tasks: [None; QUEUE_SIZE],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn push(&mut self, task: Task) -> bool {
        if self.count == QUEUE_SIZE {
            return false;
        }

        self.tasks[self.tail] = Some(task);
        self.tail = (self.tail + 1) % QUEUE_SIZE;
        self.count += 1;
        true
    }

    pub fn pop(&mut self) -> Option<Task> {
        if self.count == 0 {
            return None;
        }

        let task = self.tasks[self.head].take();
        self.head = (self.head + 1) % QUEUE_SIZE;
        self.count -= 1;
        task
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

pub fn round_robin(queue: &mut ReadyQueue) -> Option<Task> {
    let task = queue.pop()?;
    queue.push(task);
    Some(task)
}
