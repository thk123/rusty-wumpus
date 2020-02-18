Rust implementation of "Hunt the Wumpus". 

Specification

Hunt the Wumpus" (or just "Wump") is a simple (non-graphically) computer 
game originally invented by Gregory Yob. 

The basic premise is that a rather smelly monster lives in a dark cave
consisting of connected rooms.
 
Your job is to slay the wumpus using bow and arrow. In addition to the 
wumpus, the cave has two hazards: bottomless pits and giant bats.
 
If you enter a room with a bat, the bat picks you up and drops you into 
another room. 

If you enter a room with a bottomless pit, its the end of 
the game for you. 

If you enter the room with the Wumpus he eats you. 

When you enter a room you will be told if a hazard is nearby:

"I smell the wumpus": It´s in an adjoining room.
"I feel a breeze": One of the adjoining rooms is a bottomless pit.
"I hear a bat": A giant bat is in an adjoining room.

For your convenience, rooms are numbered. Every room is connected by tunnels to 
three other rooms. When entering a room, you are told something like
 
```
" You are in room 12; there are tunnels to rooms 1,13, and 4: move or shoot?"
```

Possible answers are:
 - m13 ("Move to room 13") 
 - s13-4-3 ("Shoot an arrow through rooms 13,4, and 3"). 
 
The range of an arrow is three rooms. At the start of the game, you have
five arrows. The snag about shooting is that it wakes up the wumpus 
and he moves to a room adjoining the one he was in - that could be your
room. 

Be sure to have a way to produce a debug output of the state of the cave.
