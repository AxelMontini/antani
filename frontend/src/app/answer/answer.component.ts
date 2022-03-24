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
    depDatetime: '',
    retDatetime: ''
  }
  /* */
  showMeteo :boolean = false;
  showTrains :boolean = false;
  meteo :any;

  ngOnChanges(changes: SimpleChanges) {
    console.log(this.message);
    if(this.message.showLoading) {
      fetch(`/api/weather?date=${this.message.retDatetime}&station=${this.message.to}`).then(r => {
        r.json();
        this.message.showLoading = false;
      }).then(r => console.log(r));
      fetch(`/api/connections?from=${this.message.from}&to=${this.message.to}&datetime=${this.message.depDatetime}&is_arrival_time=false`).then(r => r.json()).then(r => console.log(r));
      fetch(`/api/connections?from=${this.message.from}&to=${this.message.to}&datetime=${this.message.retDatetime}&is_arrival_time=true`).then(r => r.json()).then(r => console.log(r));
    }
  }
}
