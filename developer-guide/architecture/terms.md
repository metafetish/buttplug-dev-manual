## Glossary

Just in case you don't want to go digging through the architecture section again, here's a quick list of common terms.

* **Client**
    * The part of Buttplug implementations that applications use in order to access servers. Client APIs are what most Buttplug applications developers will see.
* **Connector**
    * Used by a client/server so it can talk to the corresponding pieces in some way. This could be embedded (other side in same program), or via networks (i.e. websockets), ipc (i.e. pipes), or other mechanisms.
* **Device**
    * A device is the general term for anything that buttplug connects to and controls. Sex toys, gamepads, estim units, fucking machines, whatever. Inside the library, it refers to either a representation of the hardware (in the client), or the actual code to talk to the hardware (in the server).
* **Message**
    * Buttplug messages are defined in the [Buttplug Spec](https://buttplug-spec.docs.buttplug.io), and are how Buttplug Clients and Servers communicate with each other.
* **Server** 
    * The part of Buttplug implementations that manages device connections and communication. This may be a standalone server, or may exist inside an application that uses Buttplug. Those wanting to add implementations for new devices will do so in Buttplug Server code.
