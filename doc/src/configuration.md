# Configuration

Garden is configured through a YAML configuration file, typically called
"garden.yaml".

Garden will find `garden.yaml` in current directory or in specific locations
on the filesystem when unspecified.  Garden searches for `garden.yaml` in the
following locations. The first one found is used.

    # Relative to the current directory
    ./garden.yaml
    ./garden/garden.yaml
    ./etc/garden/garden.yaml

    # Relative to $HOME
    ~/.config/garden/garden.yaml
    ~/etc/garden/garden.yaml

    # Global configuration
    /etc/garden/garden.yaml


Use `garden -c|--config <filename>` to specify a garden file and override
garden's file discovery.

The following example `garden.yaml` is referred to by the documentation
when showing examples.

```yaml
{{#include garden.yaml}}
```

## Garden Root

The garden root directory is configured in the `garden.root` field.
This directory is the parent directory in which all trees will be cloned.

Slashes in tree paths will create new directories on disk as needed.
`garden.root` defaults to the current directory when unspecified.


The built-in `${GARDEN_CONFIG_DIR}` variable can be used to create relocatable
setups that define a `garden.root` relative to the garden file itself.

To place all trees in a `src` directory sibling to the `garden.yaml` file, the
following configuration can be used:

    garden:
      root: ${GARDEN_CONFIG_DIR}/src

To place all trees in a `src` directory in your `$HOME` directory, the
following configuration can be used:

    garden:
      root: ~/src


## Variables

Garden configuration contains a "variables" block that allows defining
variables that are can be referenced by other garden values.

    variables:
      flavor: debug
      user: $ whoami
      libdir: $ test -e /usr/lib64 && echo lib64 || echo lib
      nproc: $ nproc
      prefix: ~/.local
      py_ver_code: from sys import version_info as v; print("%s.%s" % v[:2])
      py_ver: $ python -c '${py_ver_code}'
      py_site: ${libdir}/python${py_ver}/site-packages

Variables definitions can reference environment variables and other garden
variables.

Variable references use shell `${variable}` syntax.

Values that start with dollar-sign+space (`$ `) are called "exec expressions".
Exec expressions are run through a shell after evaluation and replaced with
the output of the evaluated command.

When resolving values, variables defined in a tree scope override/replace
variables defined at the global scope.  Variables defined in garden scope
override/replace variables defined in a tree scope.


## Built-in variables

Garden automatically defines some built-in variables that can be useful
when constructing values for variables, commands, and paths.

    GARDEN_CONFIG_DIR   -   directory containing the "garden.yaml" config file
    GARDEN_ROOT         -   root directory for trees
    TREE_NAME           -   current tree name
    TREE_PATH           -   current tree path


## Environment Variables

The "environment" block defines variables that are stored in the environment.
Names with an equals sign (`=`) suffix are treated as "set" operations and
stored in the environment as-is.  Otherwise, the variable values are prepended
to using colons (`:`).  A plus sign (`+`) suffix in the name append to a
variable instead of prepending.

Environment variables are resolved in the same order as the garden variables:
global scope, tree scope, and garden scope.  This allows gardens to
prepend/append variables after a tree, thus allowing for customization
of behavior from the garden scope.

Environment variables are resolved after garden variables.  This allows
the use of garden variables when defining environment variable values.

Environment variable names can use garden `${variable}` syntax when defining
their name, for example,

    trees:
      foo:
        environment:
          ${TREE_NAME}_LOCATION=: ${TREE_PATH}

exports a variable called `foo_LOCATION` with the location of the `foo` tree.


### OS Environment Variables

OS-level environment variables that are present in garden's runtime
environment can be referenced using garden `${variable}` expression syntax.
Garden variables have higher precedence than environment variables when
resolving `${variable}` references -- the environment is checked only when
no garden variables exist by that name.


## Gardens, Groups and Trees

Trees are Git repositories with configuration that allows for the
specification of arbitrary commands and workflows.  Groups are a simple
named grouping mechanism.

Gardens aggregate groups and trees.  Define a group and reuse the group in
different gardens to share tree lists between gardens.  Defining gardens
and groups make those names available when querying and performing operations
over trees.

Gardens can also include environment, gitconfig, and custom group-level
commands in addition to the commands provided by each tree.


## Templates

Templates allow sharing of variables, gitconfig, and environments between
trees. Adding an entry to the "templates" configuration block and they can
then be referenced into trees using `templates: <template-name>`.

Trees can also reuse another tree definition by specifying the "extend"
keyword with the name of another tree.  Only the first remote is used when
extending a tree.


## Automagic Lists

Fields that expect lists can also be specified using a single string, and the
list will be treated like a list with a single item.  This is useful, for
example, when defining groups using wildcards, or commands which can sometimes
be one-lines, and multi-line at other times.


## Wildcards

The names in garden `tree` and `group` lists, and group member names accept glob
wildcard patterns.

The "annex" group definition is: `annex/*`.   This matches all trees that
start with "annex/".  The "git-all" group has two entries -- `git*` and
`cola`.  the first matches all trees that start with "git", and the second one
matches "cola" only.


## Symlinks

Symlink trees create a symlink on the filesystem during `garden init`.
`garden exec`, and custom `garden cmd` commands ignore symlink trees.

    trees:
      media:
        path: ~/media
        symlink: /media/${USER}

The "path" entry behaves like the tree "path" entry -- when unspecified it
defaults to a path named after the tree relative to the garden root.
