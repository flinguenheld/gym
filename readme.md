## gym 💪

Maintain your memory level with gym !  
This small game allows you to practice mental arithmetic in the terminal. 🧠


### Maths

<div align="center">
  <img src="./images/multiplication.gif">
</div>

`gym --multiplication`  
`gym --addition`  
`gym --substraction`  
 

### Vocabulary
Unbelievable! You can also practice you vocabulary in your terminal 🤯  

<div align="center">
  <img src="./images/vocabulary.gif">
</div>

`gym --vocabulary`  

This part requires a file named **vocabulary.txt** beside the gym's binary with this structure :
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
