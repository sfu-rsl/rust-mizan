# Vulnerability: CVE-2020-35925 	

| **Information**       | **Details**                                                                                |
| --------------------- | ------------------------------------------------------------------------------------------ |
| **CVE**               | [CVE-2020-35925](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35925)                    |
| **Vulnerable Commit** | [f6193fe](https://github.com/johnshaw/magnetic/tree/f6193feee18c103e130e44f036df9ef9de994a55) |
| **Fixed Commit**  | [0748444](https://github.com/johnshaw/magnetic/tree/074844463245727ae9c4192f7d0a6ec0a4ae6feb) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                               |
|                       | - [fixed-file](fixed-file)                                                                 |
|                       | - [fixed-function](fixed-function)                                                         |
|                       | - [vuln-crate](vuln-crate)                                                                 |
|                       | - [vuln-file](vuln-file)                                                                   |
|                       | - [vuln-function](vuln-function)                                                           |

 ## Vulnerable Lines:
 ```rust
// Consumer end of an mpmc queue.
pub struct MPMCConsumer<T, B: Buffer<T>> {
    queue: Arc<UnsafeCell<MPMCQueue<T, B>>>,
}

// T doesn't have to implement the Send trait while the Consumer does
// UB is possiblin Safe Rust if something like an RC is used
unsafe impl<T, B: Buffer<T>> Send for MPMCConsumer<T, B> {}

/// Producer end of the queue for completeness. 
pub struct MPMCProducer<T, B: Buffer<T>> {
    queue: Arc<UnsafeCell<MPMCQueue<T, B>>>,
}

unsafe impl<T, B: Buffer<T>> Send for MPMCProducer<T, B> {}
 ```   