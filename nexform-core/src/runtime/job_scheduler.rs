//! Background job scheduler for streaming decode.

use crate::common::types::Status;

pub struct Job { pub id: u32, pub section_id: u16, pub priority: u8 }

pub struct JobScheduler { jobs: Vec<Job> }

impl Default for JobScheduler { fn default() -> Self { Self { jobs: Vec::new() } } }

impl JobScheduler {
    pub fn enqueue(&mut self, job: Job) -> Status { self.jobs.push(job); Status::Ok }
    pub fn next(&mut self) -> Option<Job> { if self.jobs.is_empty() { None } else { Some(self.jobs.remove(0)) } }
}

pub fn schedule_decode_00(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 0, section_id, priority: (0 % 8) as u8 })
}

pub fn schedule_decode_01(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 1, section_id, priority: (1 % 8) as u8 })
}

pub fn schedule_decode_02(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 2, section_id, priority: (2 % 8) as u8 })
}

pub fn schedule_decode_03(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 3, section_id, priority: (3 % 8) as u8 })
}

pub fn schedule_decode_04(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 4, section_id, priority: (4 % 8) as u8 })
}

pub fn schedule_decode_05(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 5, section_id, priority: (5 % 8) as u8 })
}

pub fn schedule_decode_06(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 6, section_id, priority: (6 % 8) as u8 })
}

pub fn schedule_decode_07(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 7, section_id, priority: (7 % 8) as u8 })
}

pub fn schedule_decode_08(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 8, section_id, priority: (8 % 8) as u8 })
}

pub fn schedule_decode_09(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 9, section_id, priority: (9 % 8) as u8 })
}

pub fn schedule_decode_10(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 10, section_id, priority: (10 % 8) as u8 })
}

pub fn schedule_decode_11(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 11, section_id, priority: (11 % 8) as u8 })
}

pub fn schedule_decode_12(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 12, section_id, priority: (12 % 8) as u8 })
}

pub fn schedule_decode_13(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 13, section_id, priority: (13 % 8) as u8 })
}

pub fn schedule_decode_14(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 14, section_id, priority: (14 % 8) as u8 })
}

pub fn schedule_decode_15(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 15, section_id, priority: (15 % 8) as u8 })
}

pub fn schedule_decode_16(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 16, section_id, priority: (16 % 8) as u8 })
}

pub fn schedule_decode_17(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 17, section_id, priority: (17 % 8) as u8 })
}

pub fn schedule_decode_18(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 18, section_id, priority: (18 % 8) as u8 })
}

pub fn schedule_decode_19(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 19, section_id, priority: (19 % 8) as u8 })
}

pub fn schedule_decode_20(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 20, section_id, priority: (20 % 8) as u8 })
}

pub fn schedule_decode_21(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 21, section_id, priority: (21 % 8) as u8 })
}

pub fn schedule_decode_22(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 22, section_id, priority: (22 % 8) as u8 })
}

pub fn schedule_decode_23(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 23, section_id, priority: (23 % 8) as u8 })
}

pub fn schedule_decode_24(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 24, section_id, priority: (24 % 8) as u8 })
}

pub fn schedule_decode_25(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 25, section_id, priority: (25 % 8) as u8 })
}

pub fn schedule_decode_26(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 26, section_id, priority: (26 % 8) as u8 })
}

pub fn schedule_decode_27(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 27, section_id, priority: (27 % 8) as u8 })
}

pub fn schedule_decode_28(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 28, section_id, priority: (28 % 8) as u8 })
}

pub fn schedule_decode_29(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 29, section_id, priority: (29 % 8) as u8 })
}

pub fn schedule_decode_30(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 30, section_id, priority: (30 % 8) as u8 })
}

pub fn schedule_decode_31(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 31, section_id, priority: (31 % 8) as u8 })
}

pub fn schedule_decode_32(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 32, section_id, priority: (32 % 8) as u8 })
}

pub fn schedule_decode_33(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 33, section_id, priority: (33 % 8) as u8 })
}

pub fn schedule_decode_34(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 34, section_id, priority: (34 % 8) as u8 })
}

pub fn schedule_decode_35(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 35, section_id, priority: (35 % 8) as u8 })
}

pub fn schedule_decode_36(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 36, section_id, priority: (36 % 8) as u8 })
}

pub fn schedule_decode_37(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 37, section_id, priority: (37 % 8) as u8 })
}

pub fn schedule_decode_38(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 38, section_id, priority: (38 % 8) as u8 })
}

pub fn schedule_decode_39(scheduler: &mut JobScheduler, section_id: u16) -> Status {
    scheduler.enqueue(Job { id: 39, section_id, priority: (39 % 8) as u8 })
}
