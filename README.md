# cloc

This is a simple command-line tool written in Rust that counts the number of Lines Of Code in a specified directory.

## Usage

Run the tool by executing the following command:

```bash
cargo run -- [OPTIONS]
```

Replace [OPTIONS] with the desired options for the tool.
Options

The tool supports the following options:

**-p**, **--path** [DIRECTORY]: Specifies the directory to count lines of code in. If not provided, the tool will count lines of code in the current working directory.

**-l**, **--log**: Enables logging of the line counting process.

## Examples

1. Count lines of code in the current working directory:

    ```bash
    cargo run
    ```

2. Count lines of code in a specific directory:

    ```bash
    cargo run -- -p /path/to/directory
    ```

3. Count lines of code in a specific directory and log results into a sqlite database (*not yet implemented*)

    ```bash
    cargo run -- -p /path/to/directory -l
    ```

## Output

The tool leverages the [ignore](https://docs.rs/ignore/latest/ignore/) crate to iterate through the files in the specified directory and its subdirectories. For each file encountered, it counts the number of lines of code and displays the filename along with the line count.

The **ignore** crate will exclude hidden and *git* related files by default but there´s an extra "ignore.g" file to expand the exclusion list.

### Logging (NOT YET IMPLEMENTED)

Adding the logging option (**-l**, **--log**) will save statistics into a SQLITE database. When logging is enabled, it updates the database with the total line count for the parent folder of the specified directory.
