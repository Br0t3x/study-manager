pub mod utils {
    use std::fs;
    use std::io::{stdin, Write};
    use regex::Regex;
    use self::structs::*;

    pub fn range_pattern() -> Regex {
        Regex::new("^(\\d+)(\\.\\.(\\d+))?$").unwrap()
    }
    pub fn list_pattern() -> Regex {
        Regex::new("^\\[\\d+((, | )\\d+)*]$").unwrap()
    }
    pub mod structs {
        use serde::{Deserialize, Serialize};
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Chapter {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub label: Option<String>,
            pub exercises: Vec<Exercise>,
            #[serde(default)]
            pub sum_exercises: bool
        }
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Exercise {
            #[serde(default)]
            pub completion_date: Option<String>,
            #[serde(default)]
            pub label: Option<String>,
        }
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Subject {
            pub name: String,
            pub chapters: Vec<Chapter>,
        }
        #[derive(Serialize, Deserialize)]
        pub struct Config {
            #[serde(default)]
            pub selected_subject: usize,
            #[serde(default)]
            pub selected_chapter: usize,
        }
    }
    pub fn merge_multiple_word_strings(args: Vec<&str>) -> Vec<String> {
        let mut new_args: Vec<String> = vec![];
        let mut it_is_a_long_string = false;

        for index in 0..args.len() {
            let arg = args[index];

            if it_is_a_long_string {
                let length = new_args.len();
                new_args[length - 1].push_str(" ");
                new_args[length - 1].push_str(arg.replace("\"", "").as_str());

                if arg.ends_with("\"") || arg.ends_with("]") {
                    it_is_a_long_string = false;
                    continue;
                }
            } else {
                // "... ou [...
                if (arg.starts_with("\"") && !arg.ends_with("\"")) || (arg.starts_with("[") && !arg.ends_with("]")) {
                    it_is_a_long_string = true;
                    new_args.push(arg.replace("\"", ""));
                }
                // "..." ou ..." ou [...]
                else if ((arg.starts_with("\"") && arg.ends_with("\"")) || arg.ends_with("\"")) || (arg.starts_with("[") && arg.ends_with("]")) {
                    new_args.push(arg.replace("\"", ""));
                }
                // ...
                else {
                    new_args.push(arg.to_string());
                }
            }
        }

        new_args
    }
    pub fn read_data() -> Vec<Subject> {
        serde_json::from_str::<Vec<Subject>>(fs::read_to_string("./data/data.json").unwrap().as_str()).expect("Error reading data file")
    }
    pub fn write_data(data: Vec<Subject>) {
        let content = serde_json::to_string(&data).expect("Error when serializing data file");
        fs::write("./data/data.json", &content).expect("Error when writing data file");
    }
    pub fn read_config() -> Config {
        serde_json::from_str(fs::read_to_string("./data/config.json").unwrap().as_str()).expect("Error reading config file")
    }
    pub fn write_config(config: Config) {
        let content = serde_json::to_string(&config).expect("Error when serializing config file");
        fs::write("./data/config.json", &content).expect("Error when writing config file");
    }
    pub fn parseable(v: &String) -> bool {
        v.parse::<usize>().is_ok()
    }
    pub fn not_parseable(v: &String) -> bool {
        !parseable(v)
    }
    pub fn rename_subject(subject_id: usize, new_name: String) {
        let mut data = read_data();
        if data.get(subject_id).is_none() {
            println!("Entered subject id doesn't exist");
            return;
        }
        let subject = &mut data[subject_id];

        println!("Subject renamed from {} to {}", subject.name, new_name);
        subject.name = new_name;
        write_data(data);
    }
    pub fn rename_chapter(subject_id: usize, chapter_id: usize, name: String) {
        let mut data = read_data();
        if data.get(subject_id).is_none() {
            println!("Entered subject id doesn't exist");
            return;
        }

        let subject = &mut data[subject_id];
        if subject.chapters.get(chapter_id).is_none() {
            println!("Entered chapter id doesn't exist");
            return;
        }

        let chapter = &mut subject.chapters[chapter_id];
        println!("{}'s chapter renamed from {} to {}", subject.name, chapter.name, name);
        chapter.name = name;
        write_data(data);
    }
    pub fn relabel(subject_id: usize, chapter_id: usize, label: Option<String>) {
        let mut data = read_data();
        if data.get(subject_id).is_none() {
            println!("Entered subject id doesn't exist");
            return;
        }

        let subject = &mut data[subject_id];
        if subject.chapters.get(chapter_id).is_none() {
            println!("Entered chapter id doesn't exist");
            return;
        }

        let chapter = &mut subject.chapters[chapter_id];

        match &label {
            Some(new_label) => {
                match &chapter.label {
                    Some(old_label) => {
                        println!("{}'s label renamed from {} to {}", chapter.name, old_label, new_label);
                    }
                    None => {
                        println!("{}'s label set to {}", chapter.name, new_label);
                    }
                }
            }
            None => {
                println!("Cleared {}'s label", chapter.name);
            }
        }
        chapter.label = label;
        write_data(data);
    }
    pub fn relabel_exercise(subject_id: usize, chapter_id: usize, exercises: Vec<usize>, label: Option<String>) {
        let mut data = read_data();
        if data.get(subject_id).is_none() {
            println!("Entered subject id doesn't exist");
            return;
        }

        let subject = &mut data[subject_id];
        if subject.chapters.get(chapter_id).is_none() {
            println!("Entered chapter id doesn't exist");
            return;
        }

        let chapter = &mut subject.chapters[chapter_id];
        let mut counting = 0;

        for exercise in &exercises {
            if chapter.exercises.get(*exercise).is_none() {
                println!("Exercise of id {} of {}'s chapter {} doesn't exist", exercise, subject.name, chapter.name);
                continue;
            }
            chapter.exercises[*exercise].label = label.clone();
            counting += 1;
            if label.is_none() {
                println!("Exercise of id {} of {}'s chapter {} have been unlabeled", exercise, subject.name, chapter.name);
            } else {
                println!("Exercise of id {} of {}'s chapter {} have been labeled to [{}]", exercise, subject.name, chapter.name, label.clone().unwrap());
            }
        }
        if label.is_none() {
            println!("\n{} exercises have been unlabeled", counting);
        } else {
            println!("\n{} exercises have been labeled to [{}]", counting, label.clone().unwrap());
        }
        write_data(data);
    }
    pub fn update_solve_date(subject_id: usize, chapter_id: usize, exercises: Vec<usize>, date: Option<String>) {
        let mut data = read_data();
        if data.get(subject_id).is_none() {
            println!("Entered subject id doesn't exist");
            return;
        }

        let subject = &mut data[subject_id];
        if subject.chapters.get(chapter_id).is_none() {
            println!("Entered chapter id doesn't exist");
            return;
        }

        let chapter = &mut subject.chapters[chapter_id];
        let mut counting = 0;

        for exercise in &exercises {
            if chapter.exercises.get(*exercise).is_none() {
                println!("Exercise of id {} of {}'s chapter {} doesn't exist", exercise, subject.name, chapter.name);
                continue;
            }
            chapter.exercises[*exercise].completion_date = date.clone();
            counting += 1;

            if date.is_none() {
                println!("Exercise of id {} of {}'s chapter {} its not solved anymore", exercise, subject.name, chapter.name);
            } else {
                println!("Exercise of id {} of {}'s chapter {} have its solve date set to {}", exercise, subject.name, chapter.name, date.clone().unwrap());
            }
        }
        if date.is_none() {
            println!("\n{} exercises are not solved anymore", counting);
        } else {
            println!("\n{} exercises have its solve date set to {}", counting, date.clone().unwrap());
        }
        write_data(data);
    }
    pub fn ask_removing_subject_permission() -> String {
        let reader = stdin();
        let mut input = String::new();
        reader
            .read_line(&mut input)
            .expect("Error when reading input");
        input.trim().to_lowercase()
    }
    pub fn ask_removing_chapter_permission(chapter_name: &String, subject_name: &String) -> String {
        print!("Do you want to remove chapter {chapter_name} of subject {subject_name} (S/n)? ");
        std::io::stdout().flush().unwrap();

        let reader = stdin();
        let mut input = String::new();
        reader
            .read_line(&mut input)
            .expect("Error when reading input");
        input.trim().to_lowercase()
    }
    pub fn ask_about_removing_subject(mut subject_id: usize) {
        let mut data = read_data();

        print!(
            "Do you want to remove subject {} (S/n)? ",
            data[subject_id].name
        );
        std::io::stdout().flush().unwrap();

        let mut config = read_config();
        let input = ask_removing_subject_permission();

        if input == String::from("s") {
            println!("Subject {} has been removed", data[subject_id].name);
            data.remove(subject_id);

            if data.get(subject_id).is_none() {
                if data.get(subject_id - 1).is_some() {
                    subject_id -= 1;
                }
            }
            config.selected_subject = subject_id;
            config.selected_chapter = 0;
            write_config(config);
            write_data(data);
        } else {
            println!("Aborting removing operation");
        }
    }
    pub fn ask_about_removing_chapter(subject_id: usize, mut chapter_id: usize) {
        let mut data = read_data();
        let mut config = read_config();
        let input = ask_removing_chapter_permission(
            &data[subject_id].chapters[chapter_id].name,
            &data[subject_id].name,
        );

        if input == String::from("s") {
            println!(
                "Chapter {} was removed from subject {}",
                data[subject_id].chapters[chapter_id].name, data[subject_id].name
            );
            data[subject_id].chapters.remove(chapter_id);
            let chapters = &data[subject_id].chapters;

            if chapters.get(chapter_id).is_none() {
                if chapter_id as i32 - 1 >= 0 {
                    if chapters.get(chapter_id - 1).is_some() {
                        chapter_id -= 1;
                    }
                }
            }
            config.selected_chapter = chapter_id;
            write_config(config);
            write_data(data);
        } else {
            println!("Aborting removing operation");
        }
    }
    pub fn get_exercises(id: String) -> Vec<usize> {
        let mut exercises: Vec<usize> = Vec::new();
        // x..y | x
        if range_pattern().is_match(&id) {
            let captures = range_pattern().captures(&id).unwrap();
            let start_point = captures.get(1);
            let end_point = captures.get(3);

            // x..y
            if start_point.is_some() && end_point.is_some() {
                let start_point = start_point.unwrap().as_str().parse::<usize>().unwrap();
                let end_point = end_point.unwrap().as_str().parse::<usize>().unwrap();

                if start_point < end_point || start_point == end_point {
                    exercises = (start_point..=end_point).collect::<Vec<usize>>();
                } else {
                    exercises = (end_point..=start_point).collect::<Vec<usize>>();
                }
            }
            // x
            else if start_point.is_some() && end_point.is_none() {
                exercises = vec![id.parse::<usize>().unwrap()];
            }
        }
        // [x, y, ..., z] | [x y ... z]
        else if list_pattern().is_match(&id) {
            exercises = Regex::new("\\d+").unwrap()
                .find_iter(&id)
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
        }

        exercises
    }
}
