mod utils;

use crate::structs::*;
use crate::utils::utils::*;
use std::collections::HashMap;
use std::fs::{File, create_dir, exists};
use std::io::{Write, stdin};

fn main() {
    let data_folder_exists = exists("./data").unwrap();
    if !data_folder_exists {
        create_dir("./data").unwrap();
    }

    let data_file_exists = exists("./data/data.json").unwrap();
    if !data_file_exists {
        File::create("./data/data.json")
            .unwrap()
            .write(b"[]\n")
            .unwrap();
    }

    let config_file_exists = exists("./data/config.json").unwrap();
    if !config_file_exists {
        File::create("./data/config.json")
            .unwrap()
            .write(b"{}\n")
            .unwrap();
    }

    let reader = stdin();
    let mut stdout = std::io::stdout();
    let mut input = String::new();

    print!("Enter some command (-h to list them and --help <command> to explain command): ");
    stdout.flush().unwrap();

    reader.read_line(&mut input).expect("Failed to read line");

    let mut args = merge_multiple_word_strings(input.trim().split(" ").collect::<Vec<&str>>());

    while args.len() > 0 {
        match args[0].as_str() {
            "list" => list(args[1..].to_vec()),
            "remove" => remove(args[1..].to_vec()),
            "report" => report(args[1..].to_vec()),
            "-h" => {
                println!("Available commands are: init, new, list, select, edit, remove, report")
            }
            "--help" => {
                if args.get(1).is_some_and(|v| match v.as_str() {
                    "new" | "select" | "list" | "edit" | "remove" | "report" => true,
                    _ => false,
                }) {}
            }
            value @ ("new" | "select" | "edit") => {
                if args.get(1).is_none() {
                    println!("{} command takes at least one argument", value);
                } else {
                    match value {
                        "new" => new(args[1..].to_vec()),
                        "select" => select(args[1..].to_vec()),
                        "edit" => edit(args[1..].to_vec()),
                        _ => return,
                    }
                }
            }
            _ => continue,
        }

        print!("Enter some command (-h to list them and --help <command> to explain command): ");
        stdout.flush().unwrap();
        input.clear();

        reader.read_line(&mut input).expect("Failed to read line");
        args = merge_multiple_word_strings(input.trim().split(" ").collect::<Vec<&str>>());
    }
}
fn list(args: Vec<String>) {
    let data = read_data();
    let config = read_config();

    if args.get(0).is_none() {
        if data.len() == 0 {
            println!("There are no subjects yet");
            return;
        }

        println!("Subjects:");

        for (i, subject) in data.iter().enumerate() {
            if i == config.selected_subject {
                println!("-> {} - {}", i, subject.name);
            } else {
                println!("   {} - {}", i, subject.name);
            }
        }
    } else if args.get(0).is_some_and(|v| *v == "all") {
        if data.get(config.selected_subject).is_some() {
            for (i, subject) in data.iter().enumerate() {
                if i == config.selected_subject {
                    println!("-> {} - {}", i + 1, subject.name);
                } else {
                    println!("   {} - {}", i + 1, subject.name);
                }
                for (j, chapter) in subject.chapters.iter().enumerate() {
                    if chapter.label.is_none() {
                        if i == config.selected_subject && j == config.selected_chapter {
                            println!("  -> {} - {}", j + 1, chapter.name);
                        } else {
                            println!("     {} - {}", j + 1, chapter.name);
                        }
                    } else {
                        if i == config.selected_subject && j == config.selected_chapter {
                            println!("  -> {} - [{}] {}", j + 1, chapter.label.clone().unwrap(), chapter.name);
                        } else {
                            println!("     {} - [{}] {}", j + 1, chapter.label.clone().unwrap(), chapter.name);
                        }
                    }
                }
            }
        }
    }
}
fn select(args: Vec<String>) {
    let data = read_data();
    let mut config = read_config();
    let mut selected_subject = config.selected_subject;
    let mut selected_chapter = config.selected_chapter;

    // select <subject_id>
    if parseable(&args[0]) && args.get(1).is_none() {
        let subject_id = args[0].parse::<usize>().unwrap() - 1;
        if data.get(subject_id).is_none() {
            println!("The entered subject id doesn't exist");
            return;
        }

        selected_subject = subject_id;
        if data[subject_id].chapters.len() == 0 {
            println!("Selected subject {}", data[subject_id].name);
        } else {
            selected_chapter = 0;
            println!(
                "Selected subject {} and its first chapter",
                data[subject_id].name
            );
        }
    }
    // select chapter <subject_id>
    else if args[0] == "chapter" && args.get(1).is_some_and(parseable) {
        if data.get(selected_subject).is_none() {
            println!("There is no selected subject");
            return;
        }
        let chapter_id = args[1].parse::<usize>().unwrap() - 1;

        if data[selected_subject].chapters.get(chapter_id).is_some() {
            selected_chapter = chapter_id;
            println!(
                "Selected chapter {} of subject {}",
                data[selected_subject].chapters[selected_chapter].name, data[selected_subject].name
            );
        } else {
            println!(
                "The entered chapter id of subject {} doesn't exist",
                data[selected_subject].name
            );
            return;
        }
    }
    // select <subject_id> <chapter_id>
    else if parseable(&args[0]) && args.get(1).is_some_and(parseable) {
        let subject_id = args[0].parse::<usize>().unwrap() - 1;
        if data.get(subject_id).is_none() {
            println!("The entered subject id doesn't exist");
            return;
        }

        let subject = &data[subject_id];
        let chapter_id = args[1].parse::<usize>().unwrap() - 1;
        if subject.chapters.get(chapter_id).is_none() {
            println!("The entered chapter id of subject doesn't exist");
            return;
        }

        selected_subject = subject_id;
        selected_chapter = chapter_id;

        println!(
            "Selected chapter {} of subject {}",
            subject.chapters[selected_chapter].name, data[selected_subject].name
        );
    }
    config.selected_subject = selected_subject;
    config.selected_chapter = selected_chapter;

    write_config(config);
}
fn new(args: Vec<String>) {
    let mut config = read_config();
    let mut data = read_data();
    if args[0] == "chapter" {
        if args.get(1).is_none() {
            println!("No chapter name was entered");
            return;
        }
        let subject = &mut data[config.selected_subject];
        let mut label: Option<String> = None;
        let mut sum_exercises = false;
        // new chapter <name> --label <label>
        if args.get(2).is_some_and(|v| v == "--label") && args.get(3).is_some() {
            label = Some(args[3].clone());
        }
        // new chapter <name> --label <label> -sum_exercises
        if args.get(4).is_some_and(|v| v == "-sum_exercises") {
            sum_exercises = true;
        }

        subject.chapters.push(Chapter {
            name: args[1].to_string(),
            label: label.clone(),
            exercises: vec![],
            sum_exercises,
        });

        config.selected_chapter = subject.chapters.len() - 1;

        if label.is_none() {
            println!("Created and selected new chapter {}", args[1]);
        } else {
            println!("Created and selected new chapter {} [{}]", args[1], args[3]);
        }

        write_config(config);
        write_data(data);
    }
    // new exercise <range>
    else if args[0] == "exercise" {
        if data.get(config.selected_subject).is_none() {
            println!("There is no selected subject");
            return;
        }

        let subject = &mut data[config.selected_subject];
        if subject.chapters.get(config.selected_chapter).is_none() {
            println!("There is no selected chapter");
            return;
        }

        if args.get(1).is_none() {
            println!("You should enter a range of exercises to be created");
            return;
        }

        let chapter = &mut subject.chapters[config.selected_chapter];
        let range = &args[1];
        let mut label: Option<String> = None;
        let old_size = chapter.exercises.len();

        if args.get(2).is_some_and(|v| v == "--label") && args.get(3).is_some() {
            label = Some(args[3].clone());
        }

        if not_parseable(range) {
            println!("The range is not a valid integer");
            return;
        }

        let exercise_number = range.parse::<i32>().unwrap();
        let mut counting = 0;

        if chapter.exercises.get(exercise_number as usize).is_none() {
            for _ in chapter.exercises.len() as i32 - 1..exercise_number - 1 {
                chapter.exercises.push(Exercise {
                    completion_date: None,
                    label: label.clone(),
                });
                counting += 1;
            }
        }
        if counting == 0 {
            println!("No exercise was created");
        } else if counting == 1 {
            println!("One exercise was created");
        } else {
            println!("Exercises in the range {old_size}..{exercise_number} were created");
        }

        write_data(data);
    } else {
        let subject = Subject {
            name: args[0].to_string(),
            chapters: vec![],
        };
        println!("Created and selected new subject {}", subject.name);
        data.push(subject);
        config.selected_subject = data.len() - 1;

        write_config(config);
        write_data(data);
    }
}
fn edit(args: Vec<String>) {
    if args.get(0).is_none() {
        return;
    }
    let config = read_config();
    let mut data = read_data();

    // edit <new_name>
    // edit exercise x | x..y | [x, y, ..., z] (--date <date>) | -unsolved
    if args.get(0).is_some_and(|v| v != "chapter" && v != "exercise" && not_parseable(v))
        && args.get(1).is_none() {
        rename_subject(config.selected_subject, args[1].clone());
    }
    // edit chapter <new_name> ou edit chapter --label | -unlabeled <new_label> -sum_exercises | -no_sum
    else if args[0] == "chapter" && args.get(1).is_some() {
        let mut sum_exercises: bool;
        if data.get(config.selected_subject).is_some_and(|v| v.chapters.get(config.selected_chapter).is_some()) {
            sum_exercises = data[config.selected_subject].chapters[config.selected_chapter].sum_exercises;

            if args.contains(&String::from("-sum_exercises")) {
                sum_exercises = true;
            } else if args.contains(&String::from("-no_sum")) {
                sum_exercises = false;
            }
            data[config.selected_subject].chapters[config.selected_chapter].sum_exercises = sum_exercises;
            write_data(data);
        }
        // edit chapter --label <new_label>
        if args[1] == "--label" && args.get(2).is_some() {
            let label = args[2].clone();
            relabel(
                config.selected_subject,
                config.selected_chapter,
                Some(label),
            );
        }
        // edit chapter -unlabeled
        else if args[1] == "-unlabeled" {
            relabel(config.selected_subject, config.selected_chapter, None);
        } else {
            rename_chapter(
                config.selected_subject,
                config.selected_chapter,
                args[1].clone(),
            );
        }
    }
    // edit <subject_id> <new_name>
    else if parseable(&args[0]) && args.get(1).is_some_and(not_parseable) {
        rename_subject(args[0].parse().unwrap(), args[1].clone());
    }
    // edit <subject_id> <chapter_id> <new_name>
    else if parseable(&args[0]) && args.get(1).is_some_and(parseable)
        && args.get(2).is_some_and(not_parseable) {
        let subject_id = args[0].parse::<usize>().unwrap();
        let chapter_id = args[1].parse::<usize>().unwrap();

        if args[2] == "--label" && args.get(3).is_some_and(not_parseable) {
            relabel(subject_id, chapter_id, Some(args[3].clone()));
        } else if args[2] == "-no_label" {
            relabel(subject_id, chapter_id, None);
        } else {
            rename_chapter(subject_id, chapter_id, args[2].clone());
        }
    }
    // edit <subject_id> (x | x..y | [x y ... z] | [x, y, ..., z]) --label | -unlabeled <new_label> -sum_exercises
    // ou edit (x | x..y | [x y ... z] | [x, y, ..., z] --label
    else if parseable(&args[0])
        && args.get(1).is_some_and(|v| range_pattern().is_match(v) || list_pattern().is_match(v))
        && args.get(2).is_some_and(|v| v == "--label" || v == "-unlabeled") {
        // unlabel all chapters in range
        let exercises = get_exercises(args[1].clone());

        // TODO: change a field of every chapter in range. Needs a refactoring
        for exercise in exercises {}
    }
    // edit exercise x | x..y | [x, y, ..., z] (--label <label>) | -unlabeled
    else if args[0] == "exercise"
        && args.get(1).is_some_and(|v| range_pattern().is_match(v) || list_pattern().is_match(v))
        && args.get(2).is_some_and(|v| v == "--label" || v == "-unlabeled") {
        let exercises: Vec<usize> = get_exercises(args[1].clone());

        if args[2] == "-unlabeled" {
            relabel_exercise(config.selected_subject, config.selected_chapter, exercises, None);
        } else {
            if args.get(3).is_some() {
                relabel_exercise(config.selected_subject, config.selected_chapter, exercises, Some(args[3].clone()));
            }
        }
    } else if args[0] == "exercise"
        && args.get(1).is_some_and(|v| range_pattern().is_match(v) || list_pattern().is_match(v))
        && args.get(2).is_some_and(|v| v == "--date" || v == "-unsolved") {
        let exercises: Vec<usize> = get_exercises(args[1].clone());

        if args[2] == "-unsolved" {
            update_solve_date(config.selected_subject, config.selected_chapter, exercises, None);
        } else {
            if args.get(3).is_some() {
                update_solve_date(config.selected_subject, config.selected_chapter, exercises, Some(args[3].clone()));
            }
        }
    }
}
fn remove(args: Vec<String>) {
    let mut data = read_data();
    let config = read_config();
    let selected_subject = config.selected_subject;
    let selected_chapter = config.selected_chapter;

    // remove selected subject
    if args.get(0).is_none() {
        ask_about_removing_subject(selected_subject);
    }
    // remove a specific subject
    else if args.get(0).is_some_and(|v| v.parse::<i32>().is_ok_and(|v| v >= 0)) && args.get(1).is_none() {
        let subject_id = args[0].parse::<usize>().unwrap();

        if data.get(subject_id).is_none() {
            println!("The subject doesn't exist");
            return;
        }
        ask_about_removing_subject(subject_id);
    }
    // remove selected chapter
    else if args.get(0).is_some_and(|v| v == "chapter") {
        if data.get(selected_subject).is_none() {
            println!("There is no selected subject to remove its selected chapter");
            return;
        }
        if data[selected_subject].chapters.get(selected_chapter).is_none() {
            println!("There is no selected chapter to remove");
        }
        ask_about_removing_chapter(selected_subject, selected_chapter);
    }
    // remove a chapter of a specific subject
    else if args.get(0).is_some_and(|v| v.parse::<i32>().is_ok_and(|n| n >= 0))
        && args.get(1).is_some_and(|v| v.parse::<i32>().is_ok_and(|n| n >= 0)) {
        if data.get(selected_subject).is_none() {
            println!("Entered subject id doesn't exist");
            return;
        }
        if data[selected_subject]
            .chapters
            .get(selected_chapter)
            .is_none()
        {
            println!("Entered chapter id doesn't exist");
        }
        ask_about_removing_chapter(args[0].parse().unwrap(), args[1].parse().unwrap());
    } else if args.get(0).is_some_and(|v| v == "exercise")
        && args.get(1).is_some_and(|v| range_pattern().is_match(v) || list_pattern().is_match(v)) {
        let subject_id = config.selected_subject;
        if data.get(config.selected_subject).is_none() {
            println!("No subject selected");
        }

        let subject = &mut data[subject_id];
        if subject.chapters.get(config.selected_chapter).is_none() {
            println!("No chapter selected");
        }

        let chapter = &mut subject.chapters[config.selected_chapter];
        let exercises = get_exercises(args[1].clone());
        let mut exercise_list: Vec<Exercise> = vec![];
        let mut counting = 0;

        for (i, exercise) in chapter.exercises.iter_mut().enumerate() {
            if exercises.contains(&i) {
                counting += 1;
            } else {
                exercise_list.push(exercise.clone());
            }
        }
        println!("Removed {counting} exercises ");
        chapter.exercises = exercise_list;
        write_data(data);
    }
}
fn report(args: Vec<String>) {
    let data = read_data();
    let config = read_config();

    if data.get(config.selected_subject).is_none() {
        println!("There is no selected subject to report");
        return;
    }
    if data[config.selected_subject].chapters.get(config.selected_chapter).is_none() {
        println!("There is no selected chapter to report");
        return;
    }

    // reports selected chapter
    if args.get(0).is_none() {
        let subject = &data[config.selected_subject];
        let chapter = &subject.chapters[config.selected_chapter];

        if chapter.exercises.len() == 0 {
            println!("There are no exercises in chapter {}", chapter.name);
            return;
        }

        println!("-> {}", subject.name);
        println!("  -> {}", chapter.name);

        let mut counting = HashMap::<String, i32>::new();
        let mut not_labeled_counting = 0;
        let mut solved = 0;
        let mut unsolved = 0;

        for (i, exercise) in chapter.exercises.iter().enumerate() {
            if exercise.label.is_none() {
                not_labeled_counting += 1;

                if exercise.completion_date.is_none() {
                    unsolved += 1;
                    println!("     {i} - #{}: unsolved", not_labeled_counting);
                } else {
                    solved += 1;
                    println!("     {i} - #{}: solved at {}", not_labeled_counting, exercise.completion_date.clone().unwrap());
                }
            } else {
                let key = exercise.label.as_ref().unwrap();
                if counting.contains_key(key) {
                    counting.entry(key.clone()).and_modify(|v| *v += 1);
                } else {
                    counting.insert(key.clone(), 1);
                }
                let count = counting.get(key).unwrap();

                if exercise.completion_date.is_none() {
                    unsolved += 1;
                    println!("     {i} - #{} [{}]: unsolved", count + 1, exercise.label.clone().unwrap());
                } else {
                    solved += 1;
                    println!("     {i} - #{} [{}]: solved at {}", count + 1, exercise.completion_date.clone().unwrap(),
                             exercise.completion_date.clone().unwrap());
                }
            }
        }
        println!("\nUnsolved exercises: {:04} ({:05.2}%)", unsolved, 100f32 * unsolved as f32 / (solved + unsolved) as f32);
        println!("Solved exercises:   {:04} ({:05.2}%)", solved, solved as f32 * 100f32 / (solved + unsolved) as f32);
    }
    // report subject
    else if args.get(0).is_some_and(|v| v == "subject" || parseable(v)) {}
    // report chapter x | x..y | [x y ... z] | [x, y, ..., z]
    else if args.get(0).is_some_and(|v| v == "chapter")
        && args.get(1).is_some_and(|v| range_pattern().is_match(v) || list_pattern().is_match(v)) {}
    // report chapter --label <label>
    else if args.get(0).is_some_and(|v| v == "chapter")
        && args.get(1).is_some_and(|v| v == "--label")
        && args.get(2).is_some() {
        let mut chapters: Vec<&Chapter> = vec![];
        if data.get(config.selected_subject).is_none() {
            println!("There is no selected subject to report");
            return;
        }

        for chapter in data[config.selected_subject].chapters.iter() {
            if chapter.label.clone().is_some_and(|label| label == args[2]) {
                chapters.push(chapter);
            }
        }

        let mut counting = 0;
        println!("-> {}", data[config.selected_subject].name);

        for chapter in chapters {
            println!("  -> {}", chapter.name);
            for (i, exercise) in chapter.exercises.iter().enumerate() {
                println!("     {counting} - #{}", counting + 1);
                if chapter.sum_exercises {
                    counting += 1;
                }
            }
        }
    }
}
