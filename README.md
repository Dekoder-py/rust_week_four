# Kyle's Tinylang

## Syntax

Strings: use `'` or `"` for strings.

Conditionals:

```
if 1 == 1 then
    says "hi";
end

if 1 != 1 then
    says "no";
else
    says "yes";
end

unless 1 == 2 then
    says "hi";
end
```

Variables: just put the name = val: `i = 2`

Builtin fetch, returns string: `says fetch(url)` || `resp = fetch(url)` (returns response body)

### Run kyle-tinylang FILENAME to run your script. There's an example file called test.tl in this repo.
