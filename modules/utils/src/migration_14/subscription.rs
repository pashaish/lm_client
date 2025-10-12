
    use std::hash::Hash;
    
    use iced::{
        Subscription,
        advanced::{
            graphics::futures::MaybeSend,
            subscription::{EventStream, Hasher, Recipe, from_recipe},
        },
        futures::{self, Stream, stream::BoxStream},
    };

    pub fn run_with_id<I, S, T>(id: I, stream: S) -> Subscription<T>
    where
        I: Hash + 'static,
        S: Stream<Item = T> + MaybeSend + 'static,
        T: 'static,
    {
        from_recipe(Runner {
            id,
            spawn: move |_| stream,
        })
    }

    struct Runner<I, F, S, T>
    where
        F: FnOnce(EventStream) -> S,
        S: Stream<Item = T>,
    {
        id: I,
        spawn: F,
    }

    impl<I, F, S, T> Recipe for Runner<I, F, S, T>
    where
        I: Hash + 'static,
        F: FnOnce(EventStream) -> S,
        S: Stream<Item = T> + MaybeSend + 'static,
    {
        type Output = T;

        fn hash(&self, state: &mut Hasher) {
            std::any::TypeId::of::<I>().hash(state);
            self.id.hash(state);
        }

        // TODO: MUST BE STATIC?
        fn stream(self: Box<Self>, input: EventStream) -> BoxStream<'static, Self::Output> {
            boxed_stream((self.spawn)(input))
        }
    }

    fn boxed_stream<'a, T, S>(stream: S) -> BoxStream<'a, T>
    where
        S: futures::Stream<Item = T> + Send + 'static,
    {
        futures::stream::StreamExt::boxed(stream)
    }


