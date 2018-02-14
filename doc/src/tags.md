## CodeBlock Tags

The special feature `termbook build` adds to your capability is that it supports
various tags that can be added to your codeblocks.

### exec

The `exec` tag can be applied to any code-block which has a designated type, such
as `bash`.
It will send the contents of the code-block to the program mentioned as type of the
code-block, and expect it to succeed by default.

````markdown
```bash,exec
echo 'this is being executed in bash'
```
````

By default `exec` expects the executed program to succeed (with exit-status 0), 
and the entire `termbook build` invocation will fail otherwise. This behaviour
is useful to assert your documentation is still matching the program you document.

You can also indicate the desired exit status, which can be useful if your program
is expected to fail.

````markdown
```bash,exec=42
echo 'this should fail with a specific exit code' && exit 42
```
````

### 'prepare' and 'use'

It's useful to be able to use arbitrary snippets that are run prior to your `exec`
code-block, which is useful to have shared code which prepares the playing field
for the code you are about to run.


````
```bash,prepare=setup
function tb() { termbook; };
```
````

Now you can `use` the block in other `prepare` blocks, or in `exec` blocks. The
former is useful for chaining `prepare` blocks.

````
```bash,use=setup,exec
tb
```
````

Here is the complete example as it will show up in your book:

```bash,prepare=alias
function tb() { termbook; };
```

```bash,use=alias,exec=1
tb
```

### hide

As you have seen in the previous example, it can be useful to hide certain code-blocks,
especially those that are used solely for preparation. It's as easy as adding the `hide`
tag.

````
```bash,hide,prepare=setup
function tb() { termbook; };
```
````

The previous example looks like this, when the `prepare` block is hidden:

```bash,hide,prepare=alias
function tb() { termbook; };
```

```bash,use=alias,exec=1
tb
```

If `hide` is used on an `exec` block, itself and its output are hidden entirely.

````
```bash,hide,exec=1
termbook
```
````
