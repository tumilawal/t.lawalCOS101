use std::io::Write;

fn main() { 
    let mut file = std::fs::File::create("EFCC-Merge.csv").unwrap();
     // as a vector
    let mut commissioner: Vec<String> = Vec::new();
    commissioner.push(String::from("Aigbogun Alamba Daudu"));
    commissioner.push(String::from("Murtala Afeez Bendu"));
    commissioner.push(String::from("Okorocha Calistus Ogbona"));
    commissioner.push(String::from("Adewale Jimoh Akanbi"));
    commissioner.push(String::from("Osazuwa Faith Etieye"));

    let ministry = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];
    // as an array
    let zone = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

  //write in to the file
    writeln!(file, "Commissioner,Ministry,Zone").unwrap();
    writeln!(file, "{},{},{}", commissioner[0], ministry[0], zone[0]).unwrap();
    writeln!(file, "{},{},{}", commissioner[1], ministry[1], zone[1]).unwrap();
    writeln!(file, "{},{},{}", commissioner[2], ministry[2], zone[2]).unwrap();
    writeln!(file, "{},{},{}", commissioner[3], ministry[3], zone[3]).unwrap();
    writeln!(file, "{},{},{}", commissioner[4], ministry[4], zone[4]).unwrap();

    println!("The file has been succesfully created!");
}