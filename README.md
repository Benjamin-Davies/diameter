# Diameter
A tool for working with [ChordPro](https://en.wikipedia.org/wiki/ChordPro) files. This tool supports transposing, conversion to and from [numbers](https://en.wikipedia.org/wiki/Nashville_Number_System), and printing to PDF. It can also be used as a Rust library.

It is called Diameter because a diameter is the [longest chord of a circle](https://en.wikipedia.org/wiki/Diameter).

## Non-standard extensions
This tool supports the ["chords above"](https://help.elvanto.com/hc/en-us/articles/7607782493463-How-to-Add-Chords-to-Lyrics#chords-above) format used by Elvanto. For example:

```
 G      G/B       Cadd9     G
Amazing grace how sweet the sound
```

Use the `-x` flag if the input file may use this format. Use the `-v` flag to output in the "chords above" format.
