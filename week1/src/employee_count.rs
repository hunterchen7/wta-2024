use std::collections::HashMap;

type Employee<'a> = &'a str;
type Manager<'a> = &'a str;

/// Given a dictionary that contains mapping of an employee and their manager, returns a dictionary
/// that contains the TOTAL number of employees working under each manager.
///
/// # Arguments
///
/// * `dict` - A HashMap<char, char> containing the mapping of an employee to their manager
///
/// # Returns
///
/// A HashMap<char, u32> containing the total number of employees working under each manager
///
/// # Example
///
/// ```
/// let dict = {
///     "A": "C",
///     "B": "C",
///     "C": "F",
///     "D": "E",
///     "E": "F",
///     "F": "F"
/// };
///
/// let result = employee_count(&dict);
///
/// assert_eq!(result, {
///     "A": 0,
///     "B": 0,
///     "C": 2,
///     "D": 0,
///     "E": 1,
///     "F": 5
/// });
/// ```
/// # Explanation
///
/// In this example C is manager of A, C is also manager of B, F is manager of C and so on.
/// A and B report to C, so C has 2 employees below them.
/// D reports to E, so E has 1 person below them.
/// C and E report to F, so F has the 2 employees directly reporting to them,
/// plus the employees reporting to C, plus the employees reporting to E.
/// F reports to themselves to indicate that they are the ceo.
///
/// # Approach
///
/// Create an adjacency list of the managers and their employees. If an employee doesn't have an
/// entry in the adjacency list, they have no employees reporting to them, so their final count is 0.
/// In other cases, perform DFS to count the number of employees reporting to each manager.
/// Memoize.
///
/// # Time and Space Complexity
///
/// The time complexity is O(n), where n is the number of employees - we iterate over each
/// employee once to create the adjacency list and DFS is also O(n) since each employee is
/// visited/computed once with memoization. Space complexity is O(n) as well since we store the
/// adjacency list and memoization results (which are also the final results).
pub fn employee_count<'a>(dict: &HashMap<Employee<'a>, Manager<'a>>) -> HashMap<Manager<'a>, u32> {
    // adjacency list
    let mut manager_employees: HashMap<Manager, Vec<Employee>> = HashMap::new();

    // fill adj list
    dict.iter().for_each(|(employee, manager)| {
        manager_employees
            .entry(*manager)
            .or_default()
            .push(*employee);
    });

    let mut result: HashMap<Manager, u32> = HashMap::new();

    // dfs to count employees
    fn dfs(
        manager: Manager,
        manager_employees: &HashMap<Manager, Vec<Employee>>,
        result: &HashMap<Manager, u32>,
    ) -> u32 {
        if !manager_employees.contains_key(&manager) {
            // base case: no employees
            return 0;
        }
        if result.contains_key(&manager) {
            // memoization
            return *result.get(&manager).unwrap();
        }
        let mut count = 0;
        for &employee in manager_employees.get(&manager).unwrap() {
            if employee == manager {
                // can't be your own employee
                continue;
            }
            count += 1 + dfs(employee, manager_employees, result);
        }
        count
    }

    dict.keys().for_each(|&manager| {
        result.insert(manager, dfs(manager, &manager_employees, &result));
    });

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_employee_count_1() {
        let mut dict = HashMap::new();
        dict.insert("A", "C");
        dict.insert("B", "C");
        dict.insert("C", "F");
        dict.insert("D", "E");
        dict.insert("E", "F");
        dict.insert("F", "F");

        let result = employee_count(&dict);
        assert_eq!(result.get(&"A"), Some(&0));
        assert_eq!(result.get(&"B"), Some(&0));
        assert_eq!(result.get(&"C"), Some(&2));
        assert_eq!(result.get(&"D"), Some(&0));
        assert_eq!(result.get(&"E"), Some(&1));
        assert_eq!(result.get(&"F"), Some(&5));
    }

    #[test]
    fn test_employee_count_branchless() {
        let mut dict = HashMap::new();
        dict.insert("A", "B");
        dict.insert("B", "C");
        dict.insert("C", "D");
        dict.insert("D", "E");
        dict.insert("E", "F");
        dict.insert("F", "F");

        let result = employee_count(&dict);
        assert_eq!(result.get(&"A"), Some(&0));
        assert_eq!(result.get(&"B"), Some(&1));
        assert_eq!(result.get(&"C"), Some(&2));
        assert_eq!(result.get(&"D"), Some(&3));
        assert_eq!(result.get(&"E"), Some(&4));
        assert_eq!(result.get(&"F"), Some(&5));
    }

    #[test]
    fn test_employee_count_empty() {
        let dict = HashMap::new();
        let result = employee_count(&dict);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_employee_count_3() {
        let mut dict = HashMap::new();

        dict.insert("A", "A");
        dict.insert("B", "A");
        dict.insert("C", "A");
        dict.insert("D", "A");
        dict.insert("E", "A");
        dict.insert("F", "A");
        dict.insert("B1", "B");
        dict.insert("B2", "B");
        dict.insert("B3", "B");
        dict.insert("B4", "B");
        dict.insert("B5", "B");
        dict.insert("C1", "C");
        dict.insert("C1A", "C1");
        dict.insert("C1B", "C1");
        dict.insert("C1C", "C1");

        let result = employee_count(&dict);
        assert_eq!(result.get(&"A"), Some(&14));
        assert_eq!(result.get(&"B"), Some(&5));
        assert_eq!(result.get(&"C"), Some(&4));
    }
}
