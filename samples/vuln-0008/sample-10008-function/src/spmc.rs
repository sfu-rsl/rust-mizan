unsafe impl<T: Send, B: Buffer<T>> Send for SPMCConsumer<T, B> {}
unsafe impl<T: Send, B: Buffer<T>> Sync for SPMCConsumer<T, B> {}
unsafe impl<T: Send, B: Buffer<T>> Send for SPMCProducer<T, B> {}
unsafe impl<T: Send, B: Buffer<T>> Sync for SPMCProducer<T, B> {}