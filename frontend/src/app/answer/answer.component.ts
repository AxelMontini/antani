import { Component, OnInit, Input, OnChanges, SimpleChanges } from '@angular/core';
import { Message } from '../request/request.component';
@Component({
  selector: 'app-answer',
  templateUrl: './answer.component.html',
  styleUrls: ['./answer.component.scss']
})

export class AnswerComponent implements OnInit, OnChanges {
  


  constructor() { }

  ngOnInit(): void {
  }

  /* Request handling */
  @Input() message: Message = {
    showLoading: false,
    from: '',
    to:'',
    datetime: ''
  }
  /* */
  showMeteo :boolean = false;
  showTrains :boolean = false;

  ngOnChanges(changes: SimpleChanges) {
    if(this.message.showLoading) {

    }
  }
}
