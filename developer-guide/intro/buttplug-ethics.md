# Butts Are Difficult

> "&#x2026; Society has put a lot of evils in our brains that we need to transcend before we make out." 
>
> [Cex - Not Trying](https://youtu.be/ONi7QwYNQz4)

No, really. For as much fun as butts and their surrounding body parts can be, applying technology to them is just a nightmare of conflicting issues and requirements and considerations that's absolutely exhausting to think about for too long.

So I'll try to keep this short.

Before we delve into what Buttplug is from the technical side, I'd like to take a minute to think about it from the social and ethical side. It's impossible to solve everything that may come up when using this library in a single page of a developer guide (it'd still be a tall order at a million pages. Thank Turing and his infernal machines for that), but I can at least go over a few things to think about while developing sex tech apps, other than whether your threads are deadlocking or memory is leaking.

## Empathy for the User Having Sex With Your Software

Let's start with the user of the software you're about to create.

When working on Buttplug applications, something that should be at the forefront of your design thinking is:

> Someone is going to fuck this.

The context of this usage should influence all levels of design, from UI/UX to low level code decisions. Assuming the user will approach a piece of software that involves Buttplug in the same way they would, say, a word processor, will end up in a fuckable word processor. If that's what you were aiming for, great, but otherwise this will just end up in a frustrated user and something that looked like a cat walked across the keyboard.

As you build projects using Buttplug, ask yourself questions that the user will encounter:

- Could I use this while turned on?
- If I'm REALLY turned on, how long does it take for me to go from "I wanna use this" to "I am using this"?
- What happens if my hardware disconnects?
- Do I have a quick way to stop whatever the hardware may be doing?
- Can I control this while possibly covered in [lube or other things]?

Since Buttplug programs are meant to be sexual, there's a good chance the user *won't* be considering these questions before embarking on whatever technical adventure you've implemented for them. Therefore it's your job to think about these issues first.

## Github After Dark

Developing sex software isn't a practice with much best practices or documented history or even a community of developers willing to admit they'd do such a thing. While we hope to change all of those with this library, that's going to take time. In order to gain acceptance in larger software communities, we hope to share our code in public forums, such as GitHub. 

These hopes have to be tempered by the issues of the general sterility of software, though. GitHub, StackOverflow, Glitch, and other community sites were not really made with NSFW content in mind. While there are certainly "adult" projects on GitHub in the form of porn site scrapers, adult games, and other applications, it's still not the norm by any means. How you present your sex software project as the community grows could end up setting standards for how services deal with NSFW code projects in the future.

Most of the code in Buttplug library isn't extremely explicit, but applications implementing Buttplug may be. Be careful with anything involving media assets that may be deemed inappropriate for certain age groups, especially on sites that don't allow search removal or age checking. Self hosting is always an option for projects involving sensitive materials.

If you would like to use a project site for Buttplug work, and they don't have an obvious policy stating how they might feel about their site being used with sensitive materials, it may be in your best interest to contact them. If you don't feeling comfortable handling this yourself, feel free to [file an issue on the main Buttplug repo](https://github.com/buttplugio/buttplug/issues) and we'll be happy to reach out and discuss. The Core Buttplug Developer Team has communicated with services in the past to figure out best practices for hosting sensitive code content, and the outcomes are usually positive.

Additionally, we highly recommend that any open source or community project using Buttplug should also have a Code of Conduct. While there have been many lively discussions on projects like databases and kernels adopting CoCs, there are some very concrete contextual reasons to have them for sex software projects. This rings especially true if it is a project that may involve some sort of generic, multiple-community/interest use.

For instance, let's say someone is writing an audio player/movie player that controls sex toys with Buttplug. A massive variety of media could be fed into this application, and there is a good chance that media used by some users may be found offensive by others. These user groups will still need support, and may possibly be sharing the same issue/bug tracker for their needs. Having a CoC in place guides moderation of situations where interests may conflict.

As for which CoC to use (if looking for a prewritten one), you can [check out ours](https://github.com/metafetish/metafetish-project-docs/blob/master/CODE_OF_CONDUCT.md) as an example. It's really just a slightly modified version of the [Contributor's Covenant](https://www.contributor-covenant.org/). Using stock CoC's on sex software projects can prove difficult due to rules about "appropriate wording", due to the sexually explicit context of the project itself. Addendums or rewording may be required, though we do recommend being cautious in how these are presented. We welcome discussion of these issues on [our message boards](https://metafetish.club) or [discord server](https://discord.buttplug.io).

Finally, you should consider if and how you'll handle contributions to your project. Regardless of whether you're running the project under your own identity, through a pseudonym, or anonymously, you shouldn't expect your contributors to all make the same choice as you did. Be ready to consider situations where someone wants to contribute but may need to use an anonymous account to do so, or may want to use their real and/or well-known identity, and how that may affect the optics and upkeep of your project.

## Mistakes Will Be Made

Now that we've covered users and services, let's focus on you, the developer.

The theme here will be as it was in the other sections: Plan ahead. Making a mistake that ends in something like data loss or crashing programs is one thing, but with sex software, there's just as likely to be social/ethical problems around technically competent implementations. How you respond to these problems affects not only your project, but the field of sex tech in general. Isn't responsibility fun?

This might seem like putting way too much burden on someone making a tiny vibrator app with a new interface, but context outweights technology here. Technology as applied to sex means people will concentrate on the sex more than the technology when discussing the topic. This is why security breaches in commercial sex tech seem extra bad, even if they may impact far fewer users than a large, non-sex related technical service being hacked. As it goes with larger companies, so it will go with smaller projects. A small project that makes some sort of mistake around sex tech may still see more fire than, say, someone's reimplementation of an algorithm or database or something. Social context matters.

Does this means you shouldn't develop for sex tech? Absolutely not. It just means you should treat it with more caution and planning than you might normally for less "interesting" software projects (and for everyone thinking "but most software is devoid of usage context and could be perverted for whatever reason!", yes I get it but I want to keep this section short, remember? So just go with it here.). 

As you'd think your design through for your users based on the criteria from the first section of this section, think your design through for yourself too. What is it you want to be responsible for with the software you're releasing? What do and don't you want your users to be able to do with it?

Note that this list is maleable. You don't have to get it right the first time, and it doesn't need to be formally stated for most smaller projects. It can grow and change as your software grows and changes, but having the stated requirements will help both you and your users in case things ever "blow up".

## The Buttplug Mission Statement

Buttplug isn't a "smaller" project though, so we do get to take on the formal statement.

For Buttplug as a project, it felt best to sum up all of the information in this section as The Buttplug Mission Statement (written by someone far smarter than me after I said it far more verbosely and with a bunch of handwaving and probably some cursing).

> Buttplug is committed to the safety, autonomy, and human rights of people using it as a sex technology standard, and stands in solidarity with the many intersectional rights of all individuals to be sex positive. As such, Buttplug encourages individual empowerment through self-directed education, and responsible behaviors which are also respectful of the needs and the choices available to everyone.

Getting the paragraphs in this section (plus a bunch of stuff not covered) squeezed into an almost-tweetable chunk means using dense wording that may sound odd for a project named "Buttplug", but this is a Load Bearing Mission Statement. It has a lot to explain and contextualize in a small space, and it does what the project needs. As your project may be more specific than "abstract hardware controller", odds are you may not need something of this manner, but it's good to at least think about what you'd say to sum up what it is you're doing and why you're doing it, before you actually have to do so.

## Mo' Butts Mo' Problems

This section, at best, should provide a framework about how to think about things as you develop your application. Your experience will be unique, and may require you to come up with your own strategies to continue development and distribution in a way that works for you and/or your community.

If you have any suggestions, please feel free to reach out via the contact info on the home page of this document! I'd love to hear about how developers create and adapt tech as they need for their users.