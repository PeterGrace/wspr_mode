# what
sets the appropriate wspr settings for the BG6JJI wspr beacon
```
Usage: wspr_mode [OPTIONS] <PORT> <CALLSIGN>

Arguments:
  <PORT>
  <CALLSIGN>

Options:
  -g, --grid <GRID>
  -p, --power <POWER>
  -b, --band <BAND>
  -h, --help           Print help
```

  - if grid is left empty, the device auto-calculates it via gps coordinates.
  - if power is left empty, 23 is assumed (the proper setting for the beacon hardware)
  - if band is left empty, band is automatically calculated via K1JT's suggested schedule:
```
Band (m)   160  80  60  40  30  20  17  15  12  10
UTC Minute  00  02  04  06  08  10  12  14  16  18
            20  22  24  26  28  30  32  34  36  38
            40  42  44  46  48  50  52  54  56  58
```
