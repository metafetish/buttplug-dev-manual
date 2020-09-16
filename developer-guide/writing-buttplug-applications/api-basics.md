# API Basics

Most of the Client Reference Implementations are built to look as similar as possible. However, there may be instances where language options (i.e. existence of things like first-class events) change the API slightly. This section goes over how the client APIs we've provided work in a generic manner. Note that these are not a standard, more just the way the Buttplug Core Team have chosen to do things. If you'd like to implement your own client in your own way, *go nuts*.

## Client/Server Interaction

There are two types of communication between the client and the server:

- Symmetric (Client -> Server -> Client)
    - Client sends a message, server replies. For instance, when a device command is sent from the client, the server will return information afterward saying whether or not that command succeeded.
- Asymmetric (Server -> Client)
    - Server sends a message to the client with no expectation of response. For instance, when a new device connects to the server, the server will tell the client the device has been added, but the server doesn't expect the client to acknowledge this. These messages are considered fire and forget.

Symmetric interaction between the client and the server may be a very, very long process. Sometimes 100s of milliseconds, sometimes possibly even multiple seconds if device connections are very poor. Client APIs try to deal with this via usage of Async/Await, assuming this capability is available in the language you have chosen to use.

For Asymmetric messages, first-class events are used, where possible (i.e. Javascript, C#). Otherwise, callbacks, Promises, or Futures are used depending on library capabilities. (i.e. python, C/C++, Rust).

:::: tabs

::: tab C#
<<< lang=csharp @/examples/csharp/AsyncExample/Program.cs
:::

::: tab Typescript
<<< lang=typescript @/examples/typescript/src/async-example.ts
:::

::: tab Javascript
<<< lang=javascript @/examples/javascript/async-example.js
:::

::: tab Twine
<<< lang=html @/examples/twine/async-example.twee
:::

::: tab Rust
<<< lang=rust @/examples/rust/src/bin/async.rs
::::

## Dealing With Errors

As with all technology, things in Buttplug can and often will go wrong. As stated in the ethics section, due to the context of Buttplug, the user may be having sex with/via an application when things go wrong. This means things can go very, very wrong. 

With that in mind, errors are covered before anything else.

Errors in live Buttplug sessions come in the following flavors.

* *Handshake*
    * Client and Server connected successfully, but something went wrong when they were negotiating the session. This could include naming problems, schema compatibility issues (see next section), or other problems.
* *Message*
    * Something went wrong in relation to message formation or communication. For instance, a message that was only supposed to be sent by a server to a client was sent in the opposite direction.
* *Device*
    * Something went wrong with a device. For instance, the device may no longer be connected, or a message was sent to a device that has no capabilities to handle it.
* *Ping*
    * If the ping system is in use, this means a ping was missed and the connection is no longer valid.
* *Unknown*
    * Reserved for instances where a newer server version is talking to an older client version, and may have error types that would not be recognized by the older client. See next section for more info on this.

The above types only apply to clients that have connected to a server. Custom exceptions or errors may also be thrown by library implementations of Buttplug. For instance, a Connector may throw a custom error or exception based on the type of transport it is using. For more information, see the documentation of the specific Buttplug implementation you are using. 

:::: tabs

::: tab C#
<<< lang=csharp @/examples/csharp/ExceptionExample/Program.cs
:::

::: tab Typescript
<<< lang=typescript @/examples/typescript/src/errors-example.ts
:::

::: tab Javascript
<<< lang=javascript @/examples/javascript/errors-example.js
:::

::: tab Twine
```html
error example
```
:::

::::

**NOTE:** You may notice that there's no way to tell exactly what an error is from this message. You get a class, but the information itself is encoded in the message, which is not standardized. Therefore it's impossible to tell whether a device disconnected, or you just send a connected device an incorrect message. This is bad, and [will hopefully be fixed at some point in the future](https://github.com/buttplugio/buttplug/issues/70).
