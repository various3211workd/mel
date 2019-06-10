# mel
![キャプチャ](https://user-images.githubusercontent.com/43775946/57007055-ba306d80-6c20-11e9-8b88-bb8a75a204f3.PNG)


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

```powershell
PS>./mel init

[ o ] Init Complete
```

### show init list
Record the file under the init directory.

If the name ends with "README.md", "README.md" is omitted, but it is displayed if it is directly under the init directory.

```powershell
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

```powershell
PS>./mel -d security

[ <3 ] TIL List

[ 1 ] /c/compile
[ 2 ] /firebase
[ 3 ] /powershell/import_VisualStudio_code.md
[ 4 ] /README.md
[ 5 ] /Rust/args
[ 6 ] /Rust/json
[ 7 ] /Rust/std_fs
[ 8 ] /Rust/termcolor

[ D ] delete Complete
```

### show tils  

```powershell
PS>./mel -n [list_number]
```

You can see the til file at the terminal.  
It is written in html.

```powershell
PS>./mel -n 5
# args

## cargo run時にパスを渡す
cargo run --args ***
```

### List Update
Update mel directory's init directory

```powershell
PS>mel update
[ U ] Update Complete
```

### Put html
Show Html code.
```powershell
PS>./mel -n 5 --html
<h1>args</h1>
<h2>cargo run時にパスを渡す</h2>
<p>cargo run --args ***</p>
```

### Write TIL
Simple editing of TIL is possible.
```powershell
PS>./mel -wn 9 "pencil is my father"
[ W ] Write Complete
```

### Get Content

```powershell
./mel -g <url> <filepath>
```

example

```powershell
PS>./mel -g https://google.com get_contents/google_home.md
[ G ] Get ontents
```