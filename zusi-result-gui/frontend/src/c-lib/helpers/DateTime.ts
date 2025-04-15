import Numbers from '@/c-lib/helpers/Numbers'

export default class DateTime {
    static format = {
        dateInputValue: (date: Date) =>
            `${date.getFullYear()}-${Numbers.format.digits(date.getMonth() + 1)}-${Numbers.format.digits(date.getDate())}`
        ,
        timeInputValue: (date: Date) =>
            `${Numbers.format.digits(date.getHours())}:${Numbers.format.digits(date.getMinutes())}`
        ,
        dateTimeLocaleInputValue: (date: Date) =>
            `${this.format.dateInputValue(date)}T${this.format.timeInputValue(date)}`
        ,
    }

    static compare(a: DateTime, b: DateTime): number {
        // @ts-ignore
        return a - b
    }
}
