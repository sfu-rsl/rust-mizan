unsafe impl<T: Send, B: Buffer<T>> Send for MPSCConsumer<T, B> {}
unsafe impl<T: Send, B: Buffer<T>> Send for MPSCProducer<T, B> {}
unsafe impl<T: Send, B: Buffer<T>> Sync for MPSCProducer<T, B> {}