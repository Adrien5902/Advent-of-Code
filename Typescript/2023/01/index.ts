const numberMap = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
]

export default function main(input: string, part: 1 | 2){
    const lines = input.split("\r\n")
    return lines.map(line => {
        const numbers: string[] = []
        for(let i = 0; i < line.length; i++){
            const n = line[i]
            if(Number(n)) {numbers.push(n)}

            if(part != 2) continue

            for(const n2 in numberMap){
                const numberLetters = numberMap[n2]
                let j = 0
                while (i + j < line.length && line[i+j] == numberLetters[j]) {
                    if (j == numberLetters.length - 1) {
                        numbers.push((Number(n2) + 1).toString());
                        break;
                    }
                    j += 1;
                }
            }
        }
        return Number(numbers[0] + numbers[numbers.length - 1])
    }).reduce((a, b) => a+b, 0)
}