## Glossary

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
