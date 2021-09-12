## resetty - heuristic terminal reset clone
resetty resets ttys in a dynamic fashion if no tty is specified. It
employs some heuristics to find the attached terminal. (It parses the
"proc" directory for example). Otherwise it just resets the specified
tty, if permissible by the operating system.
