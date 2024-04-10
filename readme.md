## gym 💪

Maintain your memory level with gym !  
This small game allows you to practice mental arithmetic in the terminal. 🧠


### Maths

`gym --multiplication`  
`gym --addition`  
`gym --substraction`  

<div align="center">
  <img src="./images/multiplication.gif">
</div>


💡 Press *CTL+C* or *ESC* to quit.


### Keyboard
Don't tell me you've changed your layout again 😮‍💨 !  

`gym --keyboard <KEYBOARD>  Options: (L)etters (C)aps (N)umbers (S)ymbols (0-9...)nb characters`

<div align="center">
  <img src="./images/keyboard.gif">
</div>


### Vocabulary
Unbelievable! You can also practice you vocabulary in your terminal 🤯  

`gym --vocabulary`  

<div align="center">
  <img src="./images/vocabulary.gif">
</div>


💡 Press *TAB* to display obfuscated help.

This part requires a file named **vocabulary.txt** beside the gym's binary.  
Feel free to add all your words with this structure :
```
word=first_synonym,second_synonym,...
word=first_synonym,second_synonym,...
```


<div align="center">
  <img src="./images/vocabulary_example.png" >
</div>
 
---
### Installation

Download the release and copy **gym** and **vocabulary.txt** files in your **~/bin/** folder for instance.

`gym -h` for the full command list.  

Or clone this repo and build it with cargo (don't forget to copy vocabulary.txt):

`cargo run --release`
