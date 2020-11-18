# Buttplug Protocol

At the core of Buttplug is the Buttplug Protocol. This protocol is defined in the [Buttplug Spec](https://buttplug-spec.docs.buttplug.io).

::: tip Why is there a protocol specification?

It may seem a little silly to go to such formal lengths for a system for controlling sex toys, and it admittedly does introduce a level of formality that isn't seen outside of larger systems like web browsers. However, this serves a couple of functions:

- It allows developers to build their own implementations of Buttplug if they so choose, and know that they should be compatible with applications using Buttplug as long as the spec is followed.
- One of the goals of Buttplug is forward-compatibility. If an app implements Buttplug but then doesn't continue development, Buttplug Servers should be able to adapt the older protocol version to new messages and hardware. The spec creates a record to work with for these conversions, so we can understand what was happening when the app was originally developed, and what has changed since.

Forward compatibility is important here because when people find sex software they like, they will use it *forever*. There are VB6 and Java sex apps from the late 90's/early 00's out there still seeing use these days!
:::

## Messages

The Buttplug Protocol is made up of messages. 99.999% of the time, unless you're actually developing a Buttplug Client or Server, you will never see a raw Buttplug Message. The Client API takes care of forming them for you, and the Server is usually only accessed via the Client. However, for sake of knowledge, Buttplug Messages look like this (at least, in its serialized JSON format):

```json
{
  "Ok": {
    "Id": 1
  }
}
```

This is an "Ok" message, which the server sends to the client to signify that a message was received and processed successfully. It, and every other Buttplug Protocol Message, has an "Id" field, which is used to give messages context. The Client will choose an Id for a message, then when the server replies to it, it will use the same Id, so the Client knows which message the Server is replying to. As Buttplug deals with hardware that communicates at vastly different speeds, messages may be replied to out of order, so there's nothing saying that a message that's send first will get its reply first.

**All of this is taken care of for you in the Client API**, so you don't really have to worry about making any of this happen yourself. :)

## Client/Server Interaction

There are two types of communication between the client and the server:

- Symmetric (Client :arrow_right: Server :arrow_right: Client)
    - Client sends a message, server replies. For instance, when a device command is sent from the client, the server will return information afterward saying whether or not that command succeeded (in the form of the "Ok" message shown above.).
- Asymmetric (Server :arrow_right: Client)
    - Server sends a message to the client with no expectation of response. For instance, when a new device connects to the server, the server will tell the client the device has been added, but the server doesn't expect the client to ask for the message, or to acknowledge that it received it. These messages are considered fire and forget.

Symmetric interaction between the client and the server may be a very, very long process. Sometimes 100s of milliseconds, sometimes possibly even multiple seconds if device connections are poor or requires intensive processing.

How asymmetric message are dealt with depends on the capabilities of the programming language implementing the library. There may be callbacks, event handlers, streams, or something else entirely. We'll cover this more in the Writing Applications section.

## Message classes

There are multiple classes of Message in the protocol, including:

- **Status**: Relaying info about the status of the system, including whether a message was processed ok or errored out, whether the system has timed out and shutdown, etc...
- **Handshake**: Used to set up connections between servers and clients.
- **Enumeration**: Getting what devices are currently connected, or what is being added/removed. Addition/list messages also contain specific information about device capabilities.
- **Device**: Messages going to/from specific devices. These include commands, or reading from sensors on the device.

## Spec Versions and Message Additions

When reading Buttplug bugs or development posts, you may here that some functionality is in development and "messages will be added in the next version". Each version of the Buttplug Message Spec has a version number, and once that version is set, the only way to change the spec is to increase the version number. This is what allows us to handle backward compatibility, since we know exactly what messages to expect when a client connects to a server (as the client will declare the spec version it's using as part of one of its handshake messages).

Adding new functionality to Buttplug means adding new message, so when Buttplug doesn't do something that developers are looking for, it means we need to add a message to the protocol, and revise the spec version. Once the spec changes, we then modify the Client API and release a new major version of the libraries (spec changes always trigger major version revisions), so that developers can use the new message.

Message additions are a rather complicated process. Message addition procedure is outlined in the Inflating Buttplug section, but for now, just knowing these terms is enough so you can translate what project contributors are talking about when they discuss the spec and new messages.

::: tip Can Clients and Servers of different spec versions talk to each other?

Yes, in some cases.

This is how backward compatibility in Buttplug systems works. If a client has an older message spec version than a server, the server should be able to accommodate the client by translating messages from/to the older version. This is also why you may want to think twice before developing your own server implementation, as trying to get these translations right is quite complicated.

If the client has a newer message version than the server, then the connection will fail on handshake. This is because the server has no idea what the client may send it, and it's assumed that if its a remote server, then the user can probably update it to the latest version and fix this issue.

:::

## For More Info, Visit Your Local Spec

The above summary is all you really need to know about messages if you're going to build a Buttplug application. You can assume that most of the methods you'll use in the Client API for your chosen programming language are working behind the scenes to form a Buttplug message of some type and send it on to the server, then receiving messages from the server and turning them into either return values for methods, or events. But you shouldn't have to worry about the low level, and if you do, it's probably a bug.

However, if you want to work Buttplug libraries, it's definitely worth becoming more familiar with this part of the system. In that case, it's best to read the [Buttplug Protocol Spec](https://buttplug-spec.docs.buttplug.io) in order to understand message functions and flows.