````bash,exec=2
echo 1>&2 'some error output' && exit 2
````

````output
some error output
``````

````rust
fn foo() {
    let x = 5;
}
````

````bash
$ echo 'something'
$ echo 'that will never be executed'
````low from included code
this is the post-call
```````