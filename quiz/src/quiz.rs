pub struct Question {
    pub title: String,
    pub answer: bool,
    pub user_answer: Option<bool>,
}

pub struct Quiz {
    pub questions: Vec<Question>,
    pub current_index: usize,
}

impl Question {
    pub fn is_answered(&self) -> bool {
        self.user_answer.is_some()
    }

    pub fn is_correct(&self) -> bool {
        self.user_answer == Some(self.answer)
    }

    pub fn sample() -> Vec<Self> {
        vec![
            Question {
                title: "Is the sky blue?".to_string(),
                answer: true,
                user_answer: None,
            },
            Question {
                title: "Is the grass green?".to_string(),
                answer: true,
                user_answer: None,
            },
            Question {
                title: "Is the sun yellow?".to_string(),
                answer: false,
                user_answer: None,
            },
        ]
    }
}

impl Quiz {
    pub fn answer(&mut self, answer: bool) -> Option<bool> {
        if self.current_index >= self.questions.len() {
            return None;
        }

        let curr_question = &mut self.questions[self.current_index];
        curr_question.user_answer = Some(answer);
        Some(curr_question.is_correct())
    }

    pub fn current_question(&self) -> &Question {
        self.questions.get(self.current_index).unwrap()
    }

    pub fn next_question(&mut self) -> Option<&Question> {
        if self.current_index == (self.questions.len() - 1) {
            return None;
        }

        self.current_index += 1;
        Some(self.current_question())
    }

    pub fn new(questions: Vec<Question>) -> Self {
        Self {
            questions,
            current_index: 0,
        }
    }

    pub fn prev_question(&mut self) -> Option<&Question> {
        if self.current_index == 0 {
            return None;
        }

        self.current_index -= 1;
        Some(self.current_question())
    }

    pub fn score(&self) -> usize {
        let mut total_score: usize = 0;

        for question in self.questions.iter() {
            if question.is_correct() {
                total_score += 1;
            }
        }

        total_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_question_is_answered() {
        let q = Question {
            title: String::from("Is this a question?"),
            answer: true,
            user_answer: Some(true),
        };

        assert_eq!(q.is_answered(), true);
    }

    #[test]
    pub fn test_question_is_correct() {
        let q = Question {
            title: String::from("Is this a question?"),
            answer: true,
            user_answer: Some(true),
        };

        assert_eq!(q.is_correct(), true);
    }
}
