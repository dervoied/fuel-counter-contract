contract;

abi TestContract {
    #[storage(read)]
    fn get_counter () -> u64;

    #[storage(read, write)]
    fn increment_counter(amount: u64) -> u64;

}

storage {
    counter: u64 = 0,
}

impl TestContract for Contract {
     #[storage(read)]
    fn get_counter () -> u64 {
        return storage.counter.read();
    }

    #[storage(read, write)]
    fn increment_counter(amount: u64) -> u64 {
        let incremented = storage.counter.read() + amount;
        storage.counter.write(incremented);
        return incremented;
    }
}
