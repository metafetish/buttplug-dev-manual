# About This Guide

The Buttplug Developer Guide covers a few different topics.

- Architecture choices made in both the protocol and the implementations.
- Using the library in applications.
- Examples of different usage patterns.

This guide should be considered **a way** to do things, but not **the (only) way**. The only invariant portion of Buttplug is the actual low-level protocol, as laid out by [the Buttplug Spec](https://buttplug-spec.docs.buttplug.io). The libraries mentioned in this guide are one try at implementing an API on top of that protocol. Other developers are free to implement their own APIs as they see fit that use that protocol, and the whole system should still work as a whole.

Chapters involving code will have interfaces similar to the one shown here. Changing the language choice on any example will change it on all examples in the project, so you can read using whatever language you're comfortable with.

<CodeSwitcher :languages="{rust:'Rust', csharp:'C#', python:'Python', ts:'TypeScript', js:'JS', twine: 'Twine (Sugarcube)'}">
<template v-slot:rust>

```rust
// This is some Rust
let a = 1 + 2;
```

</template>
<template v-slot:csharp>

```csharp
// This is some C#
var a = 1 + 2;
```

</template>
<template v-slot:python>

```python
# This is some python
a = 1 + 2
```

</template>
<template v-slot:js>

```js
// This is some Javascript
let a = 1 + 2;
```

</template>
<template v-slot:ts>

```ts
// This is some Typescript
let a = 1 + 2;
```

</template>
<template v-slot:twine>

```html
<!-- This is some Twine Sugarcube Syntax -->

<< 1 + 2 >>
```

</template>
</CodeSwitcher>

The guide tries to cover examples for implementations in all project maintained langauges. Due to variations in programming language features, there is a good chance that while all of the examples will achieve the same goal, they may do so in very different ways. Notes about language specific requirements and implementations will be included as comments in the examples for that language.

::: tip What about when you see these blocks though?

When you see callout blocks like this, it usually means there's some additional information or anecdote about the project or current subject. Not required info, so it's not in the normal paragraph flow, but things that might be nice to know.

:::

## What's in the Guide

Obviously, reading the whole guide to find out what's in it is best. A lot of work was put into writing this, and the best way to appreciate that work is to read and savor every single word.

However, based on feedback, apparently readers have "other things to do". With that severe disrespect in mind, here's a list of what you'll learn from each of the sections in this guide.

* **Flared Basics**
    * Well, you're reading it already, so that's a good start. 
    * Introduction to the library, and also gives a short runthru of the unique development and ethical issues you may run into when developing applications with Buttplug.
* **Strategies Against Buttplug Architecture**
    * Recommended reading before diving into application development, even though you're probably excited and want to get right to it. I can't blame you, really.
    * Overview of the architecture of the system, including the low level protocol, common structures in the libraries, and a guide to how the implementations are built.
    * Glossary of terms that will be used in the application building portion of the system.
* **Sticking Buttplug In**
    * If you want to build a game, movie player, control app, or whatever, *AND YOU ALREADY READ THE ARCHITECTURE CHAPTER*, this is the chapter for you.
    * All about the Client side of Buttplug. Useful for those wanting to build applications that will connect to Buttplug Servers. 
* **Winning Ways For Your Buttplug Plays**
    * After reading both the architecture and application chapters, this chapter presents different recipes for common library scenarios, including:
        * How to think about controlling different toy types
        * Building patterns for playback
        * General design frameworks for common applications (movie players, games, etc)
* **Inflating Buttplug**
    * If there's something that Buttplug doesn't do that you want it to do, this chapter will be for you.
    * How to add things to the Buttplug Libraries, including:
        * Writing clients in new programming languages
        * Extending Buttplug Servers with new device types and communication busses
        * Implementing Buttplug servers (and mostly trying to convince you that you don't want to do this)
        * Adding new message to the Protocol Spec (and also trying to convince you that you want to think *really* hard before doing this)

## Other Reference Material

Outside of this guide, there are a couple of other documents that may be handy to Buttplug Developers

* [Buttplug Protocol Spec](https://buttplug-spec.docs.buttplug.io)
    * The specification for the core of Buttplug, laying out how different parts of the system should communicate. This allows other developers to build their own version of the library if they so choose. This will be covered more in the architecture section.
* [Sex Toy Protocols I Have Known And Loved (STPIHKAL)](https://stpihkal.docs.buttplug.io)
    * An encyclopedia of different computer controlled sex toy and intimate device communication protocols. If you want to bypass Buttplug completely and just build your own interface, STPIHKAL gives you the low level information you'll need to control toys.