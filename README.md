# q_boilerplate

Cli too for creating boilerplate for Qlik Sense Extensions. Templates use Webpack, Typescript, React, styled-components, jest (testing), and storybook (ui-prototyping).

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
