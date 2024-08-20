For vectors you always need to implement the Display trait yourself. 

Because of the orphan rule you have to wrap the trait into a unit struct, and then implement the Display trait for that unit struct.

**Note: The orphan rule means that you cannot implement a foreign trait for a foreign type.**
