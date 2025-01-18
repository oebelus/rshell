# R-SHELL

This is a REPL POSIX compliant shell in Rust that's capable of interpreting shell commands, running external programs and some builtin commands.

Currently, this Shell supports:
- Builtins: `exit`, `echo`, `cat`, `type`, `cd`, `pwd`
- Navigation:
   - Absolute paths;
   - Relative paths;
   - Home directory (~).
- Quoting:
   - Single quotes;
   - Double quotes;
   - Backslash outside quotes;
   - Backslash within single quotes;