# Transaction system

## Running

```bash
# Example usage
cargo run -- transactions.csv > accounts.csv
```

Using one of the supplied test files, for easier copy/pasting

```bash
cargo run -- data/1_simple.test.csv > out.txt
```

If no argument is passed, the program will read from stdin instead.

## Code Overview

The main interaction with the library comes in the form of 3 structs: `Shard`, `Event`, and
`Summary`.

- `Event`s are a tagged enum containing the data for any of the transactional events that the system
  tracks: `Deposit`, `Withdrawal`, `Dispute`, `Resolve`, and `Chargeback`. Additional events can be
  added in the future if the need arises.
- The `Summary` struct contains a summary of a client account's current info. Primarily, the amount
  it holds and whether the account is locked.
- The `Shard` struct is where all the magic happens, managing events as they come in.

### Shard

The idea here is that there are multiple shards that work concurrently with each other. Each shard
will strive for eventual consistency, but may not be perfectly up-to-date at any given moment.

This would allow for running the system in multiple locations for lower latency, or for running
multiple shards on a single machine in order to avoid any downtime.

For example, say we have 4 shards running in parallel on a single machine. When a packet arrives, on
a TCP port or by some other means, it is delegated to one of the 4 running instances. Every so
often, pairs of shards will reconcile with each other, always maintaining 2 shards are up at all
times. (Note: The reconciliation code for the shards is not implemented at this time.)

NOTE: Because of the use of sharding, the assumption that specific events will come in a known order
(eg: Deposit -> Dispute -> Chargeback) has to be thrown out. Transactions that are flagged as being
disputed prior to receiving the amount of the deposit, will be pre-emptively flagged, and will be
properly handled when all information arrives. This may mean lead to unexpected results if you are
expecting early dispute events to be ignored.

### Data Management

Currently, the data is all stored in-memory, in a hash map. In the future, this would be changed to
offload old transactions and client accounts to storage. This would likely use a least recently used
(LRU) cache to keep track of the oldest transactions and evict them to disk to free up space in RAM
for new events that come in.

### Error Handling

First and foremost, the application should not panic. Ever. Any event that would cause an error is
stored and (currently) written to stderr. The event that caused the error is subsequently ignored.
This includes any event that would cause integer overflow.

Ideally this would instead send a notification or email somewhere (or even better send the event to
a Pub/Sub so that it can be listened for by other systems), but that is again beyond the scope of
this small project. In any form, the idea is that this should allow for the event to be manually so
that it can be manually handled later if needed.

### Folder Layout

- `src` contains the rust source files. Pretty standard.
- `data` stores a set of sample test files. The file names should be formated like
  `<test_name>.test.csv` and `<test_name>.want.csv` (making sure that the file names match). These
  file names correspend to tests written at the bottom of `src/main.rs`. The file name starting with
  `##_` is not strictly required, but is nice that it sort of keeps them relatively organized.
- `deps` has a local dependency, specifically `csv_test_proc` which is a custom library that makes
  it easy to define a test that reads the test data `*.csv` file, as mentioned in the point above.
