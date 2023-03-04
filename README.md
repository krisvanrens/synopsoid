# synopsoid

Markdown file outline parser.
It will filter all level 1 and 2 sections for printing/JSON output.
Also, synopsoid will deduplicate consecutive headers and remove formatting such as HTML and Markdown from the header text.

## Usage

```text
Usage: synopsoid [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>      The path to the input markdown file to parse
  -o, --output <OUTPUT>  The path to an (optional) output JSON file. Providing this will disable printing [default: ]
  -h, --help             Print help
  -V, --version          Print version
```

## Example

Input file `test.md`:

```md
# Chapter 1

Text will be skipped everywhere.

## Section 1.1: <emph>Yadayada</emph>

### Subsections will be skipped

# Chapter 2<br/>Subtitle

## Section 2.1: **bold**
## Section 2.2: `match`{.rs}
```

Synopsoid output for printing:

```text
$ synopsoid --path test.md

⇒ Chapter 1
  ↳ Section 1.1: Yadayada

⇒ Chapter 2 Subtitle
  ↳ Section 2.1: bold
  ↳ Section 2.2: match
```

Synopsoid output for generating JSON output:

```text
$ synopsoid --path test.md --output test.json
$ jq < test.json
```

```json
[
  {
    "H1": "Chapter 1"
  },
  {
    "H2": "Section 1.1: Yadayada"
  },
  {
    "H1": "Chapter 2 Subtitle"
  }
  {
    "H2": "Section 2.1: bold"
  },
  {
    "H2": "Section 2.2: match"
  }
]
```

## Ideas

- Use the `Display` trait instead of a custom one.
- Custom JSON serializer.
- Change the file line reader in an iterable collection...just because.
