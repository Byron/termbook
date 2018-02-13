````bash,prepare=one
echo 'one'
````

````bash,use=one,prepare=two
echo 'two'
````

````bash,use=two,exec
echo 'with multiple preparation'
````

````output
one
two
with multiple preparation
````