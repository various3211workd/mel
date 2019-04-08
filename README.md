# mel
![68747470733a2f2f71696974612d696d6167652d73746f72652e73332e616d617a6f6e6177732e636f6d2f302f3336373337372f32353333323331392d616365622d343136322d323536352d3662346439616162313537392e706e67](https://user-images.githubusercontent.com/43775946/53321943-3c4c9f00-391d-11e9-9483-083ee9c8201a.png)

mel is the coined term of "memo" + "(TIL)Today I Learning"

Can look back my daily notes.

## Supported OS
✔ Windows  
✖ Mac  
✖ Linux  

## Donwload
move release tab and donwload

## Usage

### Specification of til
Execute in the directory you want to register to mel.

```
PS>./mel init

[ o ] Init Complete
```

### show init list
Record the file under the init directory.

If the name ends with "README.md", "README.md" is omitted, but it is displayed if it is directly under the init directory.

```
PS>./mel list

[ <3 ] TIL List

[ 1 ] /c/compile
[ 2 ] /firebase
[ 3 ] /powershell/import_VisualStudio_code.md
[ 4 ] /README.md
[ 5 ] /Rust/args
[ 6 ] /Rust/json
[ 7 ] /Rust/std_fs
[ 8 ] /Rust/termcolor
[ 9 ] /security/cdb
[ 10 ] /security/process_hollowing
[ 11 ] /security/rp++
[ 12 ] /security/Thread_Execution_Hijacking
```

add -d option is delete string.

```
PS>./mel list -d security

[ <3 ] TIL List

[ 1 ] /c/compile
[ 2 ] /firebase
[ 3 ] /powershell/import_VisualStudio_code.md
[ 4 ] /README.md
[ 5 ] /Rust/args
[ 6 ] /Rust/json
[ 7 ] /Rust/std_fs
[ 8 ] /Rust/termcolor
```

### show tils  
```
PS>./mel [list_line]
```

You can see the til file at the terminal.  
It is written in html.

```
PS>./mel /Rust/args
# args

## cargo run時にパスを渡す
cargo run --args ***
```

or  

```
.PS>/mel -n [list_number]
```

```
PS>./mel -n 5
# args

## cargo run時にパスを渡す
cargo run --args ***
```

### List Update
Update mel directory's init directory

```
PS>mel update
[ U ] Update Complete
```

### Put html
Show Html code.
```
PS>./mel -no 5
<h1>args</h1>
<h2>cargo run時にパスを渡す</h2>
<p>cargo run --args ***</p>
```

### Write TIL
Simple editing of TIL is possible.
```
PS>./mel -wn 9 "pencil is my father"
[ W ] Write Complete
```
