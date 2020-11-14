# Strategies Against Buttplug Architecture

In this section, we'll cover what makes up Buttplug as an abstract architecture. Getting a hang of the definitions and contexts here is important, as these terms will be used throughout the rest of this guide.

All of the concepts here are abstract. While we do have "reference implementations", we do not require any specific programming language or serialization type for implementing the protocol or client/server architecture of Buttplug. Go wild. Surprise us. Make a Haskell version of all this that really is one big IO Monad joke. Make an INTERCAL version because there's already power play built into the language. Make a C version because&#x2026;

Ok you know what maybe don't make a C version. Do you really want to put C in your butt?

## A Note for Those Who Already Read The Spec

All of the information in the Architecture portion of this document is also in the [Buttplug Protocol Specification Introduction](https://buttplug-spec.docs.buttplug.io), in a more concise, somewhat less friendly form that may be more familiar. If you've already read and understood [the spec](https://buttplug-spec.docs.buttplug.io), you can maybe skip this section.

## The 3 Genders: Server, Client, Application

There are 3 distinct conceptual parts to Buttplug. How all of these exist (as separate programs, all in the same program, or some other combination) are up to the needs of the developer and their specific application, which will be discussed more in the [Writing Applications section](writing-buttplug-applications.md). However, when we talk about these parts in documentation and implementation, we keep them as separate ideas.

### Buttplug Servers

A Buttplug Server is the piece that actually talks to hardware. This is usually via Operating System Specific libraries or APIs. It handles coordination of device connections/disconnections, and manages which clients can talk to hardware. A few examples of jobs the server has:

-   The server is in charge of finding devices connected to the computer, be they bluetooth, usb, serial, firewire, parallel, or whatever other device communication type is supported by the server implementation in question.
-   The server contains the knowledge of how to talk to a specific toy in the way that it understands. Toy protocols are rarely shared between different brands, so many times we must translate to 10s of different protocols.
-   If a client is controlling a device, then for some reason disconnects, it is the server's job to stop the device until the client has reconnected and sent new control commands.

The server may be in charge of other tasks too. At a bare minimum, a server should handle all non-device messages in the Buttplug Protocol spec.

### Buttplug Clients

Buttplug Clients are what developers use to talk to Buttplug Servers. Clients are responsible for messages staying synced between the developer's code and the servers. They may also expose devices and interfaces in a language specific way that the developer is used to working with. Clients are covered in depth in the [Writing Applications section](writing-buttplug-applications.md).

### Applications

Applications put some sort of specific UI/UX in front of a Buttplug Client. This could be:

-   [A simple slider to control a toy from a web page](https://playground.buttplug.world)
-   [A 3D game](https://www.youtube.com/watch?v=rAYdo1vDNak)
-   [A typing tutor that makes the toy vibrate more as you type words in correctly.](https://curiousjp.itch.io/caveman-bios-teaches-erotic-typing)

The ideas here really are endless. All of these will use a Buttplug Client to talk to a Buttplug Server.

## Configuration Examples

There are multiple configurations and possibilities available, depending on the programming language, operating system, and hardware platform the developers and users choose.

Once again trying to limit specifics, here's a couple of examples of how these pieces might go together:

-   Someone builds a movie player application that is just one executable, containing both a Buttplug Server and Client. This allows them to use the Client API, which makes accessing the server easy, while also meaning users only have one thing to install and don't have to worry about connecting to outside programs.
-   Someone builds a web app to control sex toys. The web browser does not support a way to access the sex toy hardware. The web app will contain a Buttplug Client that talks over the network to a native Buttplug Server, which has the ability to talk to the hardware.
