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
   - Backslash within double quotes;
   - Executing a quoted executable.
- Redirection:
   - Redirect stdout;
   - Redirect stderr;
   - Append stdout;