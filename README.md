# synopsoid

Markdown talk outline parser.

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

Synopsoid output:

```text
⇒ Chapter 1
  ↳ Section 1.1: Yadayada

⇒ Chapter 2 Subtitle
  ↳ Section 2.1: bold
  ↳ Section 2.2: matc
```

## Ideas

- Change the file line reader in an iterable collection.
- Add JSON output.
