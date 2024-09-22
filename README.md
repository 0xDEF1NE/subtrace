<h1 align="center">
  <img src="static/logo.png" alt="subtrace" width="200px">
  <br>
</h1>

<h4 align="center">Subdomain enumeration tool.</h4>

---

subtrace is a tool designed to simplify the search for subdomains across various APIs and services, using  YAML templates. The goal is to make the work of pentesters and bug hunters easier by offering a practical and efficient solution for subdomain enumeration.

# Installing

```bash

First, make sure that `cargo` is installed on your system.
For Debian or Arch-based systems, you can use the following commands:

### Debian/Ubuntu:
sudo apt install cargo

### Arch-based:
sudo pacman -S cargo

### Fedora:
sudo dnf install cargo
...
```

After installing `cargo`, compile the code using the following command:

```sh
cargo build --release
```

Then, move the compiled binary to `/usr/bin/` to make it globally accessible:

```bash
sudo mv target/release/subtrace /usr/bin/
```

# Usage

```sh
subtrace --help
```

The command above results in the output below, displaying all the tool parameters and available options

```yaml
Subdomain Enumeration tool

Usage: subtrace [OPTIONS] --domain <DOMAIN>
       subtrace <COMMAND>

TARGET:
  -d, --domain <DOMAIN>
          Specify the main domain to search for subdomains.

TEMPLATE:
  -t, --templates <TEMPLATES>
          Specify the directory with templates for subdomain scanning.

  -l, --list-templates
          List all installed templates.

OUTPUT:
  -o, --output <OUTPUT>
          Specify the filename to write the output to.

      --silent
          Suppress verbose output and display only the findings.

OPTIMIZATIONS:
  -c, --concurrency <CONCURRENCY>
          Set the maximum number of templates to be executed in parallel (Default:12).

DEBUG:
      --debug <DEBUG>
          Set the debug level: 0 = ERROR, 1 = WARN, 2 = INFO, 3 = DEBUG

OPTIONS:
  -h, --help
          Show this help message and exit.
```

# License

subtrace is released under MIT license. See LICENSE.
