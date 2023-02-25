


# Question 1

In the lab, I was with Cameron and two others when we connected a Keytaur to output to another MIDI keyboard. 
We did so via MIDI-IN and MIDI-OUT 6-pin cords. 
We were having trouble getting this to work, but our issue turned out to be a faulty power cord for the Keytaur.
There wasn't really any good indication whether the Keytaur was on or not. 

# Question 3

We also connected that keyboard to a laptop. 
That was much easier.
The main messages to take note of were the `MIDI_ON`, `MIDI_OFF`, and `PITCH_BEND` events. 
Each change to the pitch bend wheel send a unique event, so the screen tended to flood with those kinds of events whenever we touched the wheel.

# Question 9

If you're going to look at anything from this assignment, check out what I did for question 9 in the `step_sequencer.rs` example. 
I feel really proud of how it turned out. 

# Question 11

I couldn't manage to get my BiQuad filter working as well as I wanted it to. 
I'm not quite sure why at the moment. My code looks correct, but the results from the filter don't seem correct. 
I.e. when I give b0, b1, b2, a1, a2 values corresponding to a lowpass filter, it barely changes the sound. 
Judge what I have I suppose.

# Other Questions

- For q2, see the `complex_multiplication.rs` example
- For q4 and q5, see `amp_phase_estimation.rs`
- For q6, see `arpeggiator.rs`. 
    - It took so long to do this, because for some reason on Linux with qjackctrl and qsynth, sending a midi note on with maximum velocity makes no noise. Took me like 5 hours to debug.
- For q7, see `equal_temperament.rs`
- For q8, see `complex_clock_plot.rs`.
    - I couldn't actually figure out how to draw lines like it's a clock, so I just have the graph draw points where the ends of each of the clock hands would be.