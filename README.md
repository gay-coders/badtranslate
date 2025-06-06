# badtranslate

A *silly* app that "badly" translates text by running it through Google Translate multiple times.

## what it do?

- **normal translation**: just like google translate.
- **funny mode**: by default it translates 10 times through a random order of the languages available.
- **custom languages**: you can change the code and the lang-list.json file if you wanted to.

## how use?

### basic mode
```console
$ badtranslateapp translate -t fr "The quick brown fox jumps over the lazy dog"
```

### gibberish (translated 10 times)
```console
$ badtranslateapp gibber "The quick brown fox jumps over the lazy dog"
```

### this time translated 20 times but following the json file's order 
```console
$ badtranslateapp gibber -c 20 -o true "The quick brown fox jumps over the lazy dog"
```

## how install?

1. clone the repo
2. build with cargo
3. run with cargo (or just go in the target folder.)


always remember to be silly ;3333
