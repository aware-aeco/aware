# texture-evaluator-initialize

Lifecycle: single

Call this function before calling GetColor for the first time. Ideally, this should             be on the main thread, but you can also call it on a worker thread as long as you             are sure that Initialize() or GetColor() cannot be called at the same time on another thread.
