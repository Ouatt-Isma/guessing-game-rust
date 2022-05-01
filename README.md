# guessing-game-rust


```sh
$> git clone https://github.com/apache/incubator-teaclave-sgx-sdk.git
```

```sh
$> git clone https://github.com/Ouatt-Isma/guessing-game-rust.git
```

```sh
$> docker pull baiduxlab/sgx-rust
```

```sh
$> docker run -v /Users/ouattaraismael/Documents/Mith/Prog/SGX-Rust/tmp/incubator-teaclave-sgx-sdk/:/root/sgx -ti baiduxlab/sgx-rust
```

```sh
$> cd sgx/guessing-game-rust/guessing_game/
$> make SGX_MODE=SW
```

Enjoy the game


```sh
$> cd bin
$> ./app
```

````
[+] Init Enclave Successful 30313879175170!
Enter a max value:
4
max value: 4
enter a guess value:
10
you entered: 10
Wrong guess!
enter a guess value:
12
you entered: 12
Wrong guess!
enter a guess value:
14
you entered: 14
Wrong guess!
enter a guess value:
15
you entered: 15
Wrong guess!
enter a guess value:
16
you entered: 16
thread '<unnamed>' panicked at 'Enclave autolock because 4(max value)<5(number of tentative)', enclave/src/lib.rs:80:10
fatal runtime error: failed to initiate panic, error 5
Illegal instruction
`````
