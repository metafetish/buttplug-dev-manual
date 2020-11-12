# Device Control

We've connected, we've enumerated, now it's time for the important stuff. Device Control!

## Device Capabilities

The devices Buttplug supports can do many different things. They may vibrate, stroke, rotate, stimulate via electricity, some combination of all of these, or possibly something completely different.

In order to trigger these different mechanisms, Command Messages are used. These messages all end in "Cmd". For more information on all the messages available,

When a device is added, it comes with a list of messages it can accept. For instance, if you have a vibrating Buttplug (an actual buttplug toy), it may accept the following messages

- VibrateCmd
- StopDeviceCmd

More information on available Command Messages can be found in the [Buttplug Protocol Spec](https://buttplug-spec.docs.buttplug.io).

For information on when and how to use command messages to get the desired effect you want, check out the Cookbook section of this guide (Coming soon).

If you can't find messages you need to control your device, check the [Buttplug Spec Issues page](https://github.com/buttplugio/buttplug/issues). If no issues are listed that cover your needs, feel free to file a new one and we'll discuss it!

## Sending Device Messages

As a user of a Buttplug Client API, you should never be expected to send raw Buttplug Messages. Most Client APIs will provide message sending functions for you, usually attached to device objects or structures. If the device accepts the message type represented by the function you call, it should be sent to the device. Otherwise, it will fail.

<CodeSwitcher :languages="{rust:'Rust', csharp:'C#', ts:'TypeScript', js:'JS', twine: 'Twine (Sugarcube)'}">
<template v-slot:rust>

<<< @/examples/rust/src/bin/device_control.rs

</template>
<template v-slot:csharp>

<<< @/examples/csharp/DeviceControlExample/Program.csharp

</template>
<template v-slot:js>

```js
// Need to write this example
```

</template>
<template v-slot:ts>

```ts
// Need to write this example
```

</template>
<template v-slot:twine>

```html
<!-- Need to write this example. -->
```

</template>
</CodeSwitcher>

