unsafe impl<T: Send, B: Buffer<T>> Send for MPMCConsumer<T, B> {}
unsafe impl<T: Send, B: Buffer<T>> Sync for MPMCConsumer<T, B> {}
unsafe impl<T: Send, B: Buffer<T>> Send for MPMCProducer<T, B> {}
unsafe impl<T: Send, B: Buffer<T>> Sync for MPMCProducer<T, B> {}