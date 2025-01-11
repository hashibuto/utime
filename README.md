# utime
pronounced (micro time), utime provides a minimal compliment to the standard library time tools including basic string formatting.  utime is non monotonic and should not be used to calculate durations between successive timestamps, as said timestamps may potentially be presented out of order due to time synchronization etc.

## limitations
- only deals with times starting from unix epoch
- utc only, no time zones
- non monotonic

## recommended usage
- logging
- date/time string formatting