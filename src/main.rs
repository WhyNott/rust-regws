#[derive(PartialOrd,Copy, Clone, Ord, Eq, PartialEq, Debug)]
enum Object {
    A,
    B,
    C,
    Table
}
#[derive(PartialOrd, Copy, Clone, Ord, Eq, PartialEq, Debug)]
enum GroundLiteral {
    On(Object, Object),
    Clear(Object),
    End //not super elegant, but w/e
}


#[derive(PartialOrd, Clone, Ord, Eq, PartialEq, Debug)]
struct Action {
    name: String,
    preconditions: [GroundLiteral; 4],
    add: [GroundLiteral; 4],
    remove: [GroundLiteral; 4],
}

use crate::Object::*;
use crate::GroundLiteral::*;



type State = Vec<GroundLiteral>;

//methods I need:
//isSubset
//remove

//I reckon, the size of cur_goals and the add/remove fields will always be smaller then the amount of branching that will need to be done. Hence, it doesn't really make sense to use hashsets/BTreesets, even if it will make big Oh better. 

fn is_subset<T>(a: &[T], b: &[T]) -> bool where T: PartialEq {
    for item1 in a {
	let mut present = false;
	for item2 in b {
	    if item1 == item2 {
		present = true;
	    }
	}
	if !present {
	    return false
	}
    }   
    true
}

fn remove<T>(item: T, vector: &mut Vec<T>) -> bool where T: PartialEq {
    for (i, element) in vector.iter().enumerate() {
	if *element == item {
	    vector.swap_remove(i);
	    return true  
	}
    }
    false
}


fn regws_wrap<'a, F>(init_state: &State, cur_goals: &State, past_goals: &mut Vec<State>, actions: &'a [Action; 18], path: &mut Vec<&'a Action>, continuation : &F) where F: Fn(&Vec<&Action>) + Clone {
    for depth in 1..8 {
	regws(depth, init_state, cur_goals, past_goals, actions, path, continuation);
    }
    
}



fn regws<'a, F>(depth: u8, init_state: &State, mut cur_goals: &State,  past_goals: &mut Vec<State>, actions:  &'a [Action; 18],  path: &mut Vec<&'a Action>, continuation : F) where F: Fn(&Vec<&Action>) + Clone {
    if depth > 0  {
	if is_subset(cur_goals, init_state){
	    continuation(path);
	} else {
	    
	    let mut success = false;
	    for act in actions {
		
	
		    for x in act.add.iter() {
			if cur_goals.contains(x){
			    success = true;
			    break;
			}
		    }
		
		    if success {
			for y in act.remove.iter() {
			    if cur_goals.contains(y){
				success = false;
				break;
			    }
			}
		    }
		

		let mut next_goals = cur_goals.clone();
		    if success  {
			for element in act.add.iter() {
			    if *element == End {
				break;
			    }
			    remove(*element, &mut next_goals);
			}
			for element in act.preconditions.iter() {
			    if *element == End {
				break;
			    }
			    next_goals.push(*element);
			}
			
		    }

		if !past_goals.contains(cur_goals){
		    past_goals.push(cur_goals.clone());

		   
		    path.push(act);



		    regws(depth-1, init_state, &next_goals, past_goals, actions, path, continuation.clone());

		    past_goals.pop();
		    path.pop();
		    }
		
	    }
	}
    }
}



