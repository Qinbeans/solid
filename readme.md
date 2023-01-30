# Solid

Solid is a continuation of Bricks, a sandbox like game representing an RPG world made for modularity.  Think dungeons and dragons had a child with Zelda or some other 2D RPG game.

## Structure

The game is made up of a few parts:
- Mods
- Logic
- Assets

### Mods

Mods is the idea where made to be data oriented and all any mod make has to do is list what they want the game to do.  There's some "programming" involved in the sence there are predefined functions that can be called, but the idea is that the game is made up of a bunch of mods that can be added and removed at will.

### Logic

Logic is where predefined functions are defined.  These functions are called by mods and are the "programming" part of the game.  The idea is that the logic is the core of the game and the mods are the data that is used to make the game.  Some other core parts of the game exist like default textures and a default story, but that is detailed in `Assets`.

We essentially create our own language ontop of the game engine to produce some result.

### Assets

The assets contain the core of the game aside of logic.  The assets are the textures, sounds, and other things that are used by the game.  The assets are also used by mods to add new things to the game.

## Security

As I do plan to make this multiplayer, how will I maintain security?  Simple answer is through checksums.  I can't really be bothered with checking 3rd party software, but I can at least check the integrity of the game in comparison to a server.
