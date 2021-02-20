pub(crate) mod list;
pub mod pid;

#[cfg(test)]
mod tests {
    use super::pid::PidManager;

    #[test]
    fn test_allocate_used() {
        let mut pid_manager = PidManager::new();
        let pid1 = pid_manager.allocate_pid().unwrap();
        pid_manager.release_pid(pid1);
        let pid2 = pid_manager.allocate_pid().unwrap();
        assert_eq!(pid1, pid2);
    }
}
