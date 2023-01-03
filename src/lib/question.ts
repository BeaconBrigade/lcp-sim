// the generic question type
export type Question = {
    // number out of 10
    id: number;
    // the equation of the problem
    equation: string;
    // the prompt
    prompt: string;
    // the original concentrations for resetting
    defaults: number[];
    // the answer type of the question... might be better to convert to
    // generics over an interface...
    q: MultipleChoiceQuestion | InteractiveQuestion;
};

export enum QuestionType {
    MultipleChoice,
    Interactive
}

// a multiple choice question
export type MultipleChoiceQuestion = {
    // the type of question
    type: QuestionType.MultipleChoice;
    // the choices
    options: [string, string, string, string];
    // the currently selected answer
    // maybe drop:
    selected: number | undefined;
    // function to return if the question is correct
    isRight: (guess: number) => boolean,
};

// an interactive question with sliders
export type InteractiveQuestion = {
    // the type of question
    type: QuestionType.Interactive;
    // the `idx` and `value` to change
    change: number[];
    // function to return if the question is correct
    isRight: (guess: [number, number]) => boolean,
};

function defaultQuestion(id: number): Question {
    return {
        id: id,
        equation: 'Bread + PeanutButter ↔ PbSandwich',
        prompt: 'Which option is the coolest',
        defaults: [2.0, 1.0, 1.0],
        q: {
            isRight: (guess) => guess == 2,
            type: QuestionType.MultipleChoice,
            options: ['Option 1', 'Option 2', 'Option 3', 'Option 4'],
            selected: undefined
        }
    };
}

export const questions: Question[] = [
    {
        id: 1,
        equation: 'SO2 + NO2 ↔ NO + SO3',
        prompt: 'Which change will cause an equilibrium shift to the right?',
        defaults: [2.0, 2.0, 2.0, 2.0],
        q: {
            isRight: (guess) => guess === 2,
            type: QuestionType.MultipleChoice,
            options: [
                'Increase concentration of NO',
                'Increase concentration of SO3',
                'Decrease concentration of NO',
                'Decrease concentration of SO2'
            ],
            selected: undefined
        }
    },
    {
        id: 2,
        equation: '2NH3(g) ↔ N2(g) + 3H2(g)',
        prompt: 'Modify the system to produce more ammonia',
        defaults: [2.0, 1.0, 1.5],
        q: {
            // eslint-disable-next-line @typescript-eslint/no-unused-vars
            isRight: ([idx, val]) => { console.error("interactive isRight not implemented"); return true; },
            type: QuestionType.Interactive,
            change: [2.0, 1.0, 1.5]
        }
    },
    defaultQuestion(3),
    defaultQuestion(4),
    defaultQuestion(5),
    defaultQuestion(6),
    defaultQuestion(7),
    defaultQuestion(8),
    defaultQuestion(9)
];
