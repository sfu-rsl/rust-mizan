# Vulnerability: CVE-2020-35925 	

| **Information**       | **Details**                                                                                |
| --------------------- | ------------------------------------------------------------------------------------------ |
| **CVE**               | [CVE-2020-35925](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35925)                    |
| **Vulnerable Commit** | [d7e3d58](https://github.com/johnshaw/magnetic/tree/d7e3d58c65bf4f9326f7aaf2b2c75563e721918b) |
| **Fixed Commit**  | [0748444](https://github.com/johnshaw/magnetic/tree/074844463245727ae9c4192f7d0a6ec0a4ae6feb) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                               |
|                       | - [fixed-file](fixed-file)                                                                 |
|                       | - [fixed-function](fixed-function)                                                         |
|                       | - [vuln-crate](vuln-crate)                                                                 |
|                       | - [vuln-file](vuln-file)                                                                   |
|                       | - [vuln-function](vuln-function)                                                           |

 ## Vulnerable Lines:
 ```rust
// The vulnerability lies in concurrrently accessible queues being able to hold non thread safe items
//
// T doesn't have to implement the Send trait while the Produces and Consumder do
//
// UB is possiblin Safe Rust if something like an RC is used

// mulitple producer, multiple consumer queue
unsafe impl<T, B: Buffer<T>> Send for MPMCConsumer<T, B> {}
unsafe impl<T, B: Buffer<T>> Sync for MPMCConsumer<T, B> {}

unsafe impl<T, B: Buffer<T>> Send for MPMCProducer<T, B> {}
unsafe impl<T, B: Buffer<T>> Sync for MPMCProducer<T, B> {}

// mulitple producer, singular consumer queue
unsafe impl<T, B: Buffer<T>> Send for MPSCConsumer<T, B> {}

unsafe impl<T, B: Buffer<T>> Send for MPSCProducer<T, B> {}
unsafe impl<T, B: Buffer<T>> Sync for MPSCProducer<T, B> {}

// singular producer, multiple consumer queue
unsafe impl<T, B: Buffer<T>> Send for SPMCConsumer<T, B> {}
unsafe impl<T, B: Buffer<T>> Sync for SPMCConsumer<T, B> {}

unsafe impl<T, B: Buffer<T>> Send for SPMCProducer<T, B> {}
unsafe impl<T, B: Buffer<T>> Sync for SPMCProducer<T, B> {}

// singular producer, singular consumer queue
unsafe impl<T, B: Buffer<T>> Send for SPSCConsumer<T, B> {}

unsafe impl<T, B: Buffer<T>> Send for SPSCProducer<T, B> {}

 ```   
