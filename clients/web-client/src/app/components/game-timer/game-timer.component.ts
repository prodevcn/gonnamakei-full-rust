import {Component, ElementRef, EventEmitter, Input, OnDestroy, OnInit, Output, ViewChild} from "@angular/core";
import {TimeLeftPipe} from "../../pipes/time-left/time-left.pipe";

import {BehaviorSubject} from "rxjs";
import {distinctUntilChanged, filter} from "rxjs/operators";

@Component({
    selector: "app-game-timer",
    templateUrl: "./game-timer.component.html",
    styleUrls: ["./game-timer.component.scss"],
})
export class GameTimerComponent implements OnInit, OnDestroy {

    @Input() startDateTime;
    @Input() totalTime: number;
    @Output() onTimeout = new EventEmitter();

    timerDoneEvents = new BehaviorSubject<boolean>(false);

    @ViewChild("circleRing", {static: true}) circleRing: ElementRef<SVGCircleElement>;
    circleCircumference = 0;

    timerId = null;

    constructor(private timeLeftPipe: TimeLeftPipe) {
    }

    ngOnInit(): void {
        this.initCircle();
        this.setProgress(0);
        this.initCountDown();

        this.timerDoneEvents
            .pipe(distinctUntilChanged())
            .pipe(filter(done => done))
            .subscribe(() => this.onTimeout.next());
    }

    ngOnDestroy() {
        clearInterval(this.timerId);
    }

    initCountDown() {
        const leftTime = this.timeLeftPipe.transform(this.startDateTime, this.totalTime);
        if (leftTime <= 0) {
            return;
        }

        this.timerId = setInterval(() => {
            const leftTime = this.timeLeftPipe.transform(this.startDateTime, this.totalTime);

            const elaspedTime = this.totalTime - leftTime;

            this.timerDoneEvents.next(!leftTime);

            this.setProgress(100 * elaspedTime / this.totalTime);
        }, 200);
    }

    initCircle() {
        const circle = this.circleRing.nativeElement;
        const radius = circle.r.baseVal.value;
        const circumference = radius * 2 * Math.PI;
        this.circleCircumference = circumference;

        circle.style.strokeDasharray = `${circumference} ${circumference}`;
        circle.style.strokeDashoffset = `${circumference}`;
    }

    setProgress(percent) {

        const circle = this.circleRing.nativeElement;

        const offset = this.circleCircumference - percent / 100 * this.circleCircumference;
        circle.style.strokeDashoffset = `${offset}`;
    }

}
