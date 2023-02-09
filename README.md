# ticket-assistant

A small application built to assist in customer support/service or roles where ticket tracking is important.

The Email Hinter will take an email and obscure the middle-part of the email. This can be useful if you need to provide an email in a secure way.

![image](https://user-images.githubusercontent.com/67988191/213208864-76de0249-9a88-45c7-ba73-17db4e50766e.png)


The Ticket Tracker will take in ticket numbers (Formatted as #12345) and save them to a list. From there you can copy them to your clipboard.

![image](https://user-images.githubusercontent.com/67988191/213208818-66f7d0c0-535e-4615-a2c0-56fb7b80f10a.png)

Features
--------

Dark + Light mode

![image](https://user-images.githubusercontent.com/67988191/213207338-d848d539-2a0a-4b39-8f71-563af341820c.png)

![image](https://user-images.githubusercontent.com/67988191/213207390-0c4be9cb-bb97-4526-82c3-75b5f8409f0a.png)

Ability to remove Sidepanel and Signature

![image](https://user-images.githubusercontent.com/67988191/213207489-a087a4a7-f9c5-4e6c-ae19-600e61da412a.png)

Password Mode for enhanced security when working with emails

![image](https://user-images.githubusercontent.com/67988191/213207675-297fa74c-7c72-4a5e-9f84-95a9834502a5.png)

Compiling/Building From Source
------------------------------
I ran into some issues running the windows version under a virtual-machine so if you are also experiencing this and would like to use the program or would just like to build from source, follow these steps.

First you need to download the repo. You can do this using git or just downloading the zip file. This can be done by clicking the big green 'code' button and pressing 'download zip'. From there, just unzip and put it somewhere accessible.

The next thing you need to do is install the Rust language. You can do so by simply getting the installer from their website: rust-lang.org/tools/install
Make sure you do the quick install if you do not have the prerequisites (This would be option one in the installer). Once visual studio is installed just continue with default.

Once you have rust installed, you can then move on to compiling how ever you desire.

Since Rust is very easy to use, mainly due to cargo, the final steps are quite simple.

Simply open a terminal or powershell and navigate to the directory/folder containing the cargo.toml file. From there run the following commands:

'cargo test'

This will install dependencies, which there are quite a few, and test for errors. Once that is complete run the next command to build a release:

'cargo build --release'

This will then build the executable. This is the .exe that you will use to run the program. Once that is complete you have successfuly compiled the program. You can extract the exe from '/target/release/email_hinter.exe' and use it!


Other
-----
Written in Rust and powered by egui and eframe. Will continue to update and add features to make it more versatile and streamlined. 
If you encounter any issues with the program please let me know. Feel free to edit and re-distribute this project all you want.

Please note this is just a "quick" version of the app that I built in less than two days for the sole purpose of just using it myself.
