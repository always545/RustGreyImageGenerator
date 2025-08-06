use std::process::exit;

pub struct ParserChoices{
    //choose mode -f ----> file   -d ----> directory
    pub modeSelection:String,
    //8-bit? 16-bit?
    pub FormatSelection:String,
    //where is to-be translated ?
    pub target:String,
    //where to store
    pub destination:String,
}

impl ParserChoices{
    pub fn new(modeSelection:String,FormatSelection:String,target:String,destination:String)->ParserChoices{
        let Format = match FormatSelection.as_str(){
            "i8"=> FormatSelection,
            "i16"=> FormatSelection,
            _ => "i8".to_string()
        };
        ParserChoices{
            modeSelection:modeSelection,
            FormatSelection:Format,
            target:target,
            destination:destination
        }
    }
    
    fn check_mode(&self)->Result<String,String> {
        let mode = &self.modeSelection;
        match mode.as_str(){
            "-f" => Ok("-f".to_string()),
            "-d" => Ok("-d".to_string()),
            _    => Err(String::from("ModeSelection invalid!")),
        }
    }
}
pub fn collect_And_ParseArgs()->Result<ParserChoices,String>{
    let args:Vec<String> = std::env::args().collect();
    if args.len() != 5 && args[1] != "-help"{
        println!("Usage: RCIT <Mode> <Format> <target> <destination>");
        println!("use RCIT -help for help");
        exit(1)
    }
    if args[1] == "-help"{
        help();
        exit(1)
    }
    if args.len()!=5{
        println!("Usage: RCIT <Mode> <Format> <target> <destination>");
        exit(1)
    }
    let mode = &args[1];
    let fmt = &args[2];
    let target = &args[3];
    let destination = &args[4];
    let pc = ParserChoices::new(mode.clone(), fmt.clone(), target.clone(), destination.clone());
    match pc.check_mode(){
        Ok(mode) => Ok(pc),
        Err(msg) => Err(msg)
    }
}

fn help()->(){
    println!("Usage: RCIT <Mode> <Format> <target> <destination>");
    println!("Mode ----> -f for file -d for directory");
    println!("Format ----> i8 for 8bit luma, i16 for 16 bit luma");
    println!("Target --> path to img or directory");
    println!("Destination --> where you save");
}