VZO
===

![gif](https://user-images.githubusercontent.com/343936/152670686-ca67d6ba-c704-4448-bcbc-c21058559519.gif)

**VZO** is a VST plugin allowing DAW software to send OSC (Open Sound Control)
messages.

Dependencies
------------

- zeromq

VZO requires the zeromq library. You can install it using Homebrew:

```
> brew install zeromq
```

Usage
-----

The VZO package contains the following components.

- vzo.vst (VST plugin)
- bridge (command line app)

Add the VZO plugin to an instrument track on your DAW project. It captures all
note/CC events in the added track and send them to the bridge software. Then it
converts these events into OSC messages and resend them to an OSC receiver.

The default destination address/port pair is `localhost:9000`. You can change
it with a command line argument. For example:

```
> ./bridge 192.168.0.10:8000
```

OSC Message Specifications
--------------------------

### Note Events

- Address pattern: ```/note/{channel}/{pitch}```
- Attached data: float (velocity value)

You can set `{channel}` with the VZO plugin parameter. This is not relevant to
the MIDI channel number. You can set any value for identification purposes.

There is no note-off event; It sends zero-velocity events instead. You have to
check the velocity value on the receiver side.

### CC Events

- Address pattern: ```/note/{channel}/{CC#}```
- Attached data: float (CC value)

Related Projects
----------------

- VzoVfx: Unity package to control VFX by VZO messages.
