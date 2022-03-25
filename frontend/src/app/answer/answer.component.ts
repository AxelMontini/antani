import {
  Component,
  OnInit,
  Input,
  OnChanges,
  SimpleChanges,
} from '@angular/core';
import { Message } from '../request/request.component';
import { ConnectionResponse } from '../trains/train_interface';

@Component({
  selector: 'app-answer',
  templateUrl: './answer.component.html',
  styleUrls: ['./answer.component.scss'],
})
export class AnswerComponent implements OnInit, OnChanges {
  constructor() { }

  ngOnInit(): void { }

  /* Request handling */
  @Input() message: Message | undefined;
  /* */
  showMeteo: boolean = false;
  showTrains: boolean = false;
  isHolyday: boolean = false;
  connections: ConnectionResponse | undefined;
  meteo: any;


  ngOnChanges(changes: SimpleChanges) {
    if (this.message?.showLoading) {
      const d = new Date(this.message?.lastYearDate);
      // Date YYYY-mm-dd
      const date = d.getFullYear() + "-" + (d.getMonth() + 1) + "-" + d.getDate();

      fetch(`/api/holidays?name=${this.message?.to}&date=${date}`).then(r => r.json()).then(r => { if (r.isHoliday) { this.isHolyday = true; } });
      fetch(
        `/api/connections?from=${this.message?.from}&to=${this.message?.to}&datetime=${this.message?.dateTimeDep}&is_arrival_time=false`
      )
        .then((r) => r.json())
        .then((r) => this.connections = r.connections)
        .then((_) => ((this.message) ? this.message.showLoading = false : null))
        .then((_) => (this.showTrains = true))
        .then(_ =>this.showMeteo = true);
    }
  }
}
