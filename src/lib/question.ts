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
	// the change to the system for each option
	actions: [Adjust, Adjust, Adjust, Adjust];
	// the currently selected answer
	selected: number | undefined;
	// the index of the correct answer
	correct: number;
	// the explanation for each option
	explanations: [string, string, string, string];
};

// an interactive question with sliders
export type InteractiveQuestion = {
	// the type of question
	type: QuestionType.Interactive;
	// the `idx` and `value` to change
	change: number[];
	// function to return if the question is correct
	isRight: (guess: [number, number]) => boolean;
};

// An adjustment to the system
export type Adjust =
	| { Concentration: [string, number] }
	| { Temperature: number }
	| { Volume: number };

function defaultQuestion(id: number): Question {
	return {
		id: id,
		equation: 'Bread + PeanutButter ↔ PbSandwich',
		prompt: 'Which option is the coolest',
		defaults: [2.0, 1.0, 1.0],
		q: {
			correct: 2,
			type: QuestionType.MultipleChoice,
			options: ['Option 1', 'Option 2', 'Option 3', 'Option 4'],
			actions: [
				{ Concentration: ['Bread', 1.0] },
				{ Concentration: ['PeanutButter', 1.0] },
				{ Concentration: ['PbSandwich', 2.0] },
				{ Concentration: ['Bread', 0.5] }
			],
			selected: undefined,
			explanations: [
				'This is a very funny option',
				'This is a very dumb option',
				'This is a very very right option, that tells you a lot about your personality, because to use it you must be cool',
				'This is another wrong option but with lots of text again. Simply for the reason that stuff needs to be tested is why this is here'
			]
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
			correct: 2,
			type: QuestionType.MultipleChoice,
			options: [
				'Increase concentration of NO',
				'Increase concentration of SO3',
				'Decrease concentration of NO',
				'Decrease concentration of SO2'
			],
			actions: [
				{ Concentration: ['NO', 2.5] },
				{ Concentration: ['SO3', 2.5] },
				{ Concentration: ['NO', 1.5] },
				{ Concentration: ['SO2', 1.5] }
			],
			selected: undefined,
			explanations: [
				'Increasing the concentration of NO increases concentration of the products causing a shift to the left.',
				'Increasing the concentration of SO3 adds to the products so to counteract this the equilibrium will shift left.',
				'Decreasing NO will decrease the concentration of the products so the equilibrium will shift to replace it, and shift right.',
				'Decreasing SO2 will decrease the concentration of the reactants so the equilibrium will shift left to replace it.'
			]
		}
	},
	{
		id: 2,
		equation: '2NH3(g) ↔ N2(g) + 3H2(g)',
		prompt: 'Modify the system to produce more ammonia',
		defaults: [2.0, 1.0, 1.5],
		q: {
			// eslint-disable-next-line @typescript-eslint/no-unused-vars
			isRight: ([idx, val]) => {
				console.error('interactive isRight not implemented');
				return true;
			},
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
