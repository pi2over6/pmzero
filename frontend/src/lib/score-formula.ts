/**
 * Evaluates the small arithmetic language accepted by score fields.
 * Only numbers, +, -, *, /, parentheses, decimal points, and whitespace are valid.
 */
export function evaluateScoreFormula(input: string): number | null {
	const source = input.trim();
	if (!source || !/^[\d.\s()+\-*/]+$/.test(source)) return null;

	let position = 0;

	function skipWhitespace() {
		while (/\s/.test(source[position] ?? '')) position += 1;
	}

	function parseNumber(): number | null {
		skipWhitespace();
		const match = /^(?:\d+(?:\.\d*)?|\.\d+)/.exec(source.slice(position));
		if (!match) return null;
		position += match[0].length;
		const value = Number(match[0]);
		return Number.isFinite(value) ? value : null;
	}

	function parseFactor(): number | null {
		skipWhitespace();
		const character = source[position];
		if (character === '+' || character === '-') {
			position += 1;
			const value = parseFactor();
			return value === null ? null : character === '-' ? -value : value;
		}
		if (character === '(') {
			position += 1;
			const value = parseExpression();
			skipWhitespace();
			if (value === null || source[position] !== ')') return null;
			position += 1;
			return value;
		}
		return parseNumber();
	}

	function parseTerm(): number | null {
		let value = parseFactor();
		while (value !== null) {
			skipWhitespace();
			const operator = source[position];
			if (operator !== '*' && operator !== '/') break;
			position += 1;
			const right = parseFactor();
			if (right === null || (operator === '/' && right === 0)) return null;
			value = operator === '*' ? value * right : value / right;
			if (!Number.isFinite(value)) return null;
		}
		return value;
	}

	function parseExpression(): number | null {
		let value = parseTerm();
		while (value !== null) {
			skipWhitespace();
			const operator = source[position];
			if (operator !== '+' && operator !== '-') break;
			position += 1;
			const right = parseTerm();
			if (right === null) return null;
			value = operator === '+' ? value + right : value - right;
			if (!Number.isFinite(value)) return null;
		}
		return value;
	}

	const value = parseExpression();
	skipWhitespace();
	return value !== null && position === source.length && Number.isFinite(value) ? value : null;
}

export function formatScoreFormula(input: string, multiplier = 1): string {
	const value = evaluateScoreFormula(input);
	if (value === null) return input;

	// Scores may be entered either in thousands (25) or full points (25000).
	// Decimal entries and values not divisible by 100 use thousands.
	const normalized = multiplier > 1 && (input.includes('.') || value % 100 !== 0)
		? value * multiplier
		: value;
	return String(normalized);
}
