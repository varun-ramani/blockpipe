use std::collections::{HashMap, HashSet};
use super::Value;

/// the runtime stack
#[derive(Debug)]
pub struct Environment {
    pub stack_frames: Vec<HashSet<String>>,
    pub keys: HashMap<String, Vec<Value>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            stack_frames: Vec::new(),
            keys: HashMap::new(),
        }
    }

    /// invoked at the start of a block
    pub fn push_stack_frame(&mut self) {
        self.stack_frames.push(HashSet::new());
    }

    pub fn pop_stack_frame(&mut self) -> Result<(), ()> {
        let last_frame = self.stack_frames.pop().ok_or(())?;

        for key in last_frame {
            self.keys
                .get_mut(&key)
                .expect("stack corruption: pop stack frame")
                .pop();

            if self
                .keys
                .get_mut(&key)
                .expect("stack corruption: pop stack frame")
                .is_empty()
            {
                self.keys.remove(&key);
            }
        }

        Ok(())
    }

    pub fn bind(&mut self, key: String, value: Value) {
        // if the current key is already in the stack frame, then we'll remove it
        if self
            .stack_frames
            .last()
            .expect("stack corruption: bind")
            .contains(&key)
        {
            self.keys
                .get_mut(&key)
                .expect("stack corruption: bind")
                .pop();
        }
        // otherwise, we'll just go ahead and insert the current key to the stack frame
        else {
            self.stack_frames
                .last_mut()
                .expect("stack corruption: bind")
                .insert(key.clone());
        }

        // and then we'll unconditionally insert the key
        self.keys.entry(key).or_insert(Vec::new()).push(value);
    }

    pub fn lookup(&self, key: &str) -> Option<Value> {
        Some(self.keys.get(key)?.last()?.clone())
    }

    /// grab the most recent set of bindings in the environment
    pub fn image(&self) -> HashMap<String, Value> {
        self.keys
            .iter()
            .map(|(key, value)| {
                let cloned_key = key.clone();
                let cloned_value = value
                    .last()
                    .expect("binding without associated values")
                    .clone();

                (cloned_key, cloned_value)
            })
            .collect()
    }
}