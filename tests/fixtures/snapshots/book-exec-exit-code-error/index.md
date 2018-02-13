```bash,exec=2
echo 1>&2 'some error output' && exit 2
```

```output
some error output
```