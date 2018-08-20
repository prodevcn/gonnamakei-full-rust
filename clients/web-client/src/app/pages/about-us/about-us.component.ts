import {Component, OnInit} from "@angular/core";
import {TeamMember} from "../../models";
import {TeamMembers} from "../../constants";

@Component({
    selector: "app-about-us",
    templateUrl: "./about-us.component.html",
    styleUrls: ["./about-us.component.scss"],
})
export class AboutUsComponent implements OnInit {

    selectedMember: TeamMember;
    teamMembers = [] as Array<TeamMember>;

    constructor() {
    }

    ngOnInit(): void {
        this.teamMembers = TeamMembers;
    }

}
