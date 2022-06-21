# TP-CLI

Your average path manager.

## Description

This is a cli tool to simplify the process of cd-ing into your common directories.

## Example

I have a project called `machin` in this directory, I can add a name to that directory.
```sh
cd Project/machin
#pwd: /home/mb/Project/machin
tp +machin
```
latter, in a new shell I can CD into it like so:
```sh
# pwd: /home/mb/
tp machin
#pwd: /home/mb/Project/machin
```

## Should you use it?

Consider other options, there are much better tools than this one serving the same purpose, for instance [fzf](https://github.com/junegunn/fzf) paired with a [function](https://gist.github.com/NileDaley/303090d4e8ee625807ee9f0ee1b8ef53).

## Then why?

I do use it myself because I prefer to make my own tools instead of using someone else's.
