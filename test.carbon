LDI R5
7 #length of array
LDI R7
0 #set pointer to 0
RLD R5 .start
CMP R7 #check if pointer is at last element
BRC EQ #if so set pointer to 0
[reset]
BRC JMP #if not, keep going
[no_reset]
LDI R7 .reset
0
MLD R7 .no_reset #load first element to R1
RST R1 
RLD R7 #increment pointer
RST R6 #store curent pointer to R6 so i dont have to decrement it later
INC
RST R7 #store incremented pointer in R7
LDI R0 #this is to take care of hazards as a NOP. emm forgot to make NOP so im using this as a placeholder
0
MLD R7 #load second element to acc
CMP R1 #check if elem 2 is greater than elem 1
BRC GT
[start] # if so dont swap
MST R6 #store acc(elem 2) to first mem cell {else swap}
RLD R1
MST R7 #store elem 1 to second mem cell
BRC JMP
[start]
