import {Pipe, PipeTransform} from "@angular/core";

@Pipe({
    name: "solAmount",
})
export class SolAmountPipe implements PipeTransform {

    transform(amount: number): string {
        return `${amount / 1000000000.0} SOL`;
    }

}