fn main() {

    let mut path = Vec::new();
    let actions: [Action; 18] = [
	Action {
    	    name: String::from("move-A-from-B-to-C"),
    	    preconditions: [On(A, B), Clear(A), Clear(C), End],
    	    add:             [On(A, C), Clear(B), End, End],
    	    remove:         [On(A, B), Clear(C), End, End]
	},
	Action {
    	    name: String::from("move-A-from-C-to-B"),
    	    preconditions: [On(A, C), Clear(A), Clear(B), End],
    	    add:           [On(A, B), Clear(C), End, End],
    	    remove:        [On(A, C), Clear(B), End, End]
	},
	Action {
    	    name: String::from("move-A-from-C-to-T"),
    	    preconditions: [On(A, C), Clear(A), End, End],
    	    add:           [On(A, Table), Clear(C), End, End],
    	    remove:        [On(A, C), End, End, End]
	},
	Action {
    	    name: String::from("move-A-from-B-to-T"),
    	    preconditions: [On(A, B), Clear(A), End, End],
    	    add:           [On(A, Table), Clear(B), End, End],
    	    remove:        [On(A, B), End, End, End]
	},
	Action {
    	    name: String::from("move-B-from-A-to-C"),
    	    preconditions: [On(B, A), Clear(B), Clear(C), End],
    	    add:           [On(B, C), Clear(A), End, End],
    	    remove:        [On(B, A), Clear(C), End, End]
	},
	Action {
    	    name: String::from("move-B-from-C-to-A"),
    	    preconditions: [On(B, C), Clear(B), Clear(A), End],
    	    add:           [On(B, A), Clear(C), End, End],
    	    remove:        [On(B, C), Clear(A), End, End]
	},
	Action {
    	    name: String::from("move-B-from-A-to-T"),
    	    preconditions: [On(B, A), Clear(B), End, End],
    	    add:           [On(B, Table), Clear(A), End, End],
    	    remove:        [On(B, A), End, End, End]
	}, 
	Action {
    	    name: String::from("move-B-from-C-to-T"),
    	    preconditions: [On(B, C), Clear(B), End, End],
    	    add:           [On(B, Table), Clear(C), End, End],
    	    remove:        [On(B, C), End, End, End]
	},
	Action {
    	    name: String::from("move-C-from-A-to-B"),
    	    preconditions: [On(C, A), Clear(C), Clear(B), End],
    	    add:           [On(C, B), Clear(A), End, End],
    	    remove:        [On(C, A), Clear(B), End, End]
	},
	Action {
    	    name: String::from("move-C-from-B-to-A"),
    	    preconditions: [On(C, B), Clear(C), Clear(A), End],
    	    add:           [On(C, A), Clear(B), End, End],
    	    remove:        [On(C, B), Clear(A), End, End]
	},
	Action {
    	    name: String::from("move-C-from-B-to-T"),
    	    preconditions: [On(C, B), Clear(C), End, End],
    	    add:           [On(C, Table), Clear(B), End, End],
    	    remove:        [On(C, B), End, End, End]
	},

	Action {
	    name: String::from("move-C-from-A-to-T"),
	    preconditions: [On(C, A), Clear(C), End, End],
	    add:           [On(C, Table), Clear(A), End, End],
	    remove:        [On(C, A), End, End, End]
	},
	
	Action {
	    name: String::from("move-A-from-T-to-B"),
	    preconditions: [On(A, Table), Clear(A), Clear(B), End],
	    add:           [On(A, B), End, End, End],
	    remove:        [On(A, Table), Clear(B), End, End]
	},
	Action {
    	    name: String::from("move-A-from-T-to-C"),
    	    preconditions: [On(A, Table), Clear(A), Clear(C), End],
    	    add:           [On(A, C), End, End, End],
    	    remove:        [On(A, Table), Clear(C), End, End]
	},
	Action {
    	    name: String::from("move-B-from-T-to-A"),
    	    preconditions: [On(B, Table), Clear(B), Clear(A), End],
    	    add:           [On(B, A), End, End, End],
    	    remove:        [On(B, Table), Clear(A), End, End]
	},
	Action {
	    name: String::from("move-B-from-T-to-C"),
	    preconditions: [On(B, Table), Clear(B), Clear(C), End],
	    add:           [On(B, C), End, End, End],
	    remove:        [On(B, Table), Clear(C), End, End]
	},
	Action {
    	    name: String::from("move-C-from-T-to-A"),
    	    preconditions: [On(C, Table), Clear(C), Clear(A), End],
    	    add:           [On(C, A), End, End, End],
    	    remove:        [On(C, Table), Clear(A), End, End]
	},
	Action {
    	    name: String::from("move-C-from-T-to-B"),
    	    preconditions: [On(C, Table), Clear(C), Clear(B), End],
    	    add:           [On(C, B), End, End, End],
    	    remove:        [On(C, Table), Clear(B), End, End]
	}
    ];


    
    let hello =  |result: &Vec<&Action>| {
	for action in (result.iter()).rev() {
	    println!("{}", action.name);
	}
	 std::process::exit(0); // so that it only gives 1 solution
    };
    let init : Vec<_> = vec![On(A, Table), On(B, Table), On(C, A),
				  Clear(B), Clear(C)];
    let mut goal : Vec<_> = vec![On(A, B), On(B, C)];

    let mut past_goals = Vec::new();
    
  
    regws_wrap(&init, &mut goal, &mut past_goals, &actions, &mut path, &hello);

    println!("No solutions found!");
}
