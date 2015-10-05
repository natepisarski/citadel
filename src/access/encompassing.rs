//! encompassing deals with splitting a list over scoped delimeters.
//! For instance, in the following line: `a{b{c{d}}}a`, the two a's are the
//! only two values not encompassed by the scope delimted by the curly braces.

/// Returns the sections of a list which are inside the scope of the specified delimters.
/// # Examples
/// ```
/// use citadel::access::encompassing;
/// let scoped: Vec<Vec<char>> = encompassing::encompassed_by("a{b{c}}".to_string().chars().collect::<&[char]>, ('{', '}'));
///```
pub fn encompassed_by<T: Eq+Clone>(list: &[T], (delim1, delim2): (T, T)) -> Vec<Vec<T>> {
    
    // Once current_collection's scope ends, it's copied here.
    let mut collected: Vec<Vec<T>> = vec![];

    
    // The current "pass" of the scope is collected here.
    let mut current_collection: Vec<T> = vec![];

    // How many "nests" deep the pass is.
    let mut nesting_level: usize = 0;

    for element in list {
        
        // A left delimter is found? Increase the nesting level
        if *element == delim1 {         
            let will_continue: bool = nesting_level == 0; // Don't collect the first delimiter
            nesting_level = nesting_level + 1;
            
            if will_continue {continue;}
        }
        
        // A right delimeter is found? Decrease the nesting level
        if *element == delim2 {
            if nesting_level - 1 > 0 {
                nesting_level = nesting_level - 1;
            } else if current_collection.len() > 0 {//If it's collecting, and is now 0, a scope is complete
                collected.push(current_collection.clone());
                current_collection = vec![];
                nesting_level = 0;
            }
        }

        // Already collecting? Push the current character
        if nesting_level > 0 {
            current_collection.push(element.clone());
        }
    }
    
    return collected;
}
