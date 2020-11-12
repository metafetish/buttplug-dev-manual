# About This Guide

The Buttplug Developer Guide covers a few different topics.

- Architecture choices made in both the protocol and the reference libraries.
- Using the reference libraries in applications.
- Different uses for raw Buttplug Messages.

Any portion of the guide that uses the reference libraries should be considered **a way** to do things, but not **the (only) way**. The only invariant portion of Buttplug is the actual protocol, as laid out by [the Buttplug Spec](https://buttplug-spec.docs.buttplug.io). The reference libraries are one try at implementing an API for this protocol. Others are free to implement their own APIs on top of the raw protocol.

Chapters involving code will have interfaces similar to the one shown here.

<CodeSwitcher :languages="{rust:'Rust', csharp:'C#', ts:'TypeScript', js:'JS', twine: 'Twine (Sugarcube)'}">
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
<template v-slot:js>

```js
// This is some Javascript
let a = 1 + 2;
```

</template>
<template v-slot:ts>

```ts
// This is some Javascript
let a = 1 + 2;
```

</template>
<template v-slot:twine>

```html
<!-- This is a twine example -->

<< 1 + 2 >>
```

</template>
</CodeSwitcher>

The guide tries to cover examples for all reference implementations, where possible. Due to variations in program language features, there is a good chance that while all of the examples will achieve the same goal, they may do so in very different ways. Notes about language specific requirements and implementations will be included as comments in the examples for that language.

## Terms To Be Familiar With

Here's a quick set of terms and definitions to be familiar with before continuing. These will be used and probably re-explained throughout the document, but having them all here means no one has to go looking for them.

* **Device**
    * A device is the general term for anything that buttplug connects to and controls. Sex toys, gamepads, estim units, fucking machines, whatever.
* **Server** 
    * The part of Buttplug implementations that manages device connections and communication. This may be a standalone server, or may exist inside an application that uses Buttplug. Those wanting to add implementations for new devices will do so in Buttplug Server code.
* **Client**
    * The part of Buttplug implementations that applications use in order to access servers. Client APIs are what most Buttplug applications developers will see.
* **Message**
    * Buttplug messages are defined in the [Buttplug Spec](https://buttplug-spec.docs.buttplug.io), and are how Buttplug Clients and Servers communicate with each other.
* **Connector**
    * A piece of software that sits on top of a client/server so it can talk to the corresponding pieces in some way. This could be embedded (other side in same program) via networks (i.e websockets), ipc (i.e pipes), or other mechanisms.

## What to Read

Obviously, reading the whole guide is best. A lot of work was put into writing this, and the best way to appreciate that work is to read and savor every single word.

However, based on feedback, apparently readers have "other things to
do". With that severe disrespect in mind, here's a list of what you'll
learn from each of the sections in this guide.

* **Flared Basics**
    * Well, you're reading it already, so that's a good start. The intro also contains a summary of just a few of the many, many ethics issues encountered when writing sex software, which is a good way to find out what you may be responsible for if you decide to work with the library. There's also an overview of what Buttplug architecture looks like that's pretty key to the rest of the guide. Everyone should read this whole section.
* **Sticking Buttplug In**
    * All about the Client side of Buttplug. Useful for those wanting to build applications using prebuilt servers. If you just want to build a game, movie player, control app, or whatever, this is the chapter for you.
* **Inflating Buttplug**
    * How to add things to the Buttplug Core, including extending Buttplug Servers with new device types and communication busses, implementing Buttplug servers, adding new message to the Protocol Spec, and writing clients in new programming languages.
