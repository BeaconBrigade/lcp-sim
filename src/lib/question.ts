// the generic question type
export type Question = {
    // number out of 10
    id: number,
    // the equation of the problem
    equation: string,
    // the prompt
    prompt: string,
    // the answer type of the question... might be better to convert to
    // generics over an interface...
    q: MultipleChoiceQuestion | InteractiveQuestion,
}

export enum QuestionType {
    MultipleChoice,
    Interactive,
}

// a multiple choice question
export type MultipleChoiceQuestion = {
    // the type of question
    type: QuestionType.MultipleChoice,
    // the choices
    options: [string, string, string, string],
    // the currently selected answer
    // maybe drop:
    selected: number | undefined,
}

// an interactive question with sliders
export type InteractiveQuestion = {
    // the type of question
    type: QuestionType.Interactive,
    // the original concentrations for resetting
    defaults: number[],
    // the `idx` and `value` to change
    change: number[],
}

function defaultQuestion(id: number): Question {
    return ({
        id: id,
        equation: 'Bread + PeanutButter ↔ PbSandwich',
        prompt: 'Which option is the coolest',
        q: {
            type: QuestionType.MultipleChoice,
            options: [
                'Option 1',
                'Option 2',
                'Option 3',
                'Option 4'
            ],
            selected: undefined,
        },

    })
}

export const questions: Question[] = [
    {
        id: 1,
        equation: 'SO2 + NO2 ↔ NO + SO3',
        prompt: 'Which change will cause an equilibrium shift to the right?',
        q: {
            type: QuestionType.MultipleChoice,
            options: [
                'Increase concentration of NO',
                'Increase concentration of SO3',
                'Decrease concentration of NO',
                'Decrease concentration of SO2'
            ],
            selected: undefined,
        },
    },
    {
        id: 2,
        equation: '2NH3(g) ↔ N2(g) + 3H2(g)',
        prompt: 'Modify the system to produce more ammonia',
        q: {
            type: QuestionType.Interactive,
            defaults: [2.0, 1.0, 1.5],
            change: [2.0, 1.0, 1.5],
        }
    },
    defaultQuestion(3),
    defaultQuestion(4),
    defaultQuestion(5),
    defaultQuestion(6),
    defaultQuestion(7),
    defaultQuestion(8),
    defaultQuestion(9),
];
