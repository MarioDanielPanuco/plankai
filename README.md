I'd like to thank Patrick Wychowaniec for making the Shorelark tutorial on the neural network and genetic algorithm in Rust that provided the foundation for this codebase.

His work can be found at can be found at [Github.com/Patryk27/shorelark](https://github.com/Patryk27/shorelark).

I'm treating this library as a test lab for modeling an agent or agent in complex dynamical systems.

# Plankton Agents
Agents act to maximize their reward function which is influenced by their percept of the environment's state.


### TODO
    * Adopt the rust-nalgebra library for the neural network as an investment towards a Rust GPU
        utilizing linear albegra libary
    * Tournament selection - DONE 
    * Boltzman selection - 
    * Statistics Library and Neccessary Import
    * Integration of egui crate

### Inspiration
I found out that plankton don't have much agency, the name is ironic, but it made me think more of agency and the use of actuators by invertebrate and lower vertebrate biological organisms.
This doesn't influence the research for the codebase much, but gave me the idea to make a codebase that supports complex agent-environment states. 

### Disclaimer 
Accidentally removed /www directory from the repository, which handles turning simulation-WASM into a
webpage, currently looking for a fix. Might have to create new repository to host project. 
