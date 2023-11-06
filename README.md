______________________________________________
![maxresdefault](https://i.imgur.com/UvcZifT.jpg)
______________________________________________
# XorLock
XorLock is a simple File/Directory encryption and decryption using Xor operation in rustlang. 

## Motive
I've been wondering about how ransomware works for a while. After doing some research, I started looking into some file encryption with AES or RC4 used by some ransomware operators.... Sorry Sorry i will not cover this AES or RC4 topics, I will provide you some of basic idea of file encryption simply by using XOR cipher.  

## XOR ?
 The XOR cipher (pronounced "exclusive or") is an additive cipher that depends on the XOR operation. If you want to take a deep look on XOR cipher check out this: (https://en.wikipedia.org/wiki/XOR_cipher)
 
 You can find the XOR principles below where(^ Denotes the XOR operation):<br>
<pre class="notranslate"><code>0 ^ 0 = 0
0 ^ 1 = 1
1 ^ 0 = 1
1 ^ 1 = 0
</code></pre>

## How XorLock works and its functions ?
The xorlock uses the XOR logical operation to encrypt data. A random or chosen key is used to perform this operation by creating of encrypted data. To decrypt, the same key is applied with the XOR operation again.
The XOR operation relies on a single key for both encryption and decryption, making it a symmetric encryption method.

Covering the most important function which is:
```rust
  fn encrypt(path: &Path, key: u8) {
        let mut file = File::open(path).expect("Cannot open the file");

        let mut file_content = Vec::new();
        file.read_to_end(&mut file_content).unwrap();

        for bytes in &mut file_content {
            *bytes ^= key;
        } 

        let mut file = File::create(path).expect("Cannot create a file");
        file.write_all(&file_content).unwrap();

    }
```
<h3>Code Break-Down </h3>

The first line opens the file and reads the file content into a vector of bytes. There's a for loop that modifies the file contents by applying the XOR operation to each bytes with the specified key and finally writes the modified content back to the same file.

## Installation and Usage
Your system should present rust:
```bash
$ git clone https://github.com/0x00snape/XorLock.git
$ cd XorLock
$ rustc xorlock
$ ./xorlock
```
## License
This project is licensed under [MIT](https://github.com/0x00snape/XorLock/blob/main/LICENSE)
