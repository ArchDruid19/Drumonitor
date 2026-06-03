# Drumonitor
This is meant to be used alongside [tint2](https://github.com/semplice/tint2), and provides information on CPU, RAM, and network usage.

## Commands And Options
The general syntax to run a hardware monitor is: `drumonitor [COMMAND] [OPTIONS]`

### CPU
`drumonitor cpu [OPTIONS]`

`-p` - Shows how much load is on the CPU as a percentage.

___
### RAM
`drumonitor ram [OPTIONS]`

`-p` - Shows the total memory used as a percentage

`-u` - Shows the total memory used and the total memory available in bytes

___
### Network
`drumonitor network [OPTIONS]`

*Note*: This monitor only works for devices connected via ethernet.

`-t` - Shows upload and download speed

___
### Universal Options
`-i` - Sets the time (in seconds) it takes for a stat to be updated. The default value is 5.

___
### Help
`--help` - Shows the options for the given subcommand. To see the help menu for a specific command, you can do `drumonitor [COMMAND] --help`. To see all commands, you can do `drumonitor --help`.
