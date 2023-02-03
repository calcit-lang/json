## Calcit JSON

> based on [json](https://docs.rs/json/latest/json/).

### Usages

APIs:

```cirru
json.core/parse "|{\"a\": [\"b\", 2]}"

json.core/stringify $ {} (:a 1)
json.core/stringify ({} (:a 1)) true
```

Install to `~/.config/calcit/modules/`, compile and provide `*.{dylib,so}` file with `./build.sh`.

### Workflow

https://github.com/calcit-lang/dylib-workflow

### License

MIT
