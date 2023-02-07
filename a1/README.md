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


1. Question 1 can be seen in `rust/examples/simple.rs`.
2. Question 2 can be seen in `rust/examples/bicycle_built_for_two.rs`.
3. Question 3 can be seen in `rust/src/main.rs`.
4. Question 4 can be seen in `rust/src/wave.rs` and `rust/src/main.rs`.
5. Question 5 can be seen in `rust/src/wavetable.rs`.
6. Question 6 can also be seen in `rust/src/main.rs`.
7. Question 7 can be seen in the `puredata` directory.
9. For question 9, the unfamiliar language used was Rust.
    - As discussed in class it's fine that I didn't also implement 1 through 6 in 
      a familiar language.
10. Question 10 can be seen in `rust/src/sin.rs` and `rust/benches/sin_compare.rs`.
    - It may seem odd that I'm not iterating over a bunch of data-points when benching
      these sin functions, but `criterion` is proven to be accurate on even short 
      functions.