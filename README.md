I'd like to thank Patrick Wychowaniec for making the Shorelark tutorial that provided as the foundation for this codebase. 

His work can be found at can be found at [Github.com/Patryk27/shorelark](https://github.com/Patryk27/shorelark).

I'm treating this library as a test lab for modeling an agent or agent in complex dynamical systems.

# Plankton Agents
Agents act to maximize their reward function which is influenced by their percept of the environment's state. 


### TODO
    * Adopt the rust-nalgebra library for the neural network as an investment towards a Rust GPU
        utilizing linear albegra libary
    * Tournament selection - DONE 
    * Boltzman selection
    * Add mapping to regions in the space, to simulate adding more complexity to a position in the environment.
    * Implement Statistics and Probablity library
    * Export statistics as a csv file 

### Inspiration 
I found out that plankton couldn't swim, so I thought about their agency and lack of actuators on their environment.
This doesn't influence the codebase, but gave me the idea to make a codebase that supports complex agent-environment states. 
