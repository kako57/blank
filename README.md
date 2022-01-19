# 「」

「」 is a tictactoe engine.

Currently, it tends to "give up" quickly (i.e. play innacurately)
when it sees that there are no chances of winning,
if the game were to continue with perfect play.

But, **「」 never loses**, at least in a fair game.
If it does, please create an issue.

## Example Usage

```
1 | 2 | 3
---------
4 | 5 | 6
---------
7 | 8 | 9
```

The engine numbers the squares above from 1-9 (see above).
You can set the state of the game by sending a state command to the output.

```
state xo-ox---- x
move
quit
```

The first line above is for telling the engine that
there are x's in squares 1 and 5, that there are o's in squares 2 and 4,
that there rest of the squares are empty, and that it is x's turn.

`move` will ask the engine for its move based on the state, 
printing it out as the number of the square to place x/o.

`quit` is for saying goodbye to the engine.

## TODO

- fix eval command
- stronger move selection (choose best move even if state is draw with perfect play)
