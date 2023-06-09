
# Rust (Programming Language)

The Rust Programming Language

## Introduction

Rust is fast, safe, and concurrent modern system programming language.

And it is my main preferring language.

As you can see, the followings are Rust's three keywords:

* fast
* safe
* concurrent

Let's check them one by one.

## Fast

Rust is a system programming language, which is meant to be fast.

Rust has a trait which is called "zero-cost abstraction".

It means a programmer can enhance his productivity with simplified code,
without sacrificing the performance.

This is huge for performance-sensitive fields.

## Safe

Rust is a modern programming language, which cares about the safety.

How can a system programming language be fast? It's because Rust learnt from the past.

Currently, C and C++ are being used in the system programming field majorly.

They contributed lots of portion to the fundamental core of IT industry,
but at the same time, they are very old programming languages.

I do not mean that being old is bad as itself.
But they are not just old.
They are programming languages which are born in very early ages of programming.
And that means that it's prone to have some design fallacies.

To prevent that kinds of fallacies,
Rust is designed with very deep contemplation.

They key feature is the ownership system.

In Rust, by default, variables do not just clone implicitly.
Unless they have 'Copy' trait implemented,
if you pass the variable to some function,
the basic behavior is 'moving', not 'cloning'.

If a programmer try to reuse the variable,
which has been moved lately,
the compiler will show the error and wouldn't try to compile it.

It is true that kind of trait makes the compilation hard to be successful.
However, once you have the code compiled,
the program will be significantly harder to find errors,
compared to other unsafe languages like C or C++.

That means Rust's design choice was meaningful.

## Concurrent

Rust's ownership system is also effective for the concurrency.

As the demand of performance rises,
the concurrency has become essential feature of a program.

However, writing a concurrent program in the unsafe language
is way harder than writing a single-thread program.

Since multiple threads are racing for same resources,
it is very easy to write an error-prone program.

However, Rust's ownership system makes the task much easier.
Rustaceans, which means 'Rust programmers',
called the trait the 'fearless concurrency'.

That is why Rust is being called 'fast, safe, and concurrent' language.

## Conclusion

Sadly, Rust also has some downsides.
The biggest part is the steep learning curve.

The way of programming in Rust is way too different from other programming languages.

To achieve the traits that I described above,
Rust has no choice but to have lots of design restriction.

Because of that, learning Rust can be not rewarding.
Maybe you might end up with just learning some Rust's syntax,
not ending up with few successful projects.

I cannot force you to endure it.
But only I can say is "it can be rewarding in the long-term perspective".

Rust's grammar gives lots of insight with just itself.
You might have troubles to understand that kind of choices.
But if you search up for the reason, you can figure out why.

And that kind of insight can help you write a better program,
even if you use other programming language.

That is why I recommend learning Rust.

But honestly, the portion of Rust usage in the IT industry is not that high.
However, many programmers can companies find that Rust is worth trying.
And the community is also growing fast.

Maybe the future can be different, while I'm not sure about that though.

I hope the Rust's popularity rise.
