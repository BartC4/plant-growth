#[derive(Default)]

pub(crate) struct LindenSystem {
    sequence: String,
    rule_set: Vec<(char, String)>,
    angle: f32,
    length_factor: f32,
    current_iteration: u8,
}


#[allow(dead_code)]
impl LindenSystem {
    pub(crate) fn new() -> LindenSystem {
        let sequence = String::from("x");
        let rule_set = Vec::with_capacity(10);
        let angle = 45.0;
        let length_factor = 1.2;
        let current_iteration = 0;

        LindenSystem {
            sequence,
            rule_set,
            angle,
            length_factor,
            current_iteration,
        }
    }

    pub(crate) fn iterate(&mut self) {
        let mut next = String::new();
        for c in self.sequence.chars() {
            match self.get_conversion(c) {
                Some(p) => {
                    next.push_str(p);
                }
                None => {
                    next.push(c);
                }
            }
        }
        self.sequence = next;
        self.current_iteration += 1;
    }
    
    fn add_rule(&mut self, variable: &char, conversion: &str) {
        // Check if the variable already has a rule.
        for rule in self.rule_set.iter() {
            let (other_variable, _) = rule;
            if variable == other_variable {
                // Don't add the new rule to the rule set.
                return;
            }
        }
        // Add the new rule to the rule set.
        let new_rule = (variable.clone(), conversion.to_string());
        self.rule_set.push(new_rule);
    }

    fn get_conversion(&self, c: char) -> Option<&str> {
        // Loop through the rule set and return the conversion.
        // This may be hash-able?;

        for rule in self.rule_set.iter() {
            let (var, conv) = rule;
            if *var == c {
                return Some(conv.trim());
            }
        }
        None
    }
    fn get_sequence(&self) -> &String {
        &self.sequence
    }
    pub(crate) fn get_rule_set(&self) -> &Vec<(char, String)> {
        &self.rule_set
    }
    fn set_angle(&mut self, new_angle: f32) {
        self.angle = new_angle;
    }
    fn get_angle(&self) -> f32 {
        self.angle
    }
    fn set_length_factor(&mut self, new_length_factor: f32) {
        self.length_factor = new_length_factor;
    }
    fn get_length_factor(&self) -> f32 {
        self.length_factor
    }
}