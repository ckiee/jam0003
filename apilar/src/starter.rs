pub const PROGRAM_TEXT: &str = "
# This is a full-on replicator that can be used
# to see the simulation. It replicates itself, 
# tries to grow memory, splits into two

# startup delay, because the stack of the copy
# will have the wrong start address without it
NOOP  
NOOP
NOOP
NOOP
NOOP
NOOP  # delay so we take the right address after SPLIT

# take the address of the start, adjusting it for the delay
ADDR  # s
N6
SUB   # adjust s for delay
DUP   # s c
DUP   # s c c
N8
N8
MUL
ADD   # s c t target is 64 positions below start
SWAP  # s t c

# start copy loop
ADDR  # s t c l
EAT   # do some eating and growing while we can
GROW
SWAP  # s t l c
ROT   # s l c t
DUP2
ADD   # s l c t c+t
ROT   # s l t c+t c
DUP   # s l t c+t c c
READ  # s l t c+t c inst
ROT   # s l t c inst c+t
SWAP  # s l t c c+t inst
WRITE # s l t c
N1
ADD   # s l t c+1
ROT   # s t c+1 l
SWAP  # s t l c+1
DUP   # s t l c+1 c+1
ADDR  # end
N7
N3
MUL   # 21
ADD   # s t l c+1 c+1 end
LT    # s t l c+1 b
ROT   # s t c+1 b l
SWAP  # s t c+1 l b
JMPIF # s t c+1

# done with copy loop
DROP  # s t
OVER  # s t s
ADD   # s s+t
DUP   # s s+t s+t
START # s s+t spawn processor into copy
# now split memory just before it
N2    
SUB   # s s+t-2 split_addr
RND   # random direction
SPLIT # split from s+t-2
JMP   # jump to first addr";
