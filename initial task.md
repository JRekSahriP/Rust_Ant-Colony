# [This idea was generated by ChatGPT]

### Activity: Ant Colony Simulation System

#### Context
You have been hired to create a simulation of an ant colony. Each ant performs tasks such as foraging for food, protecting the nest, or expanding the tunnels. The colony must share certain resources, such as the food stock and tunnels.

This simulation will require the use of smart pointers to manage shared data and ensure that each ant can safely modify the resources.

#### Requirements

1. Colony Structure:
   - The colony should contain:
     - A food stock (u32).
     - A map of tunnels (represented as a list of tunnels, where each tunnel is identified by a `String` name).
     - A list of ants.

2. Ant Structure:
   - Each ant should have:
     - A unique name (`String`).
     - A role in the colony (enum `Role`):
       - `Forager`: Forages for food and adds it to the stock.
       - `Guard`: Protects the colony (simulated with log messages).
       - `Builder`: Builds new tunnels.

3. Use of Smart Pointers:
   - Use `Rc<RefCell<T>>` to share the state of the food stock and tunnel map among the ants.
   - Each ant must be able to safely access and modify this data.

4. Program Features:
   - Create a colony with an initial food stock and one starting tunnel.
   - Add ants with different roles to the colony.
   - Simulate ant actions:
     - Forager: Adds food to the stock.
     - Guard: Displays a message indicating it is protecting the nest.
     - Builder: Adds a new tunnel to the map.
   - Display the state of the colony after the actions.

#### Additional Challenges
1. Allow the food stock to decrease if there are no ants foraging for food.
2. Implement a capacity limit for the number of tunnels (e.g., a maximum of 10 tunnels).
3. Add a system of random events, such as an attack that reduces the food stock.