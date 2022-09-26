```bash
# xc

USAGE:
    xc [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    conn      Connect can be used to remotely connect CPE and display the process on the terminal.
    help      Prints this message or the help of the given subcommand(s)
    list      Use list to obtain multiple CPE information and display it on the current terminal
    show      Use show to obtain CPE information and display it on the current terminal
    update    Use update to update local CPE information


USAGE:
    xc show [OPTIONS] <sn>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m <mode>        Use show to select the CPE, the default version is nexus. [possible values: nexus, valor, watsons,
                     watsons_ha, tassadar]

ARGS:
    <sn>    cpe serial number


USAGE:
    xc list [OPTIONS] <sn>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --mode <Mode>     [possible values: valor, nexus, watsons, tassadar, watsons_ha]

ARGS:
    <sn>...    cpe serial number
```
