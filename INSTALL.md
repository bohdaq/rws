## Install
[Download binary](https://github.com/bohdaq/rust-web-server/releases) for you platform from releases page.
There is a mirror for downloads on [Google Drive](https://drive.google.com/drive/folders/13iSR3VxmfFvZgOZ0LddP_EJp7GJ-lQd8?usp=sharing).
### x86 64-bit Apple macOS
> sudo cp rws /usr/local/bin
>
> sudo chmod +x /usr/local/bin/rws
#### x86 64-bit Homebrew macOS
> brew tap bohdaq/rust-web-server
>
> brew install rws

### x86 64-bit Linux
> sudo cp rws /usr/local/bin
>
> sudo chmod +x /usr/local/bin/rws
#### x86 64-bit Debian
> sudo dpkg -i --force-overwrite rws.deb
#### x86 64-bit RPM
Replace _YOUR_VERSION_ with version you downloaded.
> sudo rpm -i rws-_YOUR_VERSION_.rpm

### ARM 64-bit Linux
> sudo cp rws /usr/local/bin
>
> sudo chmod +x /usr/local/bin/rws
#### ARM 64-bit Debian
> sudo dpkg -i --force-overwrite rws.deb

### x86 64-bit Windows
Copy executable to _C:\WINDOWS\system32_ folder.


### Testing installation
To check installation execute the following code in the terminal:

> $ rws

You will see similar output:

> Rust Web Server
>
> Version:       YOUR_VERSION
>
> Authors:       Bohdan Tsap <bohdan.tsap@tutanota.com>
>
> Repository:    https://github.com/bohdaq/rust-web-server
>
> Desciption:    rust-web-server (rws) is a static content web-server written in Rust
>
> Rust Version:  RUST_VERSION
> 
> ...
> Hello, rust-web-server is up and running: http://127.0.0.1:7888


Open browser, go to http://127.0.0.1:7888, you'll see default page.

Go back to terminal, press Ctrl + C (or CMD + C) to stop server.