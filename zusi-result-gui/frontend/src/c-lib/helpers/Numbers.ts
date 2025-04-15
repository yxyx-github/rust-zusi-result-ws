export default class Numbers {
    static format = {
        digits: (value: number, digits: number = 2) => {
            let strValue = value.toString()
            for (let i = 0; i < digits; i++) {
                strValue = '0' + strValue
            }
            return strValue.slice(digits * -1)
        },
    }

    static compare(a: number, b: number): number {
        return a - b
    }
}
