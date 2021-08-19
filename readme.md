# REMCONN

REMCONN is a Rust crate to allow remote connection with multiples protocols.

The goal of this project is to learn more about Rust and about low level code and protocoles.

## Tests

To test it on Windows, we could install a SSH server.
To do this, we could follow [Official Windows Documentation].

Run the server by running in the PowerShell CLI :
```PowerShell
Start-Service sshd
```

To make the service automaticaly executed on startup, look in the documentation.

To connect on it, we have to use the local user.
We can get it by running in the PowerShell CLI :
```PowerShell
[System.Security.Principal.WindowsIdentity]::GetCurrent().Name
```

## Sources

https://datatracker.ietf.org/doc/html/rfc4251
https://datatracker.ietf.org/doc/html/rfc4252
https://datatracker.ietf.org/doc/html/rfc4253
https://datatracker.ietf.org/doc/html/rfc4254

https://github.com/gbonacini/tssh
https://blog.guillaume-gomez.fr/Rust/3/6

[Official Windows Documentation]: https://docs.microsoft.com/fr-fr/windows-server/administration/openssh/openssh_install_firstuse
