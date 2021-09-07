fn main() {
    // Creating adhoc csv data
    let penguin_data = "\
        common name,length(cm)
        Little Penguin,33
        Yellow-eyed penguin,65
        Fiordland penguin,60
        invalid,data
    ";

    // use the .lines() method of the str type to retrieve an iterator of the strings lines. i'm guessing. 
    let records = penguin_data.lines();

    // super similar  to pythons for k, v in enumerate(stuff)
    for(i, record) in records.enumerate() {
        // skip the first line (the column info line) and blank lines
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // if its not the first line or a blank line,
        // create the fields variable which is a vector (maybe thats like a dict?)
        // and make it equal to some stuff done on record that i'm not quite sure about
        let fields: Vec<_> = record
            // this ones easy enough, split the two columns of the csv
            .split(',')
            // no idea, why is field in there twice and what are the | doing
            .map(|field|field.trim())
            // again not sure, maybe it turns the previous results into a dictionary or vector or whatever
            .collect();

        // so this is conditional compiling, i've read that already, but idk what debug_assertion is or what cfg does yet
        if cfg!(debug_assertions) {
            // question marks around what :? means.. 
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        // ok finally I know whats happening, take the first index of fields which should be first column. i hope
        let name = fields[0];

        // so grab the second column (the length) and try to parse it as a 32bit float(i think) then if there is no error (the ok bit) do the if statement
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }

    }

}