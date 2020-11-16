# The 3 Genders: Server, Client, Application

## Components

Systems that use Buttplug will generally work with the following 3 components.

### Buttplug Servers

A Buttplug Server is the piece that manages communication with hardware. This is usually via Operating System Specific libraries or APIs. It handles coordination of device connections/disconnections, mapping protocol implementations to connected hardware, and maintaining communication with the currently connected client. A few examples of jobs the server has:

-   The server is in charge of finding devices connected to the computer, be they bluetooth, usb, serial, firewire, parallel, barbed wire, or whatever other device communication type is supported by the server implementation in question.
-   The server contains the knowledge of how to talk to a specific toy in the way that it understands. Toy protocols are rarely shared between different brands, so the server contains many different implementations.
-   If a client is controlling a device, then for some reason disconnects, it is the server's job to stop the device until the client has reconnected and sends new control commands.

The server may be in charge of other tasks, which we'll cover in depth in the next section. At a bare minimum, a server should handle all non-device messages in the Buttplug Protocol spec.

### Buttplug Clients

Buttplug Clients are the usable API surface of Buttplug, what developers use to talk to Buttplug Servers. Clients are responsible for messages staying synced between the developer's code and the servers. They may also expose devices and interfaces in a language specific way that the developer is used to working with.

### Applications

This is the part you're most likely going to be building!

Applications put some sort of specific UI/UX in front of a Buttplug Client. This could be:

-   [A simple slider to control a toy from a web page](https://playground.buttplug.world)
-   [A 3D game](https://www.youtube.com/watch?v=rAYdo1vDNak)
-   [A typing tutor that makes the toy vibrate more as you type words in correctly.](https://curiousjp.itch.io/caveman-bios-teaches-erotic-typing)

The ideas here really are endless. All of these will use a Buttplug Client to talk to a Buttplug Server.

## Component Configuration Examples

There are multiple configurations and possibilities available, depending on the programming language, operating system, and hardware platform the developers and users choose.

Once again trying to limit specifics, here's a couple of examples of how these pieces might go together:

-   A developer builds a movie player application that is just one executable, containing both a Buttplug Server and Client. This allows them to use the Client API, which makes accessing the server easy, while also meaning users only have one thing to install and don't have to worry about connecting to outside programs.
-   A developer builds a web app to control sex toys. The web browser does not support a way to access the sex toy hardware. The web app will contain a Buttplug Client that talks over the network to a native Buttplug Server, which has the ability to talk to the hardware directly.

## Protocol

As we've said before, Buttplug is both a protocol and a system of components. The Buttplug Protocol is the language that all of the components use to talk to each other. We'll cover it in depth in the next section, then will go into the Client and Server Components. The rest of the developer guide is spent on Applications and Configurations.