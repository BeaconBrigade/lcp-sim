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
	// the index of the correct answer
	correct: number;
	// the explanation for each option
	explanations: [string, string, string, string];
	// if the question has hardcoded actions
	isHardcoded: boolean;
	// the hardcoded new concentrations for each mc answer
	hardcoded: [number[], number[], number[], number[]];
};

// an interactive question with sliders
export type InteractiveQuestion = {
	// the type of question
	type: QuestionType.Interactive;
	// function to return if the question is correct, assumes the changes have been applied
	isRight: (change: number[]) => boolean;
	// message to display for a correct answer
	correctMsg: string;
	// message to display for a incorrect answer
	incorrectMsg: string;
};

// An adjustment to the system
export type Adjust = { Concentration: [string, number] };

function defaultActions(): [Adjust, Adjust, Adjust, Adjust] {
	return [
		{ Concentration: ['a', 1] },
		{ Concentration: ['b', 2] },
		{ Concentration: ['c', 3] },
		{ Concentration: ['d', 4] }
	];
}

export function findChange(
	changes: number[],
	defaults: number[],
	compounds: string[]
): [string, number] {
	for (let i = 0; i < changes.length; i++) {
		// found the change
		if (changes[i] !== defaults[i]) {
			return [compounds[i], changes[i]];
		}
	}

	// what if the user doesn't change anything before submitting?
	return ['', 0];
}

export function increaseAndCompound(
	changes: number[],
	defaults: number[],
	compounds: string[]
): [string, string] {
	for (let i = 0; i < changes.length; i++) {
		// found the change
		if (changes[i] != defaults[i]) {
			const increase = changes[i] > defaults[i] ? 'increase' : 'decrease';
			const compound = compounds[i];
			return [increase, compound];
		}
	}

	// what if the user doesn't change anything before submitting?
	return ['', ''];
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
			explanations: [
				'Increasing the concentration of NO increases concentration of the products causing a shift to the left.',
				'Increasing the concentration of SO3 adds to the products so to counteract this the equilibrium will shift left.',
				'Decreasing NO will decrease the concentration of the products so the equilibrium will shift to replace it, and shift right.',
				'Decreasing SO2 will decrease the concentration of the reactants so the equilibrium will shift left to replace it.'
			],
			isHardcoded: false,
			hardcoded: [[], [], [], []]
		}
	},
	{
		id: 2,
		equation: '2NH3(g) ↔ N2(g) + 3H2(g)',
		prompt: 'Modify the system to produce more ammonia',
		defaults: [2.0, 1.0, 1.5],
		q: {
			isRight: function (change: number[]): boolean {
				return change[0] > 2.0;
			},
			type: QuestionType.Interactive,
			correctMsg:
				'You caused ammonia to be increased by adding ammonia or adding nitrogen or hydrogen. Adding nitrogen or hydrogen caused an equilibrium shift to the left, producing ammonia.',
			incorrectMsg:
				'You caused ammonia to be decreased by removing ammonia or removing nitrogen or hydrogen. Removing nitrogen or hydrogen caused an equilibrium shift to the right, consuming ammonia.'
		}
	},
	{
		id: 3,
		equation: '2NH3(g) ↔ N2(g) + 3H2(g)',
		prompt: 'What would increase the concentration of NH3?',
		defaults: [2.0, 1.0, 1.5],
		q: {
			correct: 1,
			type: QuestionType.MultipleChoice,
			options: [
				'Increase the volume',
				'Increase the pressure',
				'Add a catalyst',
				'Increase the surface area'
			],
			actions: defaultActions(),
			explanations: [
				'The molar ratio is 2:4, so increasing volume will shift right, decreasing ammonia',
				'The molar ratio is 2:4 so increasing pressure will shift to the lesser side, the right, increasing ammonia',
				'Adding a catalyst will not cause a shift in equilibrium',
				'Increasing surface area will not cause a shift in equilibrium'
			],
			isHardcoded: true,
			hardcoded: [
				[1.9, 1.05, 1.65],
				[2.2, 0.9, 1.3],
				[2.0, 1.0, 1.5],
				[2.0, 1.0, 1.5]
			]
		}
	},
	{
		id: 4,
		equation: '2NH3(g) ↔ N2(g) + 3H2(g)',
		prompt: 'What would occur if you increased volume?',
		defaults: [2.0, 1.0, 1.5],
		q: {
			correct: 1,
			type: QuestionType.MultipleChoice,
			options: ['Increase NH3', 'Increase N2', 'Decrease N2', 'Decrease H2'],
			actions: defaultActions(),
			explanations: [
				'The molar ratio is 2:4 so the equilibrium will shift to the right, decreasing NH3',
				'The equilibrium will shift right, increasing N2',
				'The equilibrium will shift right, increasing N2',
				'The equilibrium will shift right, increasing H2'
			],
			isHardcoded: true,
			hardcoded: [
				[1.9, 1.05, 1.65],
				[1.9, 1.05, 1.65],
				[1.9, 1.05, 1.65],
				[1.9, 1.05, 1.65]
			]
		}
	},
	{
		id: 5,
		equation: 'CH3COOH + H2O ↔ CH3COO + H3O',
		prompt: 'Temperature is increased at t<sub>1</sub>, what is this reaction?',
		defaults: [2.0],
		q: {
			type: QuestionType.MultipleChoice,
			correct: 1,
			options: ['Endothermic', 'Exothermic', '', ''],
			actions: defaultActions(),
			explanations: [
				'An endothermic reaction consumes energy. At t<sub>1</sub> temperature is increased and H<sub>3</sub>O<sup>+</sup> is decreased indicating a shift to the left. Since energy is a reactant in endothermic reactions, this reaction is not endothermic.',
				'An exothermic reaction produces energy. At t<sub>1</sub> temperature is increased and H<sub>3</sub>O<sup>+</sup> is decreased indicating a shift to the left. This means energy was a product which makes the reaction exothermic.',
				'',
				''
			],
			isHardcoded: false,
			hardcoded: [[], [], [], []]
		}
	}
];
