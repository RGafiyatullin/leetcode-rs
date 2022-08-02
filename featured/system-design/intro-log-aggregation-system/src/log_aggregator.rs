#[derive(Debug, Clone)]
pub struct LogAggregator {
    machines: Vec<Vec<usize>>,
    services: Vec<Vec<usize>>,

    entries: Vec<Entry>,
}

#[derive(Debug, Clone)]
struct Entry {
    log_id: i32,
    // machine_id: i32,
    // service_id: i32,
    message: String,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LogAggregator {
    pub fn new(machines: i32, services: i32) -> Self {
        Self {
            machines: vec![vec![]; machines as usize],
            services: vec![vec![]; services as usize],

            entries: vec![],
        }
    }

    pub fn push_log(&mut self, log_id: i32, machine_id: i32, service_id: i32, message: String) {
        let entry = Entry {
            log_id,
            /*machine_id, service_id,*/ message,
        };
        let entry_id = self.entries.len();
        self.entries.push(entry);

        self.machines[machine_id as usize].push(entry_id);
        self.services[service_id as usize].push(entry_id);
    }

    pub fn get_logs_from_machine(&self, machine_id: i32) -> Vec<i32> {
        self.machines[machine_id as usize]
            .iter()
            .copied()
            .map(|idx| self.entries[idx].log_id)
            .collect()
    }

    pub fn get_logs_of_service(&self, service_id: i32) -> Vec<i32> {
        self.services[service_id as usize]
            .iter()
            .copied()
            .map(|idx| self.entries[idx].log_id)
            .collect()
    }

    pub fn search(&self, service_id: i32, search_string: String) -> Vec<String> {
        self.services[service_id as usize]
            .iter()
            .copied()
            .filter_map(|idx| {
                let entry = &self.entries[idx];
                if entry.message.find(search_string.as_str()).is_some() {
                    Some(entry.message.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
}
