//Uses eliza-rs for basic nlp
//cosmic.json - the script that defines basic patterns should be present

use eliza::Eliza;

//Just a wrapper around eliza crate to fit in the cosmic' code
//defines basic methods to understand and generating responses
pub struct AI {
    instance: Eliza,
}

impl AI {
    pub fn new(location: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let eliza = Eliza::new(location)?;

        Ok(AI { instance: eliza })
    }

    //Generates responses for the input according to script defined
    pub fn respond(&mut self, input: &str) -> String {
        self.instance.respond(input)
    }

    //Generates random farewell message from script
    //pub fn farewell(&self) -> String {
    //    self.instance.farewell()
    //}

    //Generates random greetings from script
    //pub fn greet(&self) -> String {
    //    self.instance.greet()
    //}
}
