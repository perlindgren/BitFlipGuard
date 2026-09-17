# BitFlipGuard

Memory bit-flips may cause arbitrary program behavior with detrimental effect to system wide safety and security. In this work we leverage the Rust type system to provide abstractions to fault detection, correction and self healing. In particular, fault detection is achieved by a secondary backing store, optionally exploiting spatial displacement to reduce the risk of correlated errors. We approach error correction by triple redundancy, performing CPU efficient majority voting, with write back to heal single faults. The presented solution complements hardware protection such as shielding and ECC, and is fully scalable, such to reduce risk of corruption of critical data, both regarding incidental environment effects as well as protection against malicious actors.

