# q_boilerplate

Cli too for creating boilerplate for Qlik Sense Extensions. Templates use Webpack, Typescript, React, styled-components, jest (testing), and storybook (ui-prototyping).

# Usage

Either clone this repo and build and install the cli using `cargo install`, or download the pre-built binary in the releases. At the moment there are only pre-built binaries for Windows because I have no yet setup a CI and most users of this cli will be windows users.

```
q_boilerplate 0.1.0
Michael Rutter <michael.john.rutter@gmail.com>
Generates boilerplate code for Qlik Extensions

USAGE:
    q_boilerplate.exe <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help             Prints this message or the help of the given subcommand(s)
    mashup           Creates mashup extension template
    visualisation    Creates Visualisation extension template
```

## visualisation subcommand
Run this sub command within the Qlik Sense Extension directory to create a new visualisation extension.

```
Creates Visualisation extension template

USAGE:
    q_boilerplate.exe visualisation [FLAGS] <name>

FLAGS:
    -h, --help       Prints help information
        --no-git     Opt out of creating a git repo for the new extension
    -V, --version    Prints version information

ARGS:
    <name>    Project name
```
