# Foreword

A few words from the author of this guide and architect of Buttplug.

## Wants and Needs

Before we jump into all of the descriptions and eccentricities and whatnot that make up Buttplug, let's start with a parable.

Have you ever bought a new thing you've been really excited about, but then it turns out that it didn't work as expected for you?

Maybe used some software or played a game that almost gets there but doesn't quite work for you?

Here's a story from my days as a virtual world developer, about what this situation looks like from the designer's side.

## I Want To Be A Shirt

"I want to be a shirt."

"A what?"

"A shirt."

"Like, you want a shirt with a picture of you on it or something? You can already do that you know."

"No. I want to *be* a shirt. I want to exist as the piece of clothing. That piece of clothing should be a shirt. I get off on the idea of being a shirt. It has been a dream of mine and we're here for dreams, right?"

Having just spend a non-trivial amount of time lovingly crafting a new type of genitals, with all sorts of bits and bobs and extras, the last thing you want to hear is that your creation is not appreciated. Building something like this that does everything it's supposed to AND communicates across multiple dimensions is not easy, and yet, according to this critic, it has completely missed the mark.

"Why do you want to be a shirt? How would this be better than having some new genitalia? Look, it even vibrates when you poke it like this!"

"Well, yes. That's very good. It even goes all red and bulgy, a nice touch. But&#x2026; I don't know. It's just always been a thing I've wanted to be. I would much rather fulfill my needs via being a shirt, than to graft on some new body part I'm not even sure I'd be in to."

"And you want people to wear you?"

"That'd be the idea, yes."

Damnit. Damn. It. Everyone was supposed to want one of these new body parts, what with the sounds and the arousals and the thrustings and the juices. Sure the inter-dimensional communication part would be a tough sell due to the learning curve, but once everyone caught on, it'd be a new paradise.

That was the plan, and it'd be awesome, and popular, and there would be much rejoicing. Yet, here we are.

"But you can't be a shirt."

"Why can't I be a shirt?"

"Because that would violate laws. A lot of laws."

"What? It's not illegal to be a shirt anywhere that I know of."

"Not like, legal laws. Physical laws. Laws of existence."

"But don't you control those kind of laws? Or don't some of the others like you have that power?"

DAMN. IT. Note to self: never create worlds that can contain anything capable of making a good point.

"I suppose we kind of do? But if I change those laws for you, I have to change them for everyone, and I don't think everyone wants to be a shirt. Or pants. Or a shoe. Much less to derive gratification from being any of those."

"Why wouldn't other people want to be shirts? There's so many possibilities. I would be a continual soft hug. I would adorn the wearer and make them look nice. I would absorb their sweat."

"We didn't give you sweat glands. How would you even pull that off?"

"I'd find a way. We've got our own tools down here too, you know. They're just not powerful enough to let me be a shirt."

"Yeah there's a reason for that. What about if you tried to&#x2026; Ugh, is there even a verb for this? What if you tried to&#x2026; shirt someone and they didn't know you were a living shirt or whatever it is you're asking for here. Think about the security issues."

"I think it'd be pretty obvious 'cause I'd be talking."

"Oh so you want to be a TALKING shirt? Anything else on this list of demands?"

"Ability to change cloth type? I'd want to stay current with the trends."

"Forget I asked. And besides, not everyon&#x2026; everyshirt would talk. We already have to deal with enough chaos around here without adding 'Non-consensual shirtings' to our list."

"Hey, you're the one building this world. You wanted feedback on how to do that, and I'm giving it to you."

"You are the worst focus group."

"You randomly asked the first person you saw. Blame fate, not me."

A quiet voice pops up from a few yards, or miles, over. Scale is difficult when you're looking down from above.

"If they get to be a shirt can I be a cube?"

Fuck.

## Ceci n'est pas un Buttplug

The preceding story actually happened.

Sure, it was in the context of the Second Life virtual world, and the narrator (me) was less god and more employee, but the events happened as stated. Someone really wanted to be a shirt, and due to the constraints of the software, it couldn't happen. This was despite me, as a software engineer (who before being hired, had made real life sex toys work with Second Life) having the ability to control the virtual world, albeit in a manner limited by software development time, project specifications, and other obstacles both gods and engineers have to deal with.

There are a lot of people out there that want to be shirts, at least, in the terms of this horribly tortured metaphor. They've bought a sex toy, and the interface provided to them to control it doesn't work for them for some reason, or the content isn't to their liking. Buttplug (the software, not the sex toy genre) was created for that situation.

Buttplug is nothing without interface applications. The only real function of Buttplug is make it easy for developers to get computers to talk to sex toys. The software those developers make will use Buttplug to talk to the sex toys.

Interface applications are the link between users and Buttplug, establishing the functionality the user was missing. Application developers, those making the interfaces, are the shirt makers (and thus, the metaphor is dead).

There are a many considerations, both technical and social, that need to be kept in mind when creating applications that will interact with sex toys. It is assumed that the sex toy will be somehow attached to a person, who will most likely be engaged in the act of sexing. This is not a situation normally covered in user interface design or software/hardware engineering textbooks. It is impossible to outline all of the relevant situational use cases. The goal of this document is to provide a framework by which design decisions about specific application needs can be informed.

Building, releasing, and using software with the kinds of contexts that are inherent in Buttplug involves risks. This document is what I've learned so far about sex toy control and interface design, and how to apply that when using or developing for Buttplug. It contains the lessons I've learned over the years so far, and will be the home for lessons learned in the future.

## Why Do I Need You To Tell Me Where My Butt Is

But who am I and why does my view on this information matter?

Probably should've led with that, huh.

Well, um, hi, I'm Kyle Machulis, lead architect of Buttplug. Since 2004, I've been researching and blogging about sex tech (via the now defunct [metafetish.com](https://metafetish.com)). I've followed online communities, talked to hundreds of people involved in many different activities, fetishes, and interests, and built many experiments to try to figure out if/how/why technology will/won't work in situations presented to me.

The information presented here comes from over a decade of my amateur research on the sex technology field, from the engineering and user interface perspective. While I strive to provide as much help and information as possible, I am an engineer, not a sexologist, psychologist, sociologist, ethicist, or one of those other -ists that deals directly with people and people issues. It is well known that engineers may not be real great at "people" (to put it lightly). With that in mind, I've tried to consult with many people that are those aforementioned -ists while building this software and writing this document. This project was by no means done alone, nor could it have been done alone to a quality I would've been satisfied with. The names of those brave enough to be associated with this work are listed here:

-   Should probably figure out who wants to be in this list.

When writing sex software, it's rather hard to avoid dealing with people, unless the software is never actually used. While the engineering portions of this document will be as rigorous as possible, many of the observations about users and usage examples will be from my personal experience. These are by no means complete studies. All stories and examples presented are anecdotal at best, and complete fabrications at worst.

Building this project has been a weird, great, weird journey. I hope Buttplug helps you with whatever your wants and needs may be.
