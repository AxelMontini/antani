import { Component, OnInit, Output, EventEmitter } from '@angular/core';
import { RequestComponent } from "../request/request.component";
import { AnswerComponent } from "../answer/answer.component";

@Component({
  selector: 'app-request-answer-component',
  templateUrl: './request-answer-component.component.html',
  styleUrls: ['./request-answer-component.component.scss']
})
export class RequestAnswerComponentComponent implements OnInit {

  constructor() { }

  message: boolean = false;

  receiveMessage($event :any) {
    this.message = $event
  }

  ngOnInit(): void {
  }

}