const MIN_PID: u16 = 100;
const MAX_PID: u16 = 1000;

use crate::list;

pub struct PidManager {
    cache: list::List<u16>,
    map: Vec<u16>,
    last_set_bit: u16,
}

impl PidManager {
    pub fn new() -> Self {
        PidManager {
            cache: list::List::new(),
            map: vec![0; (MAX_PID - MIN_PID + 1) as usize / std::mem::size_of::<u16>()],
            last_set_bit: 0,
        }
    }

    pub fn allocate_pid(&mut self) -> Result<u16, String> {
        let mut bit = self.last_set_bit + 1;
        let mut cached = false;

        if let Some(pid) = self.cache.pop() {
            bit = pid;
            cached = true;
        }

        if bit == (MAX_PID - MIN_PID) + 1 {
            Err(String::from(
                "PID map exhausted cannot spawn any new process",
            ))
        } else {
            self.map[bit as usize / std::mem::size_of::<u16>()] |=
                1 << (bit as usize % std::mem::size_of::<u16>());
            if !cached {
                self.last_set_bit = bit;
            }
            Ok(MIN_PID + bit)
        }
    }

    pub fn release_pid(&mut self, pid: u16) {
        let bit = pid - MIN_PID;

        self.map[bit as usize / std::mem::size_of::<u16>()] &=
            !(1 << (bit as usize % std::mem::size_of::<u16>()));
        self.cache.push(bit);
    }
}
