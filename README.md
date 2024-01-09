# Susuwatari
Linux only

A little tray app to get the clipboard content with some history, so you don't have to re-copy all the time

The executable is in target/debug/


# In the src/main.rs :

if you want to have more characters displayed, you can change the const LINE_LENGTH to the number of characters you want.

if you want a longer history, you can change the const BUFFER_LENGTH to the number of copy you want the app to remember


# For a proper installation
copy the executable susuwatari (in target/debug/) and put it in your /usr/local/bin/
move or copy resources/icon.png to /etc/susuwatari/
