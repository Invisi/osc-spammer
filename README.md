# osc-spammer
osc-spammer is a short and simple CLI tool to quickly send messages to software listening for 
[Open Sound Control](https://en.wikipedia.org/wiki/Open_Sound_Control) packets.

# Installation
```bash
cargo install --git https://github.com/Invisi/osc-spammer.git
```

# Usage
To send a message to port `9000` on channel `/test/abc`, use the following commands:
```bash
# String
osc-spammer /test/abc 9000 --string "Hello world"

# Bool
osc-spammer /test/abc 9000 --bool true

# Int
osc-spammer /test/abc 9000 --int 123

# Float
osc-spammer /test/abc 9000 --float 3.14159
```

Repeating the message is possible via `--count`, delay between messages can be configured via `--delay`.  
For more details, check the help via `osc-spammer --help`.
