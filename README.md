A basic random password generator and password strength checker CLI app built with Rust.

## Installation

```sh
git clone https://github.com/AlexMordred/secret.git
cd secret
cargo install --path .
```

## Usage

### Generate random password

```sh
secret generate
```

Specify password length (the default is `16`)

```sh
secret generate 32
```

Specify characters to be included in the password. Use small and capital letters, number and special characters (default setting):

```sh
secret generate 16 aA1@
```

Use small and capital letters:

```sh
secret generate 16 aA
```

Use small letters and numbers:

```sh
secret generate 16 a1
```

See full command description:

```sh
secret generate --help
```

### Check password strength

```sh
secret check qwerty123
```

Use single quotes around passwords containing special characters.

```sh
secret check 'NZanLuEoT|4Ej^(z'
```
