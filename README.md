# escapecolor

`escapecolor` is a rust crate that provides both a binary and a lib to escape colors from a string following [ANSI SGR specifications][1].

## Usage

### As a binary:

```
echo -e '\E[31mhello\E[7;32min\E[0;4;44mcolors\E[0m' | escapecolor
```

### As a lib:

```
use ansi_string::AnsiString;

fn main() {
   let bytes_with_color = vec![27,91,51,52,109,72,101,108,108,111,27,91,48,109];
   let string_with_color = String::from_utf8(bytes_with_color).unwrap();
   let ansi_string = AnsiString::new(string_with_color);
   println!("Without colors: {}", ansi_string.without_colors);
   println!("With colors: {}", ansi_string.original);
}
```

[1]: https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters
