# Whitegrove
An experimental semi-interactive roguelike built in Rust.

You can play the game [in your browser](https://ndouglas.github.io/whitegrove/) (assuming your browser supports it).

My other research roguelike, [Downdelving](https://github.com/ndouglas/downdelving/), is at this point somewhat traditional.  It was built following Bracket's [exceptional tutorial](https://bfnightly.bracketproductions.com/).

_Whitegrove_ is quite different.  It's roguelike in some ways, but I'm not sure if it's really a game or not.  It may replace _Downdelving_, or it might sputter out and die in a matter of a few commits.

But here's the basic idea:

### Instead of controlling your `@` interactively...
...you will program it using an embedded interpreter or other language.  This is low priority at the moment.

### Instead of programming explicit artificial intelligence into mobs...
...mobs will utilize neural nets and evolve over generations, each generation being an encounter with the player, other mobs, and the simulated environment.

The idea is that we can run little simulations of roguelike worlds and watch species of monsters evolve and adapt to their local conditions and threats over time.

### Instead of hacking and slashing your way through a dungeon...
...your `@` will be another stimulus injected into their environment that will behave as you specifically instruct.  Perhaps you bring peace.  Perhaps you bring a sword.  Either way, you'll see the mobs adapt to your presence and effects on the environment.

So IDK how well this will work in practice, but it might be kinda cool.  Let's find out.
