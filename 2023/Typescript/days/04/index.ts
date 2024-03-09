const numbersRegex = /^\d+/

function get_winning_numbers(card: number[][]) {
    return card[0]
        .filter(winning => card[1].find(gotten => winning == gotten))
}

export default function main(input: string, part: 1 | 2) {
    const cards = input.split("\r\n")
    const data = cards.map((card, i) => {
        const card_id = i + 1
        return card
            .replace("Card", "")
            .replace(`${card_id}: `, "")
            .trim()
            .split("|")
            .map(part =>
                part
                    .split(" ")
                    .map(v => v.trim())
                    .filter(v => v)
                    .map(n => parseInt(n))
            )
    })

    if (part == 1) {
        const part1_res = data.map(card => {
            const winning_numbers = get_winning_numbers(card)
            if (winning_numbers.length) return 2 ** (winning_numbers.length - 1)
            else return 0
        })
            .reduce((a, b) => a + b, 0)
        return part1_res
    } else {
        const counts = new Array(data.length).fill(1)
        data.forEach((card, i) => {
            const count = counts[i]
            const winning_numbers = get_winning_numbers(card)
            for (let j = 0; j < winning_numbers.length; j++) {
                counts[i + j + 1] += count
            }
        })
        return counts.reduce((a, b) => a + b)
    }
}