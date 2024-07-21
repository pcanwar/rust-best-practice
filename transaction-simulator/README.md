### Transaction Simulator

Simple command-line app built in Rust that simulates financial transactions between accounts with no hashing of transaction or using private keys.. 

This lets you:

* Send tokens between accounts
* Receive tokens into an account
* Check the balance of any account

### How to Use It

* Run it from the terminal.
 
*Commands:
* To send tokens: Type 
```sh
se <from_account> <to_account> <amount>
```
* To receive tokens: Type 

```sh
re <to_account> <amount>
```
* To check balance: Type 
```sh
bal <account_name>
```
* To get help on commands: Type

```sh 
h
```
* To exit the app: Type 

```sh
quit
```

### Examples:

* Sending 50 tokens from Alice to Bob: 

```sh 
se Alice Bob 50
```

*  Receiving 30 tokens to Alice: 

```sh 
re Alice 30
```

* Checking Alice's balance:

```sh 
bal Alice
```

* Getting help: 

```sh 
h
```

* Exiting the program: 

```sh 
quit
```

