import process from "node:process"
import fs from "node:fs"
import path from 'node:path';

let day = process.argv[2];
if(day.length == 1) day = "0" + day

const part = process.argv[3] === "2" ? 2 : 1;

class AdvError extends Error{
    constructor(message: string){
        super(message)
    }
}

(async () => {
    try {
        const main = await import(`./days/${day}`)
        const input = fs.readFileSync(path.join(__dirname, "../Inputs/01.txt")).toString()
        if(!input) throw new AdvError(`No input specified for day ${day}`)
        if(typeof main?.default != "function") throw new AdvError(`Day ${day} does not export a function`)
        console.log(main?.default(input, part))
    } catch (error) {
        if(!(error instanceof Error)) throw error

        if(error instanceof AdvError){
            console.log(error.message)
            return
        }

        if(error.message.includes("Cannot find module")){
            console.log(`Day ${day} isn't set up yet`)
        }
    }
})()