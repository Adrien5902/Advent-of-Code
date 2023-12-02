import fs from "node:fs"
import path from 'node:path';

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

export default function main(part: 1 | 2){
    const input = fs.readFileSync(path.join(__dirname, "input.txt")).toString()
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
                while(numberLetters[j] && line[i+j] == numberLetters[j]){
                    if(j == numberLetters.length){
                        numbers.push(n2)
                        break
                    }
                    j++
                }
            }
        }
        return Number(numbers[0] + numbers[numbers.length - 1])
    })/* .reduce((a, b) => a+b, 0) */
}