# Bad Translate
You know those mods about video games\
that translate the game content so many times that it becomes literal gibberish ?\
Well now you can do that in the comfort of your terminal

You could also add your own custom `lang-list.json`!\
This is just a silly app, so if anything breaks, its on you.

## To Build:
```
cargo build
```

## To Run:
```
cargo run
```

## Examples:
```
Usage: badtranslateapp <COMMAND>

Commands:
  translate        Translate normally from one language to another
  gibber-in-order  Translate to each language (within order of the json file) and then back to english to get the mess
  gibber-random    Translate to each language and then back to english to get the mess
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Regular translation
```
$ badtranslateapp translate "Hello From English" "en" "es"
Hello From English FROM en to es:
Hola desde inglés
-------------------------------------------
```

### Gibberish translate in order with custom limit
```
$ badtranslateapp gibber-in-order "Hello From English" 3
[TRANSLATE TO Afar]:
Nagaale ingliizik
-------------------------------------------
[TRANSLATE TO Abkhaz]:
Бзиара умаз
-------------------------------------------
[TRANSLATE TO Acehnese]:
Halo
-------------------------------------------
Back to English:
Halo
-------------------------------------------
```
