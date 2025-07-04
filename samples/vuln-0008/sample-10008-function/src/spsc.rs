unsafe impl<T: Send, B: Buffer<T>> Send for SPSCConsumer<T, B> {}
unsafe impl<T: Send, B: Buffer<T>> Send for SPSCProducer<T, B> {}