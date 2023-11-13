# rtools
Rtools is a command line tool suite written in Rust. Contains basic rust implementations of common tools like grep, find, and gzip, and a command line text editor akin to nano.  
***Note*** : Commands are not globally executable yet as this is still a work in progress.
## Setup
- Clone the repository and ```cd``` into it. 
- Run any of the binaries using ```cargo run --bin <command_name> <arguments>```
## Usage
### rman
View usage instructions for the other commands using this command.  
```cargo run --bin rman <command_name>```
### rfind
Find instances of a file inside a specified directory.  
```cargo run --bin rfind <file_name> <directory_name>```
### rgrep
Find instances of a string inside the specified file/directory.  
```cargo run --bin rgrep <pattern> <directory/file_name>```
### rzip
Compress the specified file.  
```cargo run --bin rzip <source> <target>```  
Run with the ```--unzip`` flag to decompress a gzip-encoded file to a target of your choice.

### rnano 
Open and edit text files from the command line.  
```cargo run --bin rnano <file_name>```
