
## Proposal for 2048 (GUI)

### Group Members

1. ndunkel
2. jthgil12
3. flourtowel

![](http://i.imgur.com/c1qgVLY.jpg)

### Objectives

* Use piston to write a GUI for 2048
	* 2D Rendering
	* User ineraction
* Development of iOS implementation (possible extra credit)
	* Using rust and swift
	* Utilizing touch screen
	
## Details

### Use piston to write a GUI for 2048

We want to use the suggested project idea of implementing a GUI for 2048 in rust using piston. And the piston 2048 GUI as inspiration. We want a to be able to use the arrow keys up, down, left, right and generate the grid on a computer. Once that is accomplished we will keeps going and try to create a GUI using rust for iOS. 

### Development of iOS implementation 

We’re going to end up with a simple but nontrivial app. The trick is that we’re going to put all the smarts in the Rust layer. We’ll roughly follow an MVVM (Model—View—View Model) architecture where the Model and View Model layers are implemented in Rust, and the iOS side is just the View layer. 

(bignerdranch.com)

### Here’s the plan

* Getting Started with Rust on iOS 
* Passing Data between Rust and iOS
* Sharing a View Model between Rust and iOS
* Writing a GUI in Rust
* Tying it All Together

### Article giving more details
* This [article](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-1/) explains the process we're going to try and follow to use rust in iOS.

**GitHub links from the above article**

* [Part 1](https://github.com/bignerdranch/rust-ios-app-part-1) 
* [Part 2](https://github.com/bignerdranch/rust-ios-app-part-2) 
* [Part 3](https://github.com/bignerdranch/rust-ios-app-part-3) 

## Milestone

We would like to as close as possible to finished with the coding for the computer GUI version and for the rest of the semester we will work on the iOS verison. 

