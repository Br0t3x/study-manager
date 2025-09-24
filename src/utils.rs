pub mod utils {
    pub enum CommandEnum {
        New,
    }
    pub struct Command {
        pub command: CommandEnum,
        pub command_options: CommandOptions,
    }
    pub struct CommandOptions {
        pub subject_list: Option<Vec<String>>,
        pub module_list: Option<Vec<String>>,
        pub label_list: Option<Vec<String>>,
        pub module_list_file: Option<String>,
    }
    impl CommandOptions {
        pub fn new() -> Self {
            Self {
                label_list: None,
                module_list: None,
                subject_list: None,
                module_list_file: None,
            }
        }
        pub fn set_subject_list(&mut self, subject_list: Vec<String>) -> &mut Self {
            self.subject_list = Some(subject_list);
            self
        }
        pub fn set_module_list_file(&mut self, module_list_file: String) -> &mut Self {
            self.module_list_file = Some(module_list_file);
            self
        }
        pub fn set_module_list(&mut self, module_list: Vec<String>) -> &mut Self {
            self.module_list = Some(module_list);
            self
        }
        pub fn set_label_list(&mut self, label_list: Vec<String>) -> &mut Self {
            self.label_list = Some(label_list);
            self
        }
    }
    impl Command {
        fn new(command: CommandEnum, command_options: CommandOptions) -> Self {
            Self {
                command,
                command_options,
            }
        }
        pub fn execute() {}
    }
    pub mod word_processing {
        pub fn get_terms(line: &str) -> Vec<String> {
            if line.is_empty() {
                return Vec::<String>::new();
            }

            let mut list = Vec::<String>::new();
            let mut is_it_a_multiple_word_term = false;

            for word in line.split_whitespace() {
                let pure_word = word.replace("\"", "");

                if !is_it_a_multiple_word_term {
                    list.push(pure_word);
                    if word.starts_with("\"") && !word.ends_with("\"") {
                        is_it_a_multiple_word_term = true;
                    }
                }
                else {
                    let term = list.last_mut().unwrap();
                    term.push(' ');
                    term.push_str(&pure_word);

                    if word.ends_with("\"") {
                        is_it_a_multiple_word_term = false;
                    }
                }
            }
            list
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            use std::io::stdin;

            #[test]
            fn test_get_terms() {
                assert_eq!(
                    get_terms("new Matemática do EM"),
                    vec![String::from("new"), String::from("Matemática do EM")]
                );
                assert_eq!(
                    get_terms("new Computaço"),
                    vec![String::from("new"), String::from("Computaço")]
                );
                assert_eq!(
                    get_terms("new \"Computaço\""),
                    vec![String::from("new"), String::from("Computaço")]
                );
                assert_eq!(
                    get_terms("new "),
                    vec![String::from("new"), String::from("Matemática do EM")]
                );
                assert_eq!(
                    terms,
                    vec![String::from("new"), String::from("Matemática do EM")]
                );
                assert_eq!(
                    terms,
                    vec![String::from("new"), String::from("Matemática do EM")]
                );
                assert_eq!(
                    terms,
                    vec![String::from("new"), String::from("Matemática do EM")]
                );
                assert_eq!(
                    terms,
                    vec![String::from("new"), String::from("Matemática do EM")]
                );
                assert_eq!(
                    terms,
                    vec![String::from("new"), String::from("Matemática do EM")]
                );
                assert_eq!(
                    terms,
                    vec![String::from("new"), String::from("Matemática do EM")]
                );
            }
        }
    }
    pub fn parse() -> Command {
        Command::new(CommandEnum::New, CommandOptions::new())
    }
}
