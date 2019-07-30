//Uses eliza-rs for basic nlp
//cosmic_brain.json - the script that defines basic patterns should be present
use eliza::Eliza;

//Just a wrapper around eliza crate to fit in the cosmic' code
//defines basic methods to understand and generating responses
pub struct AI {
    instance: Eliza,
}

//Due to conflict in mutability in respond method in eliza and message trait in serenity
//declare BRAIN in global state
//TODO: a method to achieve same without unsafe code
pub static mut BRAIN: Option<AI> = None;

impl AI {
    pub fn new(location: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let eliza = Eliza::new(location)?;

        Ok(AI { instance: eliza })
    }

    //Generates responses for the input according to script defined
    pub fn genrate_response(&mut self, input: &str) -> String {
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

//returns response from static reference
pub fn respond(input: &str) -> String {
    unsafe { BRAIN.as_mut().unwrap().genrate_response(input) }
}

//Initialise brain in the static reference
pub fn brain_init(location: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let brain = AI::new(location)?;
        BRAIN = Some(brain);
    }

    Ok(())
}
