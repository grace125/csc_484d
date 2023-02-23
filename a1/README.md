# Assignment Requirements and How to Evaluate them:

- [x] (1) Play 3 seconds of a 440Hz sinusoid at 44100 samples per second.
- [x] (2) Create a function that takes a MIDI note, and generates a 
      corresponding midi sound
    - Use this to create a recognizable melody.
- [x] (3) Generalize to take in a simple score language which specifies 
      pitches and duration
- [x] (4) Implement Sawtooth, Pulse, Triangle and Noise oscillators.
- [x] (5) Store your waves in a buffer with a user-provided buffer.
- [x] (6) Make your synth polyphonic with multiple oscillators.
- [x] (7) Implement (1) and (2) in PureData
- [x] (9) Implement 1 through 6 in an unfamiliar language
- [x] (10) Implement and benchmark 3 versions of the sin function

<br>

- For (1), see `rust/examples/simple.rs`.
- For (2), see `rust/examples/bicycle_built_for_two.rs`.

See `rust/src/main.rs` and `rust/assets/polyphony.ron` for question 3.
I decided to use the ron file format because it's clean and easy.
Run with `cargo run some/input/file.ron`.

- For (4), see `rust/src/wave.rs` and `rust/src/main.rs`. 
- For (5), see `rust/src/wavetable.rs`.
- For (6), see `rust/src/main.rs`.
- For (7), see `puredata` directory.

For question 9, the unfamiliar language used was Rust.
As discussed in class it's fine that I didn't also implement 1 through 6 in a familiar language.

Question 10 can be seen in `rust/src/sin.rs` and `rust/benches/sin_compare.rs`.
It may seem odd that I'm benchmarking over one sample of sin,
but `criterion` is proven to be accurate for benchmarking on even short functions. 

Do reach out if any of my code needs more clarification.
