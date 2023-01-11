# dRUmSTer

CLI Drum Machine

## Features

- [x] Parse CSV
- [x] Play Beat
- [x] csv parsing more resilient with warnings which fields are malformed (missing fields, wrong type)
- [ ] Awesome visualization of playing beat -> checkout [crossterm](https://github.com/crossterm-rs/crossterm)
- [ ] Edit drum machine in CLI
- [ ] Export to sound file
- [ ] allow to play triplets | can be highly difficult
- [ ] big ui with [tui](https://github.com/fdehau/tui-rs)

## CSV

### Format

```
bpm;beats;base;resolution;(instrument1;instrument2;...)
beats of instrument 1 in resolution unit
```

- bpm - Beats per minute
- beats - amount of beats in each tact
- base - base note value for beats
- resolution - tiniest value for beats
- list of instruments
- each following line has strokes for each instrument in order of appearance

beats and base build the time signature (beats/base -> 4/4, 6/8, etc)
bpm says how many beats of the base note value
resolution expresses on which note value the drum machine is programmed.

The fields base and resolution require a note value

| note value         | value |
|--------------------|-------|
| whole note         | 1     |
| half note          | 2     |
| quarter note       | 4     |
| eighth note        | 8     |
| sixteenth note     | 16    |
| thirty-second note | 32    |
| sixty-fourth note  | 64    |

### Example

The beat is in 4/4 tact and has 80 bpm with the tiniest playable note value 16th.

The beat plays eighth on HiHat Snare on 2 and 4 and Kick on each hit.

```
80;4;4;16;HiHat;Snare;Kick
0;2;4;6;8;10;12;14
4;12
0;4;8;12
```

### CSV Validation
- bpm - 0 < bpm <= 300
- beats - 0 < beats <= 255 (as its from type u8 but reasonable max value would be around 20)
- base - note value
- resolution - note value
- instrument - Kick, Snare, HiHat, more to come
- instrument-strokes - 0 <= strokes < resolution * (beats / base)