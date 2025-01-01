## **Ants**  
In the game, there are three main types of ants, each with a specific role.  

### **1. Forager**  
- **Role**: Collects food for the colony.  
- **In the game**: Each **Forager** ant collects a random amount of food based on the number of tunnels in the colony. The maximum amount of food collected is directly influenced by the number of tunnels present.  

### **2. Builder**  
- **Role**: Expands the colony's tunnels.  
- **In the game**: Each **Builder** ant increases the number of tunnels in the colony.  

### **3. Guard**  
- **Role**: Increases the colony's defense against threats.  
- **In the game**: Each **Guard** ant raises the colony's defense level, helping to reduce the impact of daily threats.  

---

## **Food**  
Each day, ants consume food as follows:  

- Every ant consumes 2 units of food per day.  
  - For example, if the colony has 5 ants, the daily consumption will be **10 units** of food.  

### **Creating Ants**  
When a new ant is created, it consumes an amount of food **based on the total number of ants** in the colony.  

---

## **Threats**  
- **Every day, a random threat is generated**, based on the total number of ants and tunnels in the colony. The threat is a random number ranging from 0 to **1 + the total number of ants and tunnels**.  
- **Defense**: The colony's defense level is determined by the number of **Guard** ants. Defense can block or reduce the effects of the threat.  
  - If the threat is **greater than the defense**, the colony suffers damage.  
  - If the threat is **less than or equal to the defense**, the colony takes no damage.  

### **Effects of Threats**  
- If the defense is insufficient to counter the threat, the following may occur:  
  - **Loss of food**: The colony may lose 5 units of food.  
  - **Loss of tunnels**: One tunnel may be destroyed.  

- If the defense is sufficient to counter the threat, the defense level will be reduced by the threat value.  

---

## **Tunnels**  
- **Each Builder ant** creates a new tunnel every day. The total number of tunnels provides more food for the ants and affects threat generation.  

---

## **Daily Cycle**  
Each day cycle consists of the following actions:  

1. **Ant Actions**:  
   - The ants perform their roles:  
     - **Foragers** collect food.  
     - **Builders** construct new tunnels.  
     - **Guards** increase the colony's defense level.  

2. **Threat Check**:  
   - A random threat is generated and compared to the colony's defense level.  
   - Depending on the result, the colony may lose food or tunnels.  

3. **Food Consumption**:  
   - All ants consume 2 units of food per day.  
   - If the food stock reaches 0, the colony dies of starvation.  

---

# **Commands**  
- **ants list | ants | list | as**: Lists all ants in the colony.  
- **ant new | new | an**: Creates a new ant.  
- **state | colony | s**: Displays the current state of the colony (food, tunnels, and defense).  
- **next | n**: Advances to the next day.  
- **help**: Displays the list of commands.  
- **quit | exit**: Ends the game.  