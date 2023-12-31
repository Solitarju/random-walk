# random-walk

What is a random walk? <https://en.wikipedia.org/wiki/Random_walk>  
A random walk is a random process that describes a path that consists of a succession of random steps on some mathematical space.  

In the above source code the random walk starts out as a point on a 1920x1080 texture, it then generates a random direction to move on a grid of a specified size (default of 4px).  

The above program is in 2d space, which means the possible directions are: up, down, left and right. This process continues for a specified "steps" which determines how many times it will move in a random direction from the center origin point.  

The random walk will be generated when the program starts and can be regenerated with a Key press (Enter).  

How does it work?

- Generation  
The random walk is generated by looping for 0..STEPS iterations, on each iteration we generate one of 4 random integers (0, 1, 2, 3) which each determine a direction to travel. 
On each iteration, we push the random integer to a Vec<i32\> which is then returned. We do this multiple times depending on how many runs were specified (RUNS constant at the start of the source code), and in the end we return a Vector of random draws.

- Rendering  
Rendering isn't as straight forward as it seems, of course we could just draw the random walk every frame but when you scale this to a bigger random walk with more steps, performance drastically degrades. To combat this we use a stored texture and a boolean to check whether we need to draw on that frame, if we do, we draw the random walk onto the texture normally and switch the boolean back to indicate we don't need to repeat this process in the following frames. Once we have drawn the random walk onto the texture, we can just render the already drawn texture until the user decides to regenerate the random walk, which results on a performance increase of 150x (20fps to 3200fps) on my personal machine running an rx580 GPU and i5 3rd gen CPU.  

And that's the essentials of the program! Of course there are a few more features such as rendering the fps, mouse position and screen panning but, these features aren't so complicated, so I feel they don't need an explanation.